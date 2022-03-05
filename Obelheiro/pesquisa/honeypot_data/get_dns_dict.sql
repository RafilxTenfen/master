
SELECT strftime("%Y", tempoFinal) as year, ((strftime("%m", tempoFinal) - 1) / 3) + 1 AS period, DNS_MEMORY_DICT.*, DNS_PAYLOAD_DICT.payload
  FROM DNS_MEMORY_DICT
  JOIN DNS_PAYLOAD_DICT
    ON DNS_MEMORY_DICT.payloadID == DNS_PAYLOAD_DICT.payloadID
 LIMIT 5;
