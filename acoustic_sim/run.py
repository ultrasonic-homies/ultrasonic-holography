import time
import numpy as np
import hat

import serial
import serial.tools.list_ports

# copied from https://github.com/upnalab/SonicSurface/blob/0ed2604fa0f35c3a1a2991216aaef871016f5ad7/ControlSoftware/Python/SonicSurface.py
class SonicSurface:
    PHASE_DIVS = 32
    N_EMMITERS = 256
    EMITTERS_ORDER = [0, 7, 1, 2, 64, 71, 65, 66, 128, 135, 129, 130, 192, 199, 193, 194, 4, 3, 6, 5, 68, 67, 70, 69, 132, 131, 134, 133, 196, 195, 198, 197, 8, 15, 9, 10, 72, 79, 73, 74, 136, 143, 137, 138, 200, 207, 201, 202, 12, 11, 14, 13, 76, 75, 78, 77, 140, 139, 142, 141, 204, 203, 206, 205, 16, 23, 17, 18, 80, 87, 81, 82, 144, 151, 145, 146, 208, 215, 209, 210, 20, 19, 22, 21, 84, 83, 86, 85, 148, 147, 150, 149, 212, 211, 214, 213, 24, 31, 25, 26, 88, 95, 89, 90, 152, 159, 153, 154, 216, 223, 217, 218, 28, 27, 30, 29, 92, 91, 94, 93, 156, 155, 158, 157, 220, 219, 222, 221, 32, 39, 33, 34, 96, 103, 97, 98, 160, 167, 161, 162, 224, 231, 225, 226, 36, 35, 38, 37, 100, 99, 102, 101, 164, 163, 166, 165, 228, 227, 230, 229, 40, 47, 41, 42, 104, 111, 105, 106, 168, 175, 169, 170, 232, 239, 233, 234, 44, 43, 46, 45, 108, 107, 110, 109, 172, 171, 174, 173, 236, 235, 238, 237, 48, 55, 49, 50, 112, 119, 113, 114, 176, 183, 177, 178, 240, 247, 241, 242, 52, 51, 54, 53, 116, 115, 118, 117, 180, 179, 182, 181, 244, 243, 246, 245, 56, 63, 57, 58, 120, 127, 121, 122, 184, 191, 185, 186, 248, 255, 249, 250, 60, 59, 62, 61, 124, 123, 126, 125, 188, 187, 190, 189, 252, 251, 254, 253]
    
    def __init__(self):
        self.serialConn = None
        self.phaseOffsets = np.zeros( [self.N_EMMITERS] )
        self.phases = np.zeros( [1,self.N_EMMITERS], dtype=np.complex128 )
        self.onOrOff = np.full(self.N_EMMITERS, False)
   
    @staticmethod
    def listSerial():
        ports = serial.tools.list_ports.comports()
        print("Serial Ports:")
        for i, port in enumerate(ports, start=1):
            print(f"{i}: {port.device}")
            
    def disconnect(self):
        if self.serialConn != None:
            self.serialConn.close()
            self.serialConn = None
    
    def connect(self, indexPort):
        if indexPort == -1:
            self.listSerial()
            indexPort = int(input("Enter index of serial port: "))
        selectedPort = serial.tools.list_ports.comports()[indexPort - 1]
        self.disconnect()
        self.serialConn = serial.Serial(selectedPort.device, baudrate=230400)
    
    # Phases range from 0 to 2pi. NaN phase values are deactivated tranducers
    def sendPhases(self, phases, permuteToFPGAOrder=True):
        assert( phases.shape == (self.N_EMMITERS,) )
        deactivated = np.isnan(phases)
        phases = (phases % (2*np.pi)) * self.PHASE_DIVS / 2 / np.pi
        phases[deactivated] = self.PHASE_DIVS
        dataToSend = np.empty(phases.size, np.uint8)
        order = self.EMITTERS_ORDER if permuteToFPGAOrder else np.arange(phases.size)
        dataToSend[order] = phases
        self.serialConn.write( bytes([254]) ) #start phases
        self.serialConn.write(bytes(dataToSend.astype(np.uint8)))
        self.serialConn.write( bytes([253]) ) #commit

    def switchOnOrOff(self, on):
        self.onOrOff[:] = on
        dataToSend = np.full(self.N_EMMITERS, 0 if on else self.PHASE_DIVS)
        self.serialConn.write( bytes([254]) ) #start phases
        self.serialConn.write(bytes(dataToSend.astype(np.uint8)))
        self.serialConn.write( bytes([253]) ) #commit
        
    def switchTransducerOnOrOff(self, position, on):
        assert position < self.N_EMMITERS
        self.onOrOff[self.EMITTERS_ORDER[position]] = on
        dataToSend = 0 * self.onOrOff + self.PHASE_DIVS * np.logical_not(self.onOrOff)
        self.serialConn.write(bytes([254])) #start phases
        self.serialConn.write(bytes(dataToSend.astype(np.uint8)))
        self.serialConn.write(bytes([253])) #commit


def particle_SHM(ts, midpoint=[0.05, 0.05, 0.14], amp=0.025, axis=0, freq=1):
    r = np.full([len(ts), 3], midpoint)
    r[:, axis] += amp * np.sin(2 * np.pi * freq * ts)
    return r

if __name__ == "__main__":
    surface = SonicSurface()
    # connect to serial port for the SonicSurface
    surface.listSerial()
    port_num = input("Pick serial port: ")
    surface.connect(int(port_num))


    # constants
    time_inc = 0.01 # secs
    # linear SHM
    # freq = 0.5 # secs
    # period = 1 / freq
    # ts = np.linspace(0, period, int(period / time_inc))
    # xs = particle_SHM(ts, freq=freq)

    # circular motion
    start_x = 0.05 # 5cm
    start_y = 0.05 # 5cm
    start_z = 0.14 # 14cm
    freq = 0.5
    period = 1/freq
    spacings = int(period/time_inc)
    ts = np.linspace(0, 1/freq, spacings)
    positions = []
    for t in ts:
        x = start_x + 0.02 * np.sin(2 * np.pi *freq * 2*t)
        y = start_y + 0.02 * np.cos(2 * np.pi *freq * 2*t)
        z = start_z #+ 0.02 * np.sin(2 * np.pi *freq*t)
        positions.append((x, y, z))
    
    
    phase_list = []
    for position in positions:
        # need phases to be from 0 to 2pi
        phases = np.angle(hat.run_hat([position], phase_res=32)) + np.pi
        phase_list.append(phases)

    # send the first position and hold
    phase_one = phase_list[0]
    phases_padded = np.pad(phase_one, [(6, 0), (0, 6)], constant_values=np.NaN)
    surface.sendPhases(phases_padded.flatten())
    
    input("Hit Enter to start")
    
    i = 0
    while True:
        phases = phase_list[i % spacings]
        phases_padded = np.pad(phases, [(6, 0), (0, 6)], constant_values=np.NaN)
        surface.sendPhases(phases_padded.flatten())
        i += 1
        # not exact since sending phases takes time, but close enough
        time.sleep(time_inc)
    
    # turn off array
    # surface.sendPhases(np.full([16, 16], np.NaN).flatten())
