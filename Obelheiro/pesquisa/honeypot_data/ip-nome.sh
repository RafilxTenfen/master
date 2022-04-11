#! /bin/sh

# tenta realizar a resolução reversa de IPs no arquivo passado por parametro

for ip in `cat $1`
do
    echo -n "${ip}   "
    OUTPUT=`host $ip | grep domain`

    if [ -z "${OUTPUT}" ]; then
	    echo
    else
	    echo ${OUTPUT} | cut -f5 -d" "
    fi
done
