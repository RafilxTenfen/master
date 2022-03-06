  SELECT year, period, qname, qtype, SUM(count) as count
    FROM DNS_ANALYSIS
GROUP BY year, period, qname, qtype
ORDER BY count DESC;
