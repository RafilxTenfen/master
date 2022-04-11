SELECT *
  FROM DNS_MEMORY_DICT
  JOIN DNS_PAYLOAD_DICT
    ON DNS_MEMORY_DICT.payloadID = DNS_PAYLOAD_DICT.payloadID
 LIMIT 5;w

SELECT strftime("%Y", tempoFinal) as year, ((strftime("%m", tempoFinal) - 1) / 3) + 1 AS period, DNS_MEMORY_DICT.*, DNS_PAYLOAD_DICT.payload
  FROM DNS_MEMORY_DICT
  JOIN DNS_PAYLOAD_DICT
    ON DNS_MEMORY_DICT.payloadID == DNS_PAYLOAD_DICT.payloadID
 LIMIT 5;

SELECT DATETIME("2020-10-29 16:16:19.158460");

SELECT strftime("%Y", DATETIME("2020-10-29 16:16:19.158460"));

SELECT strftime("%Y", "2020-10-29 16:16:19.158460");

SELECT strftime("%m", "2020-10-29 16:16:19.158460");

SELECT ((12 - 1) / 3) + 1 AS period;

SELECT


  SELECT count(*) as qnt_repeat_query_id, *
    FROM DNS_ANALYSIS
    JOIN DNS_ANALYSIS_QUESTION
      ON DNS_ANALYSIS.id = DNS_ANALYSIS_QUESTION.dns_analysis_id
GROUP BY query_id
  HAVING qnt_repeat_query_id > 1
ORDER BY count desc
   LIMIT 10

DROP TABLE IF EXISTS GROUP_BY_QUERYID;
CREATE TEMPORARY TABLE GROUP_BY_QUERYID AS
	  SELECT count(*) as qnt_repeat_query_id, requests_per_attack, year, period, query_id, qname, qtype
		FROM DNS_ANALYSIS
		JOIN DNS_ANALYSIS_QUESTION
		  ON DNS_ANALYSIS.id = DNS_ANALYSIS_QUESTION.dns_analysis_id
	GROUP BY query_id
	  HAVING qnt_repeat_query_id > 1
	ORDER BY qnt_repeat_query_id desc
;

DROP TABLE IF EXISTS GROUP_BY_QUERYID_QNAME;
CREATE TEMPORARY TABLE GROUP_BY_QUERYID_QNAME AS
	  SELECT count(*) as qnt_repeat_query_id, requests_per_attack, year, period, query_id, qname, qtype
		FROM DNS_ANALYSIS
		JOIN DNS_ANALYSIS_QUESTION
		  ON DNS_ANALYSIS.id = DNS_ANALYSIS_QUESTION.dns_analysis_id
	GROUP BY query_id, qname
	  HAVING qnt_repeat_query_id > 1
	ORDER BY qnt_repeat_query_id desc
;


  SELECT qnt_repeat_query_id / requests_per_attack AS same_attack_ratio, *
    FROM GROUP_BY_QUERYID
   WHERE same_attack_ratio > 1
ORDER BY same_attack_ratio DESC

  SELECT qnt_repeat_query_id / requests_per_attack AS same_attack_ratio, *
    FROM GROUP_BY_QUERYID_QNAME
   WHERE same_attack_ratio > 1
ORDER BY same_attack_ratio DESC


DROP TABLE IF EXISTS TB_DATE_RANGE;
CREATE TEMPORARY TABLE TB_DATE_RANGE AS
SELECT *, CAST(tempo_inicio as datetime), CAST(tempo_final as datetime), CAST(CAST(year AS text) || CAST(period AS text) as integer) as year_period
  FROM DNS_ANALYSIS
  JOIN DNS_ANALYSIS_QUESTION
    ON DNS_ANALYSIS.id = DNS_ANALYSIS_QUESTION.dns_analysis_id
 WHERE QTYPE != 0
   AND query_id = 17767
   AND qname in ("isc.org.", "sl.")
   AND qtype = "ANY"
;

DROP TABLE IF EXISTS TB_DATE_OVERLAP;
CREATE TEMPORARY TABLE TB_DATE_OVERLAP AS
  SELECT tbisc.id, tbisc.query_id, tbisc.tempo_inicio, tbisc.tempo_final, tbisc.requests_per_attack, tbisc.qname,
         tbsl.qname, tbsl.requests_per_attack, tbsl.tempo_inicio, tbsl.tempo_final, tbsl.id, tbsl.query_id
    FROM TB_DATE_RANGE AS tbisc
    JOIN TB_DATE_RANGE AS tbsl
      ON (
  	     tbisc.id <> tbsl.id
  	 AND tbisc.qname <> tbsl.qname
       AND tbisc.tempo_inicio >= tbsl.tempo_inicio
  	 AND tbisc.tempo_final <= tbsl.tempo_final
  	)
;


SELECT id, tempo_inicio, tempo_final, query_id, qtype, qname, requests_per_attack,
      (SELECT COUNT(*)
         FROM TB_DATE_RANGE AS tbsl
	    WHERE tbisc.id <> tbsl.id
		  AND tbisc.qname <> tbsl.qname
	      AND NOT (tbisc.tempo_inicio > tbsl.tempo_final or tbisc.tempo_final < tbsl.tempo_inicio)) as amount_overlap
  FROM TB_DATE_RANGE AS tbisc
 WHERE amount_overlap > 0



  SELECT id, tempo_inicio, tempo_final, query_id, qtype, qname, requests_per_attack,
         (SELECT COUNT(*)
            FROM TB_DATE_RANGE AS tbsl
           WHERE tbisc.id <> tbsl.id
             AND tbisc.qname <> tbsl.qname
             AND NOT (tbisc.tempo_inicio > tbsl.tempo_final or tbisc.tempo_final < tbsl.tempo_inicio)) as amount_overlap
    FROM TB_DATE_RANGE AS tbisc
   WHERE amount_overlap > 0
ORDER BY amount_overlap DESC


DROP TABLE IF EXISTS TB_DATE_OVERLAP;
CREATE TEMPORARY TABLE TB_DATE_OVERLAP AS
  SELECT id, tempo_inicio, tempo_final, query_id, qtype, qname, requests_per_attack,
         (SELECT COUNT(*)
            FROM TB_DATE_RANGE AS tbsl
           WHERE tbisc.id <> tbsl.id
             AND tbisc.qname <> tbsl.qname
             AND NOT (tbisc.tempo_inicio > tbsl.tempo_final or tbisc.tempo_final < tbsl.tempo_inicio)) as amount_overlap
    FROM TB_DATE_RANGE AS tbisc

GROUP BY tbisc.tempo_inicio, tbisc.tempo_final
  2020-11-28 18:32:27.035397	2020-11-28 18:32:27.035400
  2020-11-28 18:29:42.529789	2020-11-28 18:34:47.455305


DROP TABLE IF EXISTS TB_DATE_OVERLAP;
CREATE TABLE TB_DATE_OVERLAP AS
  SELECT id, tempo_inicio, tempo_final, query_id, qtype, qname, requests_per_attack,
         (SELECT COUNT(*)
            FROM TB_DATE_RANGE AS tbsl
           WHERE tbisc.id <> tbsl.id
             AND tbisc.qname <> tbsl.qname
             AND (
					( tbsl.tempo_inicio BETWEEN tbisc.tempo_inicio AND tbisc.tempo_final )
				 OR ( tbsl.tempo_final BETWEEN tbisc.tempo_inicio AND tbisc.tempo_final )
				 )
		   ) as amount_overlap
    FROM TB_DATE_RANGE AS tbisc



SELECT *
  FROM TB_DATE_RANGE
 WHERE QTYPE != 0
   AND query_id = 17767
   AND qname = "isc.org."
   AND qtype = "ANY"
   AND tempo_inicio >= datetime("2021-01-31 16:50:23.832700")
   AND tempo_final <= datetime("2021-01-31 22:34:50.086930")




DROP TABLE IF EXISTS TB_DATE_RANGE;
CREATE TEMPORARY TABLE TB_DATE_RANGE AS
SELECT *, CAST(tempo_inicio as datetime), CAST(tempo_final as datetime), CAST(CAST(year AS text) || CAST(period AS text) as integer) as year_period
  FROM DNS_ANALYSIS
  JOIN DNS_ANALYSIS_QUESTION
    ON DNS_ANALYSIS.id = DNS_ANALYSIS_QUESTION.dns_analysis_id
 WHERE QTYPE != 0
;

DROP TABLE IF EXISTS TB_DATE_OVERLAP;
CREATE TABLE TB_DATE_OVERLAP AS
  SELECT id, tempo_inicio, tempo_final, query_id, qtype, qname, requests_per_attack,
         (SELECT COUNT(*)
            FROM TB_DATE_RANGE AS tbsl
           WHERE tbisc.id <> tbsl.id
             AND tbisc.qname <> tbsl.qname
             AND (
					( tbsl.tempo_inicio BETWEEN tbisc.tempo_inicio AND tbisc.tempo_final )
				 OR ( tbsl.tempo_final BETWEEN tbisc.tempo_inicio AND tbisc.tempo_final )
				 )
		   ) as amount_overlap
    FROM TB_DATE_RANGE AS tbisc



DROP TABLE IF EXISTS TB_DATE_OVERLAP_QUERYID;
CREATE TABLE TB_DATE_OVERLAP_QUERYID AS
  SELECT id, tempo_inicio, tempo_final, query_id, qtype, qname, requests_per_attack, ip,
         (SELECT COUNT(*)
            FROM TB_DATE_RANGE AS tbsl
           WHERE tbisc.id <> tbsl.id
             AND tbisc.qname <> tbsl.qname
			 AND tbisc.query_id = tbsl.query_id
             AND (
					( tbsl.tempo_inicio BETWEEN tbisc.tempo_inicio AND tbisc.tempo_final )
				 OR ( tbsl.tempo_final BETWEEN tbisc.tempo_inicio AND tbisc.tempo_final )
				 )
		   ) as amount_overlap
    FROM TB_DATE_RANGE AS tbisc

----------------------------
-- Execution finished without errors.
-- Result: query executed successfully. Took 37106070ms
-- At line 2:
-- CREATE TABLE TB_DATE_OVERLAP AS
--   SELECT id, tempo_inicio, tempo_final, query_id, qtype, qname, requests_per_attack, ip,
--          (SELECT COUNT(*)
--             FROM TB_DATE_RANGE AS tbsl
--            WHERE tbisc.id <> tbsl.id
--              AND tbisc.qname <> tbsl.qname
--              AND (
-- 					( tbsl.tempo_inicio BETWEEN tbisc.tempo_inicio AND tbisc.tempo_final )
-- 				 OR ( tbsl.tempo_final BETWEEN tbisc.tempo_inicio AND tbisc.tempo_final )
-- 				 )
-- 		   ) as amount_overlap
--     FROM TB_DATE_RANGE AS tbisc