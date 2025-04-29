#!/usr/bin/env bash

GAMESCOPE="gamescope -b $(cat /sys/class/drm/*/modes | sort -h | uniq | rofi -dmenu | sed -E 's/^/-W /;s/x/ -H /') -r $(printf '30\n60\n72\n90\n120\n144\n165\n240' | rofi -dmenu) --"
[ "$USE_GAMESCOPE" -eq 0 ] && GAMESCOPE=""

$GAMESCOPE "${@:1}"
