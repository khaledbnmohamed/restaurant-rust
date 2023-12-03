import socket
import sys
import threading
import json

table_amount = 10
item_amount = 20
num_thread = 10


def send_recv(sock, msg):
    sock.send(msg.encode())
    _ = sock.recv(512)


def send_recv_json(sock, msg):
    sock.send(msg.encode())
    res = ""
    data = sock.recv(512)
    print("response is {}", data)
    res += data.decode("utf-8")
    flag = len(data) == 512
    while flag:
        data = sock.recv(512)
        res += data.decode("utf-8")
        flag = len(data) == 512

    try:
        return json.loads(res)
    except json.JSONDecodeError as e:
        print("Error decoding JSON:", e)
        return None


def run_client_add(host, port, thread_id):
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

    try:
        s.connect((host, port))
        print('Connect to %s:%d' % (host, port))
    except:
        print('Unable to connect %s:%d' % (host, port))
        exit(1)

    for table_id in range(0, table_amount):
        item_id_start = item_amount * thread_id
        item_id_end = item_amount * (thread_id + 1)

        # Case 1: Send JSON payload
        for item_id in range(item_id_start, item_id_end):
            json_payload = {
                "item_name": f"Potato Chips {item_id}",
                "preparation_time_minutes": "5",
                "table_number": str(table_id)
            }
            json_str = json.dumps(json_payload)
            content_length = len(json_str)

            request = (
                f"POST /api/items/ HTTP/1.1\r\n"
                f"Host: {host}:{port}\r\n"
                f"Content-Type: application/json\r\n"
                f"Content-Length: {content_length}\r\n\r\n"
                f"{json_str}"
            )

            send_recv_json(s, request)

    s.close()

def run_client_check_all(host, port):
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

    try:
        s.connect((host, port))
        print('Connect to %s:%d' % (host, port))
    except:
        print('Unable to connect %s:%d' % (host, port))
        exit(1)

    print("=== Checking ===")

    for table_id in range(1, table_amount):
        print("Debugging {}", table_id)
        response = send_recv_json(
            s, f"GET /api/items?table_number={table_id} HTTP/1.1\r\nHost: {host}:{port}\r\n\r\n")

        print("Debugging response {}", response)
        # Handle the case where the response is empty or not a valid JSON
        if response is None:
            print("table {} has an empty or invalid response".format(table_id))
            exit(1)

        if len(response) != item_amount * num_thread:
            print("table {} has an incorrect amount of items".format(table_id))
            exit(1)

    print("All tables have the correct amount of items")

    s.close()


if __name__ == '__main__':
    host = '127.0.0.1'
    port = 8000

    if len(sys.argv) == 3:
        host = sys.argv[1]
        port = int(sys.argv[2])
    elif len(sys.argv) != 1:
        print('Usage: client.py [host] [port]')
        exit(1)

    threads = []

    print("Running {} threads...".format(num_thread))
    print("Each thread adds {} items for each {} tables.".format(item_amount, table_amount))
    for i in range(0, num_thread):
        t = threading.Thread(target=run_client_add, args=(host, port, i))
        threads.append(t)
        t.start()

    for i in range(0, num_thread):
        t = threads[i]
        t.join()

    run_client_check_all(host, port)
