from email import header
import sqlite3
import dnslib

#t[each] = (ip_network(each).supernet(new_prefix=24) )


con = sqlite3.connect('./db/database-2022-05-11/dnstor_statistics_cldap.sqlite', timeout=10)
cur = con.cursor()

cur.execute("DROP TABLE IF EXISTS CLDAP_ANALYSIS;")
cur.execute("""
CREATE TABLE CLDAP_ANALYSIS (
	id INTEGER NOT NULL,
	year INTEGER NOT NULL,
	period INTEGER NOT NULL,
	requests_per_attack INTEGER NOT NULL,
	query_id INTEGER NOT NULL,
  tempo_inicio TEXT NOT NULL,
  tempo_final TEXT NOT NULL,
  ip TEXT NOT NULL
);
""")
con.commit()

dnsId = int(1)
dnsAnalysis = []
dnsAnalysisQuestions = []
unableToParse = 0

for row in cur.execute("""
SELECT ip AS vitima_ip, count AS requests_per_attack, CAST(CAST(year AS text) || CAST(period AS text) as integer) as year_period, payload
  FROM (
      SELECT *,  strftime(\"%Y\", tempoInicio) as year, ((strftime(\"%m\", tempoFinal) - 1) / 3) + 1 AS period
        FROM CLDAP_MEMORY_DICT
        JOIN CLDAP_PAYLOAD_DICT
          ON CLDAP_MEMORY_DICT.payloadID == CLDAP_PAYLOAD_DICT.payloadID;
  )
"""):
  vitima_ip = int(row[0])
  requests_per_attack = int(row[1])
  year_period = int(row[2])
  strQuotedPacket = row[3]

  # removes B' at the beginning, remove ' from the end and cast it to bytes
  bytePayloadUnscaped = bytes(strQuotedPacket[2:-1], encoding="utf-8")
  # decode and encode again to change "\\" to "\"
  bytePayload = bytePayloadUnscaped.decode("unicode_escape").encode("raw_unicode_escape")
  # print("\n bytePayload", bytePayload, "len(bytePayload) ", len(bytePayload))

  try:
    dnsRecordPayload = dnslib.DNSRecord.parse(bytePayload)
    # print("\ndnsRecordPayload:", dnsRecordPayload)
    dnsHeader = dnsRecordPayload.header
    if dnsHeader:
      queryId = dnsHeader.id
      dnsAnalysis.append((dnsId, year, period, count, queryId, tempoInicio, tempoFinal, ip))
    else:
      dnsAnalysis.append((dnsId, year, period, count, -1, tempoInicio, tempoFinal, ip))
  except Exception as e:
    print("\ncould not parse", bytePayload)
    print("\nError msg", e)
    unableToParse += 1
    continue

  for dnsQuestion in dnsRecordPayload.questions:
    # print("\n", dnsQuestion.qname, type(dnsQuestion.qname.idna()))
    dnsAnalysisQuestions.append((dnsId, dnsQuestion.qname.idna().lower(), dnslib.QTYPE.get(dnsQuestion.qtype)))
    # print("\n", "dnsQuestion", dnsQuestion)

  dnsId += 1

if unableToParse:
  print("\nUnable to parse DNSRecord:", unableToParse, "payloads")

cur.executemany('INSERT INTO DNS_ANALYSIS VALUES (?,?,?,?,?,?,?,?)', dnsAnalysis)
con.commit()

cur.executemany('INSERT INTO DNS_ANALYSIS_QUESTION VALUES (?,?,?)', dnsAnalysisQuestions)
con.commit()
con.close()

