import socket
import sys
import threading
import json

TABLE_AMOUNT = 100
ITEM_AMOUNT = 20
NUM_THREAD = 10

def test_all(host, port):
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

    try:
        s.connect((host, port))
        print(f'✅✅✅ Connected to {host}:{port}')
    except:
        print(f' ✖️✖️✖️ Unable to connect {host}:{port}')
        exit(1)

    print("=== Checking ===")

    for table_number in range(TABLE_AMOUNT):
        response = send_recv_json(s, f"GET /get/{table_number}".encode())
        if len(response) != ITEM_AMOUNT * NUM_THREAD:
            print(f"Table {table_number} has an incorrect amount of items")
            exit(1)

    print("💃💃💃💃💃💃💃 Tables are all correct 💃💃💃💃💃💃💃 ")

    s.close()

def send_recv(socket, msg):
    socket.send(msg)
    _ = socket.recv(512)


def send_recv_json(socket, msg):
    socket.send(msg)
    res = ""
    data = socket.recv(512)
    res += data.decode("utf-8")
    flag = len(data) == 512
    while flag:
        data = socket.recv(512)
        res += data.decode("utf-8")
        flag = len(data) == 512

    return json.loads(res)


def run_client_add(host, port, thread_id):
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

    try:
        s.connect((host, port))
        print(f'✅✅✅ Connected to {host}:{port}')
    except:
        print(f'✖️✖️✖️ Unable to connect {host}:{port}')
        exit(1)

    for table_number in range(TABLE_AMOUNT):
        item_id_start = ITEM_AMOUNT * thread_id
        item_id_end = ITEM_AMOUNT * (thread_id + 1)

        for item_id in range(item_id_start, item_id_end):
            send_recv(s, f"POST /add/{table_number}/{item_id}".encode())

    s.close()





def main():
    host = '127.0.0.1'
    port = 8080

    if len(sys.argv) == 3:
        host = sys.argv[1]
        port = int(sys.argv[2])
    elif len(sys.argv) != 1:
        print('Usage: client.py [host] [port]')
        exit(1)

    threads = []

    print(f"Running {NUM_THREAD} threads...")
    print(f"Each thread adds {ITEM_AMOUNT} items for each {TABLE_AMOUNT} tables.")

    for i in range(NUM_THREAD):
        t = threading.Thread(target=run_client_add, args=(host, port, i))
        threads.append(t)
        t.start()

    for t in threads:
        t.join()

    test_all(host, port)


if __name__ == '__main__':
    main()
