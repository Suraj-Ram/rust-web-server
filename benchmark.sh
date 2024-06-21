#!/bin/bash

# URL to send requests to
url="http://localhost:7878/sleep"

# Send 4 requests concurrently 
curl -Z -X GET $url  &
curl -Z -X GET $url  &
curl -Z -X GET $url  &
curl -Z -X GET $url  &
wait
