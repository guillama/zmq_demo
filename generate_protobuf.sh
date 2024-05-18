#!/bin/bash

set -e

PROTO_FILE="protobuf/zmqproto.proto"
PROTO_INCLUDE_DIR="protobuf"
PROTO_DEST_DIR="client/src/gen/"

python3 -m grpc_tools.protoc -I=${PROTO_INCLUDE_DIR} --python_out=${PROTO_DEST_DIR} --grpc_python_out=${PROTO_DEST_DIR} ${PROTO_FILE}