#!/bin/bash
# NOTE: this file should be placed at /usr/local/bin/special_ping.sh

PING=$(ping -qc1 discord.com 2>&1 | awk -F'/' 'END{ print (/^rtt/? $5:"FAIL") }')

if [ $PING == "FAIL" ]; then
	echo '- FAIL'
else
	PING2=$(echo $PING | cut -d '.' -f1)
	echo + OK $PING2 ms
fi
