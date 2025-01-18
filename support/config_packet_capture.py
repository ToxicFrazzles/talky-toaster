import time
from typing import List, Tuple
import serial
from multiprocessing import Process, Queue



COMMAND_HEADER = b"\xfd\xfc\xfb\xfa"
COMMAND_FOOTER = b"\x04\x03\x02\x01"



KNOWN_COMMANDS = [
    (b"\xfd\xfc\xfb\xfa\x02\x00\x00\x00\x04\x03\x02\x01", "Read firmware version"),
    (b"\xfd\xfc\xfb\xfa\x0c\x00\x00\x01\x00\x00\x06\x00\x76\x31\x2e\x35\x2e\x39\x04\x03\x02\x01", "Firmware version response"),
    (b"\xfd\xfc\xfb\xfa\x02\x00\x11\x00\x04\x03\x02\x01", "Read serial number"),
    (b"\xfd\xfc\xfb\xfa\x0e\x00\x11\x01\x00\x00\x08\x00\x46\x46\x46\x46\x46\x46\x46\x46\x04\x03\x02\x01", "Serial number response"),
    (b"\xfd\xfc\xfb\xfa\x08\x00\x12\x00\x00\x00\x00\x00\x00\x00\x04\x03\x02\x01", "Set debug mode"),
    (b"\xfd\xfc\xfb\xfa\x08\x00\x12\x00\x00\x00\x04\x00\x00\x00\x04\x03\x02\x01", "Set report mode"),
]



PORT_TX = "COM4"
PORT_RX = "COM5"



def command_reader(name: str, ser_name: str, queue: Queue):
    with serial.Serial(ser_name, 115200, timeout=0.1) as ser:
        data = b""
        while ser.readable():
            data += ser.read_until(COMMAND_FOOTER)
            commands, end = extract_commands(data)
            data = data[end:]
            for command in commands:
                queue.put((name, command))


def extract_commands(data: bytes) -> Tuple[List[bytes], int]:
    commands = []
    data_end = None
    while data:
        command_header_offset = data.find(COMMAND_HEADER)
        if command_header_offset == -1:
            data_end = len(data) -6
            break
        
        command_footer_offset = data.find(COMMAND_FOOTER, command_header_offset)
        if command_footer_offset == -1:
            break

        data_end = command_footer_offset+4
        commands.append(data[command_header_offset:data_end])
        data = data[data_end:]
    return commands, data_end


def capture(ser_tx: serial.Serial, ser_rx: serial.Serial):
    data_tx = b""
    data_rx = b""
    while ser_tx.readable() and ser_rx.readable():
        data_tx += ser_tx.read_all()
        data_rx += ser_rx.read_all()
        # print(len(data_tx), len(data_rx))
        commands, tx_end = extract_commands(data_tx)
        data_tx = data_tx[tx_end:]
        for command in commands:
            if command in KNOWN_COMMANDS: 
                continue
            print("TX:", command.hex(" "))
        
        commands, rx_end = extract_commands(data_rx)
        data_rx = data_rx[rx_end:]
        for command in commands:
            if command in KNOWN_COMMANDS: 
                continue
            print("RX:", command.hex(" "))


if __name__ == "__main__":
    command_queue = Queue(64)
    p_tx = Process(target=command_reader, args=("TX", PORT_TX,command_queue))
    p_rx = Process(target=command_reader, args=("RX", PORT_RX,command_queue))
    p_tx.start()
    p_rx.start()
    while True:
        task_name, cmd = command_queue.get()
        if len(cmd) > 200:
            continue
        cmnt = ""
        for command, comment in KNOWN_COMMANDS:
            if cmd == command:
                cmnt = comment
                break
        
        header = cmd[:4]
        length = cmd[4:6]
        cmd_code = cmd[6:8]
        remainder = cmd[8:-4]
        footer = cmd[-4:]

        nice_cmd = f"{header.hex(' ')} - {length.hex(' ')} - {cmd_code.hex(' ')} - {remainder.hex(' '):<65} - {footer.hex(' ')}"
        print(f"{task_name}: {nice_cmd:<120} {cmnt}")
