# original script that recreated the java code in python
import serial
off = b"\xc0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x20\x00\x00\x20\x20\x00\x00\x20\x20\x00\x00\x20\x20\x00\x00\x20\x20\x00\x00\x20\x20\x00\x00\x20\x20\x00\x00\x20\x20\x00\x00\x20\x20\x00\x00\x20\x20\x00\x00\x20\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\xfd"
on = b'\xc0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x03\x08\x02\x02\x01\x00\x08\x06\x00\x03\x04\x04\x04\x03\x06\x05\x02\x04\x03\x1f\x1f\x04\x08\x01\x05\x06\x06\x01\x01\x00\t\x01\x02\x06\x01\x04\x08\x01\x07\x05\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x02\x06\x04\x05\x06\x07\x06\x02\x02\x05\x1e\x03\x04\x02\x05\x1e\x06\x04\x02\x04\x00\x05\x04\x1f\x03\x03\x04\x00\x1f\x01\x05\x04\x04\x00\x07\x03\x05\x06\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x05\x00\x00\x07\x07\x00\x00\t\x07\x00\x00\x06\x03\x00\x00\x03\x02\x00\x00\x08\x06\x00\x00\x03\x01\x00\x00\x04\x03\x00\x00\x06\x02\x00\x00\x04\x03\x00\x00\x05\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\xfd'



# Open the serial connection
try:
    ser = serial.Serial('/dev/tty.usbserial-10', baudrate=230400)
except serial.SerialException as e:
    print("Failed to open serial port:", e)
else:
    # Check if the serial port is open
    if ser.isOpen():
        while True:
            try:
                key = input("Press enter to send on message")
                if key != "":
                    break
                ser.write(on)
                ser.flush()
                print("Message sent successfully.")
                key = input("Press enter to send off message")
                if key != "":
                    break
                ser.write(off)
                ser.flush()
            except serial.SerialException as e:
                print("Failed to send message:", e)
        print("Exiting")
    else:
        print("Serial port is not open.")