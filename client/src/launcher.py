import threading

from zmq_client import start_client_loop

class ClientThread(object):
    def __init__(self, id):
        self.id = id

        thread = threading.Thread(target=self.run)
        thread.daemon = True
        thread.start()

    def run(self):
        print(f"Start id {self.id}")
        start_client_loop(self.id)


def main():
    for id in range(1, 101):
        ClientThread(id)

if __name__ == "__main__":
    main()

    while True:
        pass