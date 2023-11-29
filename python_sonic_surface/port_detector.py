"""
Utility that prints out serial port names.
"""
import serial.tools.list_ports


def print_serial_ports():
    """
    Get the port name of a connected Arduino if one is connected.
    Iterates over the serial ports and looks for Arduino or "USB-SERIAL" in their names to auto connect to a port.
    :return: a string that is the name of the connected Arduin's port, or a RuntimeError if none found
    """
    ports = list(serial.tools.list_ports.comports())
    if len(ports) == 0:
        error = "No COM ports detected, is the Arduino plugged in?"
        print(error)
        raise RuntimeError(error)
    print("#####\nDetected the following COM ports (name, description, device): ")
    for p in ports:
        print(f"{p.name}: {p.description}, {p.device}")


if __name__ == "__main__":
    print_serial_ports()