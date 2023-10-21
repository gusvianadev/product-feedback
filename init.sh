#!/bin/bash

session="product-feedback"

tmux new-session -d -s $session

window=1
tmux rename-window -t $session:$window "neovim" c-m
tmux send-keys -t $session:$window "v ." c-m

window=2
tmux new-window -t $session:$window -n "server"
tmux send-keys -t $session:$window "cargo watch -x 'shuttle run --port 3000'" c-m

tmux attach-session -t $session
