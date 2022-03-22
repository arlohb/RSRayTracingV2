#!/bin/bash
set -eu

# I need custom headers to enable multithreading in the web

# cargo basic-http-server does not allow for custom headers

# If I instead used node http-server,
# This PR would allow for custom headers
# https://github.com/http-party/http-server/pull/282
# But it been LITERALLY 6 YEARS without being merged.
# I could just use the PR,
# but a better solution would be great
# http-server has SO MANY forks, so maybe one of them has this

# Perhaps I need to make my own simple server,
# but there's got to be one out there that allows for custom headers

# This python script can do it with Simple HTTP Server
# https://gist.github.com/daimatz/3ca32ae11d9635372853
# this is probably best

cargo install basic-http-server

# https://stackoverflow.com/questions/21336126/linux-bash-script-to-extract-ip-address
my_ip=$(ip route get 8.8.8.8 | awk -F"src " 'NR==1{split($2,a," ");print a[1]}')

cd docs
basic-http-server --addr "${my_ip}:8080" .
