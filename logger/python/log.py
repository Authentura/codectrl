import os
import sys
import json
import socket
import traceback

from pprint import pprint
from colorama import Fore, Back, Style


# You may be tempted to ask: "Why is this file so complex for something so simple?"
# You may even think to yourself "I could do this in 10 lines of code".
#
# Well the answer is simple:
#   This is nowhere near done, and is only a very early version of this code.
#
# Some possible upcoming features:
#    - Showing a couple lines above and before the log code. (NOTE: addding this now)
#    - Optional breakpoint (stoping execution with input())
#    - Some other, probably fancier way of displaying stuff
#    - Configurable colours


class traceback_file:
    def __init__(self, file_data):
        """
            Not goint to document this entire function at
            1 am, but instead am going to regret not doing
            so later.

            As a general idea tho, It takes the string that
            is returned by the traceback module and makes
            it into an object with the following properties.


            Properties:
              - path  // where its at
              - fname  // name of file
              - line  // line number (int)
              - place // Function or module its in
              - code  // line of code
        """
        self._format(file_data)

        self.json = {
            "path": self.path,
            "fname": self.fname,
            "line": self.line,
            "place": self.place,
            "code": self.code,
        }

    def _file_name(self, file_data):
        file_data = file_data.split()
        self.path = file_data[1].strip('"')
        self.fname = file_data[1].split('/')[-1].strip('"')

    def _line_no(self, line_data):
        line_data = line_data.split()
        try:
            self.line = int(line_data[1])
        except:
            self.line = line_data[1]

    def _get_code(self, data):
        data = data.split('\n')
        self.place = data[0].strip(' in ')
        self.code = data[1].strip()

    def _format(self, file_data):
        data_array = file_data.strip('\n').split(',')
        self._file_name(data_array[0])
        self._line_no(data_array[1])
        # If there are any commas in the function call
        # then just add all the rest here
        self._get_code(','.join(data_array[2:]))


class log:
    def __init__(self, data):
        """Log stuff in a cool way"""
        self.argument = self._check_arg(data)

        self.stack = self._get_stack()

        self.code_snippet = self._get_neighbour_lines()

        self.json = self._format_json()

        try:
            s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            s.connect(('127.0.0.1', 3001))
        except Exception as e:
            print("Error logger could not connect to server!", file=sys.stderr)
            sys.exit(-1)

        s.send(json.dumps(self.json).encode("utf-8"))
        s.send(b'\0')
        s.close()

    def _get_stack(self):
        """ Get the current call stack formatted """
        stack = list(traceback.format_stack())

        stack.pop()
        stack.pop()
        stack_formatted = []

        for call in stack:
            stack_formatted.append(traceback_file(call))

        return stack_formatted

    def _check_arg(self, args):
        self.arg_type = 'string'
        if type(args) == str:
            self.arg_type = 'string'
            return args

        # If the type is bytes or bytearray then try  convert it to
        # str, each char that fails should get escaped.
        elif type(args) == bytes or type(args) == bytearray:
            string = ''
            for c in args:
                try:
                    string += chr(c)
                except:
                    self.arg_type = 'string/escaped'
                    string += '\\' + str(hex(ord(c)))[1:]

            return string

        else:
            self.arg_type = type(args).__name__
            return args

    def _format_json(self):
        return {
            "args": self.argument,
            "atype": self.arg_type,
            "stack": [x.json for x in self.stack],
            "code_snippet": self.code_snippet
        }

    def _get_neighbour_lines(self):
        """
            Function gets a few lines above and
            below the line that called the log
            function
        """
        # print(self.stack[-1].code)

        with open(self.stack[-1].fname, "r")as ifstream:
            code = ifstream.read().split('\n')

        useful_lines = []
        for i in range(self.stack[-1].line-5, self.stack[-1].line+5):
            try:
                useful_lines.append(code[i])
            except IndexError:
                pass

        return useful_lines
