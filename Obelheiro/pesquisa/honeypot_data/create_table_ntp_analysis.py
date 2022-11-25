# from email import header
import sqlite3
# import ntplib

#t[each] = (ip_network(each).supernet(new_prefix=24) )
# ntplib.

ntp_con = sqlite3.connect('./db/database-2022-05-11/dnstor_statistics_ntp.sqlite', timeout=10)
ntp_ntp = ntp_con.cursor()

mix_connection = sqlite3.connect('./db/database-2022-05-11/mix_protocol.sqlite', timeout=10)
mix_cursor = mix_connection.cursor()

monlist_byte0 = b'\x17'
monlist_byte3 = b'\x2a'


payload_types = {'monlist': 0}

def get_ntp_type(ntp_payload, raw_payload):

  if len(ntp_payload) <= 3:
    add_to_payload_types(ntp_payload)
    return "outros"

  # checks for the following
  # b'\x17' and b'\x2a'
  # not b'x17' and b'x2a'
  # not b'\\x17' and b'\\x2a'
  if (monlist_byte0[0] == ntp_payload[0] and monlist_byte3[0] == ntp_payload[3]):
    payload_types['monlist'] += 1
    return "monlist"

  add_to_payload_types(raw_payload)
  return "outros"

def add_to_payload_types(ntp_payload: bytes):
  if payload_types.get(ntp_payload) == None:
    payload_types[ntp_payload] = 0
  payload_types[ntp_payload] += 1


mix_cursor.execute("DROP TABLE IF EXISTS NTP_PAYLOAD_TYPES;")
mix_cursor.execute("""
CREATE TABLE NTP_PAYLOAD_TYPES (
	id INTEGER NOT NULL,
  payload TEXT NOT NULL,
  quantity INTEGER NOT NULL
);
""")

mix_cursor.execute("DROP TABLE IF EXISTS NTP_ANALYSIS;")
mix_cursor.execute("""
CREATE TABLE NTP_ANALYSIS (
	id INTEGER NOT NULL,
  ip TEXT NOT NULL,
	requests_per_attack INTEGER NOT NULL,
  tempo_inicio TEXT NOT NULL,
  tempo_final TEXT NOT NULL,
	payload_id INTEGER NOT NULL,
  payload TEXT NOT NULL,
  raw_payload TEXT NOT NULL,
	year INTEGER NOT NULL,
	period INTEGER NOT NULL,
	ntp_type TEXT NOT NULL
);
""")
mix_connection.commit()

ntpId = int(1)
ntpAnalysis = []

for row in ntp_ntp.execute("""
SELECT *, CAST(CAST(year AS text) || CAST(period AS text) as integer) as year_period
  FROM (SELECT ip, count, tempoInicio, tempoFinal, NTP_MEMORY_DICT.payloadID, payload, strftime("%Y", tempoFinal) as year, ((strftime("%m", tempoFinal) - 1) / 3) + 1 AS period
          FROM NTP_MEMORY_DICT
          JOIN NTP_PAYLOAD_DICT
            ON NTP_MEMORY_DICT.payloadID = NTP_PAYLOAD_DICT.payloadID)
 WHERE year_period >= 20184 AND
       year_period <= 20221 AND
       count >= 5;
"""):
  ip = row[0]
  count = int(row[1])
  tempoInicio = row[2]
  tempoFinal = row[3]
  payloadID = int(row[4])
  strQuotedPacket = row[5]
  year = int(row[6])
  period = int(row[7])

  print("strQuotedPacket", strQuotedPacket)
  # removes B' at the beginning, remove ' from the end and cast it to bytes
  # bytePayloadUnscaped = bytes(strQuotedPacket, encoding="utf-8")
  bytePayloadUnscaped = bytes(strQuotedPacket[2:-1], encoding="utf-8")
  # decode and encode again to change "\\" to "\"
  bytePayload = bytePayloadUnscaped.decode("unicode_escape").encode("raw_unicode_escape")
  # bytePayload = bytePayloadUnscaped.decode("unicode_escape").encode("raw_unicode_escape")
  # print("\n bytePayload", bytePayload, "len(bytePayload) ", len(bytePayload))
  ntp_type = get_ntp_type(bytePayload, strQuotedPacket)
  # ntp_type = get_ntp_type(strQuotedPacket)
  print(ntp_type)
  ntpAnalysis.append((ntpId, ip, count, tempoInicio, tempoFinal, payloadID, bytePayload, strQuotedPacket, year, period, ntp_type))
  print(ntpAnalysis[len(ntpAnalysis) - 1])
  ntpId += 1

mix_cursor.executemany('INSERT INTO NTP_ANALYSIS VALUES (?,?,?,?,?,?,?,?,?,?,?)', ntpAnalysis)
mix_connection.commit()

id = 0
for payload in payload_types:
  id += 1
  print(payload_types[payload], payload)
  ntp_ntp.execute('INSERT INTO NTP_PAYLOAD_TYPES VALUES (?,?,?)', [id, payload, payload_types[payload]])
mix_connection.commit()

ntp_con.close()
mix_connection.close()

# print(payload_types)
print(payload_types.values())
# print(payload_types.keys())