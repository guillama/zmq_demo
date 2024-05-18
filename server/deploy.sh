#/bin/bash

set -e
set -x

TARGET_BINARY="target/armv7-unknown-linux-gnueabihf/release/rasp0mq"

if [ $# -lt 2 ]
then
    echo "Usage: $0 [BINARY] [INSTANCE]"
    echo "Ex: $0 ${TARGET_BINARY} verbose"
    echo "Ex: $0 ${TARGET_BINARY} noverbose"
    exit 1
fi

SRC_BIN=${1}
SERVICE_NAME="rasp0mq"
SERVICE_FILENAME="rasp0mq@.service"
SERVICE_LOG_FILE="/tmp/${SRC_BIN}.log"
TARGET_SSH="maxime@192.168.1.19"
TARGET_DIR="/home/maxime/"
TARGET_SERVICE_DIR=${TARGET_SSH}:${TARGET_DIR}/.config/systemd/user/

ssh ${TARGET_SSH} "systemctl --user stop ${SERVICE_NAME}@noverbose" || true
ssh ${TARGET_SSH} "systemctl --user stop ${SERVICE_NAME}@verbose" || true
ssh ${TARGET_SSH} "rm -f ${SERVICE_LOG_FILE}"

rsync ${SERVICE_FILENAME} ${TARGET_SERVICE_DIR}
rsync ${SRC_BIN} ${TARGET_SSH}:${TARGET_DIR}

ssh ${TARGET_SSH} "systemctl --user daemon-reload"
ssh ${TARGET_SSH} "systemctl --user start ${SERVICE_NAME}@$2"
ssh ${TARGET_SSH} "systemctl --user enable ${SERVICE_NAME}@$2"