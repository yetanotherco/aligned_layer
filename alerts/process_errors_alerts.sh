#!/bin/bash

source $1

journalctl -feu "$SERVICE"  -n 0 | while read LINE; do
  (echo "$LINE" | grep -e "$EXPRESSION") && curl -X POST --silent --data-urlencode \
    "payload={\"text\": \"$(echo $LINE | sed "s/\"/'/g")\"}" "$SLACK_WEBHOOK_URL";
done
