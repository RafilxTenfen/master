SELECT ip, count, tempoInicio, tempoFinal, NTP_MEMORY_DICT.payloadID, payload,
  FROM NTP_MEMORY_DICT
  JOIN NTP_PAYLOAD_DICT
    ON NTP_MEMORY_DICT.payloadID = NTP_PAYLOAD_DICT.payloadID


