#!/bin/bash

set -euo pipefail

while true; do
  battery_level="$(cat /sys/class/power_supply/BAT0/capacity)"

  [ "$battery_level" -lt 15 ] &&
    notify-send -a "BATTERY LEVEL LOW" "$battery_level" \
      -t 5000 \
      -e \
      --hint int:value:"$battery_level" \
      --hint string:x-dunst-stack-tag:low-battery

  sleep 90s
done
