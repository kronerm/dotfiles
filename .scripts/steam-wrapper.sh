#!/usr/bin/env bash

set -Eeuo pipefail

gamescope=""

if [ "${GAMESCOPE_DISABLED:-"0"}" != "1" ]; then
  available_modes_with_refresh_rates_json="$(wlr-randr --json | jq '[.[].modes[]] | group_by(.width, .height) | map({"\(.[0].width)x\(.[0].height)": map(.refresh)}) | add')"
  selected_display_resolution="$(echo -e "$available_modes_with_refresh_rates_json" | jq -r 'keys[]' | sort -h | rofi -dmenu)"
  selected_display_refresh_rate="$(echo -e "$available_modes_with_refresh_rates_json" | jq -r ".\"$selected_display_resolution\"[]" | sort -h | rofi -dmenu)"

  selected_display_resolution_width="$(echo "$selected_display_resolution" | cut -d'x' -f1)"
  selected_display_resolution_height="$(echo "$selected_display_resolution" | cut -d'x' -f2)"

  gamescope="gamescope -b -W $selected_display_resolution_width -H $selected_display_resolution_height -r $selected_display_refresh_rate --"
fi

$gamescope "${@:1}"
