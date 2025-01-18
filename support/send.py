import serial
import time


if __name__ == "__main__":
    with serial.Serial("COM3", 115200, timeout=0.1) as ser:
        # ser.write(b"\xfd\xfc\xfb\xfa\x08\x00\x07\x00\x10\x00" b"\x00\xca\x9b\x3b" b"\x04\x03\x02\x01")
        time.sleep(0.1)
        ser.write(b"\xfd\xfc\xfb\xfa\x04\x00\x08\x00\x10\x00\x04\x03\x02\x01")
