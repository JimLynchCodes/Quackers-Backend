#!/bin/bash
EXECUTABLE_FILE_NAME="quackers-ws-server"

# Delete the previous rollback option executable in the "off_deck" folder (if it exists)
[ -f "/root/off_deck/$EXECUTABLE_FILE_NAME" ] && rm -f /root/off_deck/$EXECUTABLE_FILE_NAME

# Move the currently running executable from "live" to "off_deck" (if it exists)
[ -f "/root/live/$EXECUTABLE_FILE_NAME" ] && mv /root/live/$EXECUTABLE_FILE_NAME /root/off_deck/

# Move new executable from "on_deck" to "live"
mv /root/on_deck/$EXECUTABLE_FILE_NAME /root/live/

# Restart the running server with systemctl
sudo systemctl daemon-reload
sudo systemctl restart quackers_ws_server_systemd.service
