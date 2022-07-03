import sqlite3

mix_connection = sqlite3.connect('./db/database-2022-05-11/mix_protocol.sqlite', timeout=10)
mix_cursor = mix_connection.cursor()
encoding = 'cp437'

mix_cursor.execute("DROP TABLE IF EXISTS MEMCACHED_PAYLOAD_TYPES;")
mix_cursor.execute("""
CREATE TABLE MEMCACHED_PAYLOAD_TYPES (
	id INTEGER NOT NULL,
  payload TEXT NOT NULL,
  quantity INTEGER NOT NULL
);
""")

mix_cursor.execute("DROP TABLE IF EXISTS MEMCACHED_ANALYSIS;")
mix_cursor.execute("""
CREATE TABLE MEMCACHED_ANALYSIS (
	id INTEGER NOT NULL,
  ip TEXT NOT NULL,
	requests_per_attack INTEGER NOT NULL,
  tempo_inicio TEXT NOT NULL,
  tempo_final TEXT NOT NULL,
  payload TEXT NOT NULL,
  payload_decoded TEXT NOT NULL,
  get_payload_value TEXT NOT NULL,
  memcached_request_type TEXT NOT NULL
);
""")
mix_connection.commit()

protocol_id = int(1)

protocol_mix = []
memcached_connection = sqlite3.connect('./db/database-2022-05-11/dnstor_statistics_memcached.sqlite', timeout=10)
memcached_cursor = memcached_connection.cursor()


byte_stats = b"stats"
byte_set = b"set"
byte_add = b"add"
byte_get = b"get"
byte_cas = b"cas"
byte_replace = b"replace"
byte_append = b"append"
byte_prepend = b"prepend"
byte_flush_all = b"flush_all"

payload_types = {
  'stats': 0,
  'set': 0,
  'add': 0,
  'get': 0,
  'cas': 0,
  'replace': 0,
  'append': 0,
  'prepend': 0,
  'flush_all': 0,
  'outros': 0,
}

def contain_bytes(memcached_payload, byte_keyword):
  memcached_payload_len = len(memcached_payload)
  byte_keyword_len = len(byte_keyword)

  if memcached_payload_len < byte_keyword_len:
    return False

  currentIndex = 0
  while currentIndex < memcached_payload_len:

    for keyword_idx in range(0, byte_keyword_len-1):
      if memcached_payload[currentIndex] != byte_keyword[keyword_idx]:
        currentIndex += 1
        break

      end_memcached_payload = currentIndex+(byte_keyword_len)
      if end_memcached_payload >= memcached_payload_len:
        return False

      if memcached_payload[currentIndex: end_memcached_payload] == byte_keyword:
        return True

  return False

def get_bytes_after(memcached_payload, byte_keyword):
  memcached_payload_len = len(memcached_payload)
  byte_keyword_len = len(byte_keyword)

  if memcached_payload_len < byte_keyword_len:
    return "-"

  currentIndex = 0
  while currentIndex < memcached_payload_len:

    for keyword_idx in range(0, byte_keyword_len-1):
      if memcached_payload[currentIndex] != byte_keyword[keyword_idx]:
        currentIndex += 1
        break

      end_memcached_payload = currentIndex+(byte_keyword_len)
      if end_memcached_payload >= memcached_payload_len:
        return "-"

      if memcached_payload[currentIndex: end_memcached_payload] == byte_keyword:
        return memcached_payload[end_memcached_payload: (memcached_payload_len-1)]

  return "-"

def get_memcached_request_type(memcached_payload):

  print("get_memcached_request_type", memcached_payload)

  if contain_bytes(memcached_payload, byte_stats):
    payload_types['stats'] += 1
    return "Stats"
  if contain_bytes(memcached_payload, byte_set):
    payload_types['set'] += 1
    return "Set"
  if contain_bytes(memcached_payload, byte_get):
    payload_types['get'] += 1
    return "Get"
  if contain_bytes(memcached_payload, byte_add):
    payload_types['add'] += 1
    return "Add"
  if contain_bytes(memcached_payload, byte_flush_all):
    payload_types['flush_all'] += 1
    return "FlushAll"
  if contain_bytes(memcached_payload, byte_cas):
    payload_types['cas'] += 1
    return "Cas"
  if contain_bytes(memcached_payload, byte_replace):
    payload_types['replace'] += 1
    return "Replace"
  if contain_bytes(memcached_payload, byte_append):
    payload_types['append'] += 1
    return "Append"
  if contain_bytes(memcached_payload, byte_prepend):
    payload_types['prepend'] += 1
    return "Prepend"

  payload_types['outros'] += 1
  # add_to_payload_types(memcached_payload)
  return "Outros"

def add_to_payload_types(memcached_payload: bytes):
  if payload_types.get(memcached_payload) == None:
    payload_types[memcached_payload] = 0
  payload_types[memcached_payload] += 1


for memcached_row in memcached_cursor.execute("""
SELECT ip, count, tempoInicio, tempoFinal, payload
  FROM MEMCACHED_MEMORY_DICT
  JOIN MEMCACHED_PAYLOAD_DICT
    ON MEMCACHED_MEMORY_DICT.payloadID = MEMCACHED_PAYLOAD_DICT.payloadID
 WHERE count > 5
"""):
  ip = memcached_row[0]
  requests_per_attack = int(memcached_row[1])
  tempoInicio = memcached_row[2]
  tempoFinal = memcached_row[3]
  payload = memcached_row[4]

  print("payload", payload)
  bytePayloadUnscaped = bytes(payload[2:-1], encoding="utf-8")
  bytePayload = bytePayloadUnscaped.decode("unicode_escape").encode("raw_unicode_escape")
  decoded_payload = bytePayload.decode(encoding)

  memcached_request_type = get_memcached_request_type(bytePayload)
  get_payload_value = "-"
  if memcached_request_type == "Get":
    get_payload_value = get_bytes_after(bytePayload, byte_get)
  # memcached_request_type = get_memcached_request_type(strQuotedPacket)
  print(memcached_request_type)

  protocol_mix.append((protocol_id, ip, requests_per_attack, tempoInicio, tempoFinal, bytePayload, decoded_payload, get_payload_value, memcached_request_type))
  protocol_id += 1

memcached_connection.close()

mix_cursor.executemany('INSERT INTO MEMCACHED_ANALYSIS VALUES (?,?,?,?,?,?,?,?,?)', protocol_mix)
mix_connection.commit()
print('memcached finish: ', len(protocol_mix))
print('payload_types: ', len(payload_types), payload_types)


id = 0
for payload in payload_types:
  id += 1
  print(payload_types[payload], payload)
  mix_cursor.execute('INSERT INTO MEMCACHED_PAYLOAD_TYPES VALUES (?,?,?)', [id, payload, payload_types[payload]])
mix_connection.commit()

mix_connection.close()
