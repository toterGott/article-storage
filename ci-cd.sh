#!/bin/bash
while true
do
	git pull > /dev/null 2>&1
	NEW_HASH=$(git rev-parse HEAD)
	OLD_HASH=$(<hash)
	echo $NEW_HASH > hash

	if [ "$NEW_HASH" != "$OLD_HASH" ]
	then
		cargo build --release &&
		( killall article-storage
		nohup ./target/release/article-storage >/dev/null 2>&1 & )
	fi

	sleep 5
done
