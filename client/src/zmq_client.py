import zmq
import os
import sys
import random
import time
import zmq
import click

from gen.zmqproto_pb2 import ZmqProtocol

PORT = 8000
MAX_DATA_SIZE = 24
TARGET_IP = "192.168.1.19"

class ZmqInterface:
    seq_num = 0

    def __init__(self, socket_type, id):
        self.context = zmq.Context()
        self.socket = self.context.socket(socket_type)
        self.socket.connect(f"tcp://{TARGET_IP}:{PORT}")

        self.message = ZmqProtocol()
        self.id = id
    
    def send_random_msg(self):
        self.message.msg_type = random.randint(0, 2)
        self.message.id = self.id
        self.message.seq_num = self.seq_num
        self.message.size = random.randint(1, MAX_DATA_SIZE)
        self.message.data = os.urandom(self.message.size)

        self.seq_num += 1

        self.socket.send(self.message.SerializeToString())

    def recv(self):
        return self.socket.recv()

    def __del__(self):
        print("Destroy zmq socket and context")
        self.socket.close(linger=0)
        self.context.term()
        self.context.destroy()

@click.command()
@click.option('--id', type=int, help='Set client identifier', required=True)
@click.option('--verbose', type=bool, help='Enable verbose mode', required=False, is_flag=True)
def start_client_command(id: int, verbose: bool):
    start_client_loop(id, verbose)

def start_client_loop(id: int, verbose: bool):
    zmq_socket = ZmqInterface(zmq.REQ, id)

    while True:
        try:
            run(zmq_socket, verbose)
        except KeyboardInterrupt:
            print(f"Warning: Program has been interrupted")
            break
        except zmq.ZMQError as e:
            print(f"Error : {e}")
            break

def run(zmq_socket, verbose: bool):
    zmq_socket.send_random_msg()
    msg = zmq_socket.recv()

    # Decode message to printable format
    decoded_msg = ZmqProtocol()
    decoded_msg.ParseFromString(msg)

    if verbose:
        print(f"Received : {decoded_msg}")

if __name__ == "__main__":
    start_client_command()