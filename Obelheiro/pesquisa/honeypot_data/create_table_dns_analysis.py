import sqlite3
import binascii
import dnslib
# from github_com.paulc import dnslib
# from './src/dnslib' import DNSRecord

con = sqlite3.connect('dnstor_statistics_dns.sqlite', timeout=10)
cur = con.cursor()

cur.execute("DROP TABLE IF EXISTS DNS_ANALYSIS;")
cur.execute("""
CREATE TABLE DNS_ANALYSIS (
	year INTEGER NOT NULL,
	period INTEGER NOT NULL,
	qname TEXT,
	qtype TEXT,
	count INTEGER NOT NULL
);
""")
con.commit()

dnsAnalysis = []

for row in cur.execute("""
  SELECT strftime("%Y", tempoFinal) as year, ((strftime("%m", tempoFinal) - 1) / 3) + 1 AS period, DNS_MEMORY_DICT.ip, DNS_MEMORY_DICT.count, CAST(DNS_PAYLOAD_DICT.payload as TEXT) as payload
    FROM DNS_MEMORY_DICT
    JOIN DNS_PAYLOAD_DICT
      ON DNS_MEMORY_DICT.payloadID == DNS_PAYLOAD_DICT.payloadID;
"""):
  year = int(row[0])
  period = int(row[1])
  count = int(row[3])
  strQuotedPacket = row[4]

  # removes B' at the beginning, remove ' from the end and cast it to bytes
  bytePayloadUnscaped = bytes(strQuotedPacket[2:-1], encoding="utf-8")
  # decode and encode again to change "\\" to "\"
  bytePayload = bytePayloadUnscaped.decode("unicode_escape").encode("raw_unicode_escape")
  dnsRecordPayload = dnslib.DNSRecord.parse(bytePayload)

  for dnsQuestion in dnsRecordPayload.questions:
    # print("\n", dnsQuestion.qname, type(dnsQuestion.qname.idna()))
    dnsAnalysis.append((year, period, dnsQuestion.qname.idna(), dnslib.QTYPE.get(dnsQuestion.qtype), count))

cur.executemany('INSERT INTO DNS_ANALYSIS VALUES (?,?,?,?,?)', dnsAnalysis)
con.commit()
con.close()

