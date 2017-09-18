#!/bin/bash

set -eu

tmux new-session -d -n service -s service-spell-check "monto-spell-check -p 10003"
tmux new-window -n mitmproxy -t service-spell-check:1 "mitmproxy -p 10002 -R http://localhost:10003"
tmux attach -t service-spell-check:1
