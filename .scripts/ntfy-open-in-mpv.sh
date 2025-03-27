#!/bin/bash

print_notification() {
  notify-send \
    -t 4000 \
    -e \
    --hint string:x-dunst-stack-tag:open-in-mpv \
    -a "$1" \
    "$2"
}

curl -sN ntfy.sh/"$TOPIC"/raw |
  while read -r line; do
    [ -z "$line" ] && continue
    print_notification 'open-in-mpv' "attempting to play: '$line'"
    (mpv "$line" || print_notification 'open-in-mpv' "error during playback: '$line'") &
  done
