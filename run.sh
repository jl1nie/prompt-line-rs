#!/bin/bash
# Run prompt-line-rs in WSL (force X11 backend)
WAYLAND_DISPLAY= cargo run "$@"
