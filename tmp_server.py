import json
import socket
import threading

from pprint import pprint

class Colours:
    HEADER = '\033[95m'
    OKBLUE = '\033[94m'
    OKCYAN = '\033[96m'
    OKGREEN = '\033[92m'
    WARNING = '\033[93m'
    FAIL = '\033[91m'
    ENDC = '\033[0m'
    BOLD = '\033[1m'
    UNDERLINE = '\033[4m'


def tmp_display(json_data):
    """
        Tmp display function that I will replace
        with something better later.

        For now I just want to get to researching
        and dont want to focus on this script forever.
    """

    # I am creating a multiline string instead
    # of just multiple prints because 2 instances
    # of this function could run at almost the same
    # time and I dont rly want the outputs messed up
    output = [
            f"{Colours.HEADER}{json_data['args']}{Colours.ENDC}",
        ]

    for call in json_data['stack']:
        output.append(f"\t{Colours.OKCYAN}{call['code']:<90}{call['path']:<150}Line: {call['line']} {Colours.ENDC}")

    output.append("\n")

    for line in json_data['code_snippet']:
        if line.strip() == json_data['stack'][-1]['code'].strip():
            output.append(f"\t{Colours.WARNING}{line} {Colours.ENDC}")
        else:
            output.append(f"\t{line}")

    output.append("\n\n")

    print('\n'.join(output))


class SocketThread:
    def __init__(self, c, addr):
        self.c = c
        self.host, self.port = addr

        self.data = self._recv_data()

        tmp_display(json.loads(self.data))

    def _recv_data(self):
        data = b''
        while not self.c._closed:

            # Get one mb of data
            # If it fails then the connection is closed.
            try:
                char = self.c.recv(1)
            except:
                break

            # client should send a null byte at the end of the stream
            if char == b'\0':
                break
            # recv just returns empty strings
            # when the socket is open but nothing is sent
            if len(char) == 0:
                continue


            data += char

        try:
            decoded = data.decode('utf-8')
        except UnicodeDecodeError:
            print("Could not decode data")
            print(data)

        return decoded







def main():

    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    s.bind(('127.0.0.1', 3001))
    s.listen(10000)

    threads = []
    while True:
        try:
            x = threading.Thread(target=SocketThread, args=s.accept())
            x.start()
            threads.append(x)
        except Exception as e:
            print(e)
            break

    # kill all threads
    print("Shutting down all threads...")
    for x in threads:
        x.kill()

    print("bye!")





if __name__ == "__main__":
    main()
