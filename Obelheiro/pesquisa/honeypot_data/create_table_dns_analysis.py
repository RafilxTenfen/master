from email import header
import sqlite3
import dnslib

#t[each] = (ip_network(each).supernet(new_prefix=24) )


con = sqlite3.connect('dnstor_statistics_dns.sqlite', timeout=10)
cur = con.cursor()

cur.execute("DROP TABLE IF EXISTS DNS_ANALYSIS;")
cur.execute("""
CREATE TABLE DNS_ANALYSIS (
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

cur.execute("DROP TABLE IF EXISTS DNS_ANALYSIS_QUESTION;")
cur.execute("""
CREATE TABLE DNS_ANALYSIS_QUESTION (
	dns_analysis_id INTEGER NOT NULL,
	qname VARCHAR(255),
	qtype VARCHAR(255)
);
""")
con.commit()

dnsId = int(1)
dnsAnalysis = []
dnsAnalysisQuestions = []
unableToParse = 0

for row in cur.execute("""
  SELECT strftime("%Y", tempoFinal) as year, ((strftime("%m", tempoFinal) - 1) / 3) + 1 AS period, DNS_MEMORY_DICT.ip, DNS_MEMORY_DICT.count, CAST(DNS_PAYLOAD_DICT.payload as TEXT) as payload, tempoInicio, tempoFinal, ip
    FROM DNS_MEMORY_DICT
    JOIN DNS_PAYLOAD_DICT
      ON DNS_MEMORY_DICT.payloadID == DNS_PAYLOAD_DICT.payloadID;
"""):
  year = int(row[0])
  period = int(row[1])
  count = int(row[3])
  strQuotedPacket = row[4]
  tempoInicio = row[5]
  tempoFinal = row[6]
  ip = row[7]

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
    dnsAnalysisQuestions.append((dnsId, dnsQuestion.qname.idna(), dnslib.QTYPE.get(dnsQuestion.qtype)))
    # print("\n", "dnsQuestion", dnsQuestion)

  dnsId += 1

if unableToParse:
  print("\nUnable to parse DNSRecord:", unableToParse, "payloads")

cur.executemany('INSERT INTO DNS_ANALYSIS VALUES (?,?,?,?,?,?,?,?)', dnsAnalysis)
con.commit()

cur.executemany('INSERT INTO DNS_ANALYSIS_QUESTION VALUES (?,?,?)', dnsAnalysisQuestions)
con.commit()
con.close()

