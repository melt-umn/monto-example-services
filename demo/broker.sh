#!/bin/bash

set -eu

tmux new-session -d -n broker -s broker "monto-broker"
tmux new-window -n mitmproxy -t broker:1 "mitmproxy -p 28888 -R http://localhost:28889"
tmux attach -t broker:1
