#!/bin/bash

set -eu

tmux new-session -d -n service -s service-example "monto-example-services"
tmux new-window -n mitmproxy -t service-example:1 "mitmproxy -p 10000 -R http://localhost:10001"
tmux attach -t service-example:1
