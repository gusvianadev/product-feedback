#!/bin/bash

session="product-feedback"

tmux new-session -d -s $session

window=1
tmux rename-window -t $session:$window "neovim"
tmux send-keys -t $session:$window "v ." c-m

tmux attach-session -t $session
