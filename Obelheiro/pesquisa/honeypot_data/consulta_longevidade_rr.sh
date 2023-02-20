#!/usr/bin/env bash
sudo resolvectl flush-caches

dig +novc +edns=0 +bufsize=16384 +dnssec ambulance.gov.ae MX
dig +novc +edns=0 +bufsize=16384 +dnssec ncc.ae MX
dig +novc +edns=0 +bufsize=16384 +dnssec szgmc.gov.ae. MX
dig +novc +edns=0 +bufsize=16384 +dnssec dc.gov.ae. MX
dig +novc +edns=0 +bufsize=16384 +dnssec fca.gov.ae. MX
dig +novc +edns=0 +bufsize=16384 +dnssec dans.gov.ae. MX
dig +novc +edns=0 +bufsize=16384 +dnssec almajles.gov.ae. MX
dig +novc +edns=0 +bufsize=16384 +dnssec investbank.ae. MX
dig +novc +edns=0 +bufsize=16384 +dnssec tstng.net. A
dig +novc +edns=0 +bufsize=16384 +dnssec pwad.gov.ae. MX