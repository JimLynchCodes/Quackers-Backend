#!/bin/bash
EXECUTABLE_FILE_NAME="quackers-ws-server"

# If no off_deck executable exists, print error and exit script
if [ ! -f "/root/off_deck/$EXECUTABLE_FILE_NAME" ]; then
    echo "$EXECUTABLE_FILE_NAME does not exist in /root/off_deck/ the directory... Exiting script."
    exit 1  # Error status code
fi
# Move the current executable in live back to on_deck (if it exists)
[ -f "/root/live/$EXECUTABLE_FILE_NAME" ] && mv /root/live/$EXECUTABLE_FILE_NAME /root/on_deck/

# Move the executable in off_deck back to live (if it exists)
[ -f "/root/off_deck/$EXECUTABLE_FILE_NAME" ] && mv /root/live/$EXECUTABLE_FILE_NAME /root/on_deck/
mv /root/on_deck/$EXECUTABLE_FILE_NAME /root/live/

# Restart the running server with systemctl
sudo systemctl daemon-reload
sudo systemctl restart quackers_ws_server_systemd.service
