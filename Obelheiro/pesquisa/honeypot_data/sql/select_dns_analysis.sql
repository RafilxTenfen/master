  SELECT count(*) as countGrouped, year, period, qname, qtype, SUM(count) as quantity
    FROM DNS_ANALYSIS
GROUP BY year, period, qname, qtype
ORDER BY quantity DESC;