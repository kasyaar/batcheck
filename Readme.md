## Overview
Helps me to extend my old zenbook battery life.
Simply checks battery status and show notice to on/off charging manually in case battery in configured threshold(now hardcoded from 81% to 85%).
## Setup
prerequisities: rust and cargo installed, libdebus-dev
### build and install
```shell
mkdir ~/bin
#assumes you've clone repo into workspace dir
cd workspace/batcheck
cargo build --release
cp targer/release/batcheck ~/bin/
crontab -e
```
Then put following strings into crontab:
```shell
#to run batcheck every 5 mins
DISPLAY=":0.0"
XAUTHORITY="/home/username/.Xauthority"
XDG_RUNTIME_DIR="/run/user/1000" #user id in system - can check by run `id` command
*/5 * * * * /home/username/bin/batcheck >> /tmp/batcheck.log

```

## Screenshot
![screenshot](/screenshot.png)

## TODO
set battery charging threshhold as a parameters.

