# used for testing when we were sending serial data to com0com
import serial

# Replace 'COM10' with the correct COM port you want to read from
ser = serial.Serial('COM17', baudrate=230400, timeout=1)  # Adjust baudrate and timeout as needed

try:
    while True:
        # Read data from the serial port

        data = ser.readline().decode().strip()

        if data:
            print(data)  # Print received data
except KeyboardInterrupt:
    ser.close()