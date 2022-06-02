# from email import header
import sqlite3
import ntplib

#t[each] = (ip_network(each).supernet(new_prefix=24) )
# ntplib.

con = sqlite3.connect('./db/database-2022-05-11/dnstor_statistics_ntp.sqlite', timeout=10)
cur = con.cursor()


def get_ntp_type(ntp_payload):
  ntp_data_pattern = b'\x17\x00\x03\x2a\x00\x00\x00\x00'
  if ntp_data_pattern == ntp_payload:
    return "Monlist"
  return "Other"


cur.execute("DROP TABLE IF EXISTS NTP_ANALYSIS;")
cur.execute("""
CREATE TABLE NTP_ANALYSIS (
	id INTEGER NOT NULL,
  ip TEXT NOT NULL,
	requests_per_attack INTEGER NOT NULL,
  tempo_inicio TEXT NOT NULL,
  tempo_final TEXT NOT NULL,
	payload_id INTEGER NOT NULL,
  payload TEXT NOT NULL,
	year INTEGER NOT NULL,
	period INTEGER NOT NULL,
	ntp_type TEXT NOT NULL
);
""")
con.commit()

ntpId = int(1)
ntpAnalysis = []

for row in cur.execute("""
SELECT ip, count, tempoInicio, tempoFinal, NTP_MEMORY_DICT.payloadID, payload, strftime("%Y", tempoFinal) as year, ((strftime("%m", tempoFinal) - 1) / 3) + 1 AS period
  FROM NTP_MEMORY_DICT
  JOIN NTP_PAYLOAD_DICT
    ON NTP_MEMORY_DICT.payloadID = NTP_PAYLOAD_DICT.payloadID;
"""):
  ip = row[0]
  count = int(row[1])
  tempoInicio = row[2]
  tempoFinal = row[3]
  payloadID = int(row[4])
  strQuotedPacket = row[5]
  year = int(row[6])
  period = int(row[7])

  # removes B' at the beginning, remove ' from the end and cast it to bytes
  bytePayloadUnscaped = bytes(strQuotedPacket[2:-1], encoding="utf-8")
  # decode and encode again to change "\\" to "\"
  bytePayload = bytePayloadUnscaped.decode("unicode_escape").encode("raw_unicode_escape")
  # print("\n bytePayload", bytePayload, "len(bytePayload) ", len(bytePayload))
  ntp_type = get_ntp_type(bytePayload)
  print(ntp_type)
  ntpAnalysis.append((ntpId, ip, count, tempoInicio, tempoFinal, payloadID, bytePayload, year, period, ntp_type))
  print(ntpAnalysis[len(ntpAnalysis) - 1])
  ntpId += 1

cur.executemany('INSERT INTO NTP_ANALYSIS VALUES (?,?,?,?,?,?,?,?,?,?)', ntpAnalysis)
con.commit()

con.close()

