#!/usr/bin/bash

# sudo apt install batctl -y
sudo service wpa_supplicant stop
sudo systemctl mask wpa_supplicant.service
sudo ip link set wlan0 down
sudo iw dev wlan0 set type ibss
sudo ip link set mtu 1500 dev wlan0
sudo iw wlan0 set type ibss
sudo ifconfig wlan0 mtu 1500
sudo iwconfig wlan0 channel 3
sudo ip link set wlan0 up
sudo iw wlan0 ibss join robo-bivouac 2432

sudo modprobe batman-adv
sudo batctl if add wlan0
sudo ip link set up dev wlan0
sudo ip link set up dev bat0
sudo ifconfig bat0 172.27.0.1/16 # Assign an IP address to the bat0 interface

echo "Ad-hoc network started with IP address 172.16.0.1/24"
