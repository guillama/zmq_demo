#/bin/bash

set -e

#export OUT_DIR=""

SRC_BIN=${1}
SERVICE_NAME="rasp0mq"
TARGET_SSH="maxime@192.168.1.19"
TARGET_DIR="/home/maxime/"
TARGET_SERVICE_DIR=${TARGET_SSH}:${TARGET_DIR}/.config/systemd/user/

ssh ${TARGET_SSH} "systemctl --user stop ${SERVICE_NAME}"

rsync ${SERVICE_NAME}.service ${TARGET_SERVICE_DIR}
rsync ${SRC_BIN} ${TARGET_SSH}:${TARGET_DIR}

ssh ${TARGET_SSH} "systemctl --user daemon-reload"
ssh ${TARGET_SSH} "systemctl --user start ${SERVICE_NAME}"