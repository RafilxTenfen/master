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
