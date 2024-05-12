#!/bin/bash

python3 -m grpc_tools.protoc -I=protobuf/ --python_out=client/src/gen/ --grpc_python_out="client/src/gen/" protobuf/zmqproto.proto