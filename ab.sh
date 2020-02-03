#!/bin/bash

NODE_PORT="${NODE_PORT-3000}"
BASE_URL="http://localhost:${NODE_PORT}"
MAX_NUMBER="1000000"

temp_file=$(mktemp)

trap 'rm -f "$temp_file"; exit' INT TERM
trap 'rm -f "$temp_file"; kill 0' EXIT

cat > "$temp_file" << EOF
{
    "max": $MAX_NUMBER
}
EOF

node src/server > /dev/null &

until curl -s "${BASE_URL}/healthz" > /dev/null; do
    sleep 0.1
done

echo -n "Sync Results:  "

ab \
    -c 10 \
    -n 1000 \
    -p "$temp_file" \
    -T 'application/json' \
    "${BASE_URL}/api/sync" 2>&1 |
grep 'Requests per second'

echo -n "Async Results: "

ab \
    -c 10 \
    -n 1000 \
    -p "$temp_file" \
    -T 'application/json' \
    "${BASE_URL}/api/async" 2>&1 |
grep 'Requests per second'
