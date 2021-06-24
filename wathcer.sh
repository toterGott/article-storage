#!/bin/sh
while true; do
	ps cax | grep $(cat /var/run/article-storage.pid) > /dev/null
	if [ $? -ne 0 ]
	then
		./run.sh
	fi
	sleep 1
done

