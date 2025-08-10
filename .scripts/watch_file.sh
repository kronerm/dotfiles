#!/bin/sh

FILE_PATH="$1"

cat "$FILE_PATH"
while inotifywait -e close_write "$FILE_PATH" >/dev/null 2>&1; do
  sleep 0.1
  cat "$FILE_PATH"
done
