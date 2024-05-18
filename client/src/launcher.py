import subprocess

CLIENT_SCRIPT="zmq_client.py"

def exec(cmd, check=True):
    subprocess.run(cmd, shell=True, check=check, )


def main():
    try:
        start_clients()
    except KeyboardInterrupt:
        print("Terminate zmq_client instances")
        exec(f"pkill -f {CLIENT_SCRIPT}", check=False)

def start_clients():
    for id in range(1, 101):
        print(f"Start {CLIENT_SCRIPT} with id {id}")
        exec(f"python3 {CLIENT_SCRIPT} --id {id} &")

    while True:
        pass

if __name__ == "__main__":
    main()