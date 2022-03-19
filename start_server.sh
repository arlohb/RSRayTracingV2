#!/bin/bash
set -eu

# Starts a local web-server that serves the contents of the `doc/` folder,
# which is the folder to where the web version is compiled.

cargo install basic-http-server

# https://stackoverflow.com/questions/21336126/linux-bash-script-to-extract-ip-address
my_ip=$(ip route get 8.8.8.8 | awk -F"src " 'NR==1{split($2,a," ");print a[1]}')

cd docs
basic-http-server --addr "${my_ip}:8080" .
