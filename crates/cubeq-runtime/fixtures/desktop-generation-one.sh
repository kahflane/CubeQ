#!/bin/sh
printf 'stable-generation-one:%s\n' "$CUBEQ_DESKTOP_SUPERVISED"
: > "$CUBEQ_DESKTOP_RESTART_MARKER"
mv "$(dirname "$0")/desktop-child.next" "$0"
exit 75
