import sqlite3

mix_connection = sqlite3.connect('../db/database-2022-05-11/mix_protocol.sqlite', timeout=10)
mix_cursor = mix_connection.cursor()
# MIX PROTOCOL PRECISA FILTRAR >=5 E YEAR_PERIOD ENTRE 20184 E 20221

mix_cursor.execute("DROP TABLE IF EXISTS MIX_PROTOCOL;")
mix_cursor.execute("""
CREATE TABLE MIX_PROTOCOL (
	id INTEGER NOT NULL,
  attack_protocol TEXT NOT NULL,
	requests_per_attack INTEGER NOT NULL,
  tempo_inicio TEXT NOT NULL,
  tempo_final TEXT NOT NULL,
  vitima_ip TEXT NOT NULL,
  raw_payload TEXT DEFAULT "-"
);
""")
mix_connection.commit()

protocol_id = int(1)
protocol_mix = []

ntp_connection = sqlite3.connect('../db/database-2022-05-11/dnstor_statistics_ntp.sqlite', timeout=10)
ntp_cursor = ntp_connection.cursor()

for ntp_row in ntp_cursor.execute("""
  SELECT count, tempoInicio, tempoFinal, ip, payload
     FROM NTP_MEMORY_DICT
LEFT JOIN NTP_PAYLOAD_DICT
       ON NTP_MEMORY_DICT.payloadID = NTP_PAYLOAD_DICT.payloadID;
"""):
  count = int(ntp_row[0])
  tempoInicio = ntp_row[1]
  tempoFinal = ntp_row[2]
  ipVitima = ntp_row[3]
  payload = ntp_row[4]

  protocol_mix.append((protocol_id, "NTP", count, tempoInicio, tempoFinal, ipVitima, payload))
  protocol_id += 1

ntp_connection.close()
print('ntp finish: ', len(protocol_mix))

mix_cursor.executemany('INSERT INTO MIX_PROTOCOL VALUES (?,?,?,?,?,?,?)', protocol_mix)
mix_connection.commit()


protocol_mix = []
dns_connection = sqlite3.connect('../db/database-2022-05-11/dnstor_statistics_dns.sqlite', timeout=10)
dns_cursor = dns_connection.cursor()

for dns_row in dns_cursor.execute("""
   SELECT count, tempoInicio, tempoFinal, ip, payload
     FROM DNS_MEMORY_DICT
LEFT JOIN DNS_PAYLOAD_DICT
       ON DNS_MEMORY_DICT.payloadID = DNS_PAYLOAD_DICT.payloadID;
"""):
  count = int(dns_row[0])
  tempoInicio = dns_row[1]
  tempoFinal = dns_row[2]
  ipVitima = dns_row[3]
  payload = dns_row[4]

  protocol_mix.append((protocol_id, "DNS", count, tempoInicio, tempoFinal, ipVitima, payload))
  protocol_id += 1

dns_connection.close()

mix_cursor.executemany('INSERT INTO MIX_PROTOCOL VALUES (?,?,?,?,?,?,?)', protocol_mix)
mix_connection.commit()
print('dns finish: ', len(protocol_mix))


protocol_mix = []
chargen_connection = sqlite3.connect('../db/database-2022-05-11/dnstor_statistics_chargen.sqlite', timeout=10)
chargen_cursor = chargen_connection.cursor()

for chargen_row in chargen_cursor.execute("""
   SELECT count, tempoInicio, tempoFinal, ip, payload
     FROM CHARGEN_MEMORY_DICT
LEFT JOIN CHARGEN_PAYLOAD_DICT
       ON CHARGEN_MEMORY_DICT.payloadID = CHARGEN_PAYLOAD_DICT.payloadID;
"""):
  count = int(chargen_row[0])
  tempoInicio = chargen_row[1]
  tempoFinal = chargen_row[2]
  ipVitima = chargen_row[3]
  payload = chargen_row[4]

  protocol_mix.append((protocol_id, "CHARGEN", count, tempoInicio, tempoFinal, ipVitima, payload))
  protocol_id += 1

chargen_connection.close()

mix_cursor.executemany('INSERT INTO MIX_PROTOCOL VALUES (?,?,?,?,?,?,?)', protocol_mix)
mix_connection.commit()
print('chargen finish: ', len(protocol_mix))



protocol_mix = []
cldap_connection = sqlite3.connect('../db/database-2022-05-11/dnstor_statistics_cldap.sqlite', timeout=10)
cldap_cursor = cldap_connection.cursor()

for cldap_row in cldap_cursor.execute("""
   SELECT count, tempoInicio, tempoFinal, ip, payload
     FROM CLDAP_MEMORY_DICT
LEFT JOIN CLDAP_PAYLOAD_DICT
       ON CLDAP_MEMORY_DICT.payloadID = CLDAP_PAYLOAD_DICT.payloadID;
"""):
  count = int(cldap_row[0])
  tempoInicio = cldap_row[1]
  tempoFinal = cldap_row[2]
  ipVitima = cldap_row[3]
  payload = cldap_row[4]

  protocol_mix.append((protocol_id, "CLDAP", count, tempoInicio, tempoFinal, ipVitima, payload))
  protocol_id += 1

cldap_connection.close()

mix_cursor.executemany('INSERT INTO MIX_PROTOCOL VALUES (?,?,?,?,?,?,?)', protocol_mix)
mix_connection.commit()
print('cldap finish: ', len(protocol_mix))


protocol_mix = []
coap_connection = sqlite3.connect('../db/database-2022-05-11/dnstor_statistics_coap.sqlite', timeout=10)
coap_cursor = coap_connection.cursor()

for coap_row in coap_cursor.execute("""
   SELECT count, tempoInicio, tempoFinal, ip, payload
     FROM COAP_MEMORY_DICT
LEFT JOIN COAP_PAYLOAD_DICT
       ON COAP_MEMORY_DICT.payloadID = COAP_PAYLOAD_DICT.payloadID;
"""):
  count = int(coap_row[0])
  tempoInicio = coap_row[1]
  tempoFinal = coap_row[2]
  ipVitima = coap_row[3]
  payload = coap_row[4]

  protocol_mix.append((protocol_id, "COAP", count, tempoInicio, tempoFinal, ipVitima, payload))
  protocol_id += 1

coap_connection.close()

mix_cursor.executemany('INSERT INTO MIX_PROTOCOL VALUES (?,?,?,?,?,?,?)', protocol_mix)
mix_connection.commit()
print('coap finish: ', len(protocol_mix))

protocol_mix = []
memcached_connection = sqlite3.connect('../db/database-2022-05-11/dnstor_statistics_memcached.sqlite', timeout=10)
memcached_cursor = memcached_connection.cursor()

for memcached_row in memcached_cursor.execute("""
   SELECT count, tempoInicio, tempoFinal, ip, payload
     FROM MEMCACHED_MEMORY_DICT
LEFT JOIN MEMCACHED_PAYLOAD_DICT
       ON MEMCACHED_MEMORY_DICT.payloadID = MEMCACHED_PAYLOAD_DICT.payloadID;
"""):
  count = int(memcached_row[0])
  tempoInicio = memcached_row[1]
  tempoFinal = memcached_row[2]
  ipVitima = memcached_row[3]
  payload = memcached_row[4]

  protocol_mix.append((protocol_id, "MEMCACHED", count, tempoInicio, tempoFinal, ipVitima, payload))
  protocol_id += 1

memcached_connection.close()

mix_cursor.executemany('INSERT INTO MIX_PROTOCOL VALUES (?,?,?,?,?,?,?)', protocol_mix)
mix_connection.commit()
print('memcached finish: ', len(protocol_mix))



protocol_mix = []
qotd_connection = sqlite3.connect('../db/database-2022-05-11/dnstor_statistics_qotd.sqlite', timeout=10)
qotd_cursor = qotd_connection.cursor()

for qotd_row in qotd_cursor.execute("""
   SELECT count, tempoInicio, tempoFinal, ip, payload
     FROM QOTD_MEMORY_DICT
LEFT JOIN QOTD_PAYLOAD_DICT
       ON QOTD_MEMORY_DICT.payloadID = QOTD_PAYLOAD_DICT.payloadID;
"""):
  count = int(qotd_row[0])
  tempoInicio = qotd_row[1]
  tempoFinal = qotd_row[2]
  ipVitima = qotd_row[3]
  payload = qotd_row[4]

  protocol_mix.append((protocol_id, "QOTD", count, tempoInicio, tempoFinal, ipVitima, payload))
  protocol_id += 1

qotd_connection.close()

mix_cursor.executemany('INSERT INTO MIX_PROTOCOL VALUES (?,?,?,?,?,?,?)', protocol_mix)
mix_connection.commit()
print('qotd finish: ', len(protocol_mix))



protocol_mix = []
ssdp_connection = sqlite3.connect('../db/database-2022-05-11/dnstor_statistics_ssdp.sqlite', timeout=10)
ssdp_cursor = ssdp_connection.cursor()

for ssdp_row in ssdp_cursor.execute("""
   SELECT count, tempoInicio, tempoFinal, ip, payload
     FROM SSDP_MEMORY_DICT
LEFT JOIN SSDP_PAYLOAD_DICT
       ON SSDP_MEMORY_DICT.payloadID = SSDP_PAYLOAD_DICT.payloadID;
"""):
  count = int(ssdp_row[0])
  tempoInicio = ssdp_row[1]
  tempoFinal = ssdp_row[2]
  ipVitima = ssdp_row[3]
  payload = ssdp_row[4]

  protocol_mix.append((protocol_id, "SSDP", count, tempoInicio, tempoFinal, ipVitima, payload))
  protocol_id += 1

ssdp_connection.close()

mix_cursor.executemany('INSERT INTO MIX_PROTOCOL VALUES (?,?,?,?,?,?,?)', protocol_mix)
mix_connection.commit()
print('ssdp finish: ', len(protocol_mix))


protocol_mix = []
steam_games_connection = sqlite3.connect('../db/database-2022-05-11/dnstor_statistics_steam_games.sqlite', timeout=10)
steam_games_cursor = steam_games_connection.cursor()

for steam_games_row in steam_games_cursor.execute("""
   SELECT count, tempoInicio, tempoFinal, ip, payload
     FROM MEMORY_DICT
LEFT JOIN PAYLOAD_DICT
       ON MEMORY_DICT.payloadID = PAYLOAD_DICT.payloadID;
"""):
  count = int(steam_games_row[0])
  tempoInicio = steam_games_row[1]
  tempoFinal = steam_games_row[2]
  ipVitima = steam_games_row[3]
  payload = steam_games_row[4]

  protocol_mix.append((protocol_id, "STEAM_GAMES", count, tempoInicio, tempoFinal, ipVitima, payload))
  protocol_id += 1

steam_games_connection.close()

mix_cursor.executemany('INSERT INTO MIX_PROTOCOL VALUES (?,?,?,?,?,?,?)', protocol_mix)
mix_connection.commit()
print('steam_games finish: ', len(protocol_mix))




mix_connection.close()

