# qt5-tools designer 
# pyuic5 -x .\better_gui\better_gui.ui -o .\better_gui\better_gui.py
# use the ui from better_gui.py
from new_gui import Ui_AcousticLevitationWindow
# import the necessary modules
from PyQt5 import QtWidgets
import sys
from PyQt5.QtGui import QPixmap
from PyQt5.QtCore import QObject, QEvent
# import qapplication
from PyQt5.QtWidgets import QApplication
import cv2
from enum import Enum
import redis
import time
from threading import Thread, Event
from typing import Optional
import random
import numpy as np
import mido
import redis
from pathlib import Path
import json
import ormsgpack
import pyautogui


def note_to_freq(note_number):
    return 440 * (2 ** ((note_number - 69) / 12))

def midi_note_to_name(midi_note):
    notes = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B']
    octave = (midi_note // 12) - 1
    note_name = notes[midi_note % 12]
    return f"{note_name}{octave}"


def print_notes(filename, r, stop_flag: Event):
    mid = mido.MidiFile(filename)

    for msg in mid.play():
        if stop_flag.is_set():
            return
        if msg.type == 'note_on':
            freq = note_to_freq(msg.note)

            # print(f"Note On: Channel={msg.channel}, Note={msg.note}, Velocity={msg.velocity}")
            # print(note_to_freq(msg.note), midi_note_to_name(msg.note))
            # we can't play e 5, so bring anything higher down to below e5
            message = {"type": "m", "command": f"{freq},true"}
            msg_json = ormsgpack.packb(message)
            # msg_json = json.dumps(message)
            # print(msg_json)
            r.publish("commands", msg_json)
        elif msg.type == 'note_off':
            freq = note_to_freq(msg.note)
            message = {"type": "m", "command": f"{freq},false"}
            msg_json = ormsgpack.packb(message)

            # msg_json = json.dumps(message)
            # print(msg_json)
            r.publish("commands", msg_json)


def get_normalized_mouse_position():
    # Get the screen size
    screen_width, screen_height = pyautogui.size()

    # Get the current mouse position
    mouse_x, mouse_y = pyautogui.position()

    # Normalize the mouse position
    normalized_x = mouse_x / screen_width
    normalized_y = mouse_y / screen_height

    return 1 - normalized_x, 1 - normalized_y # flip idk why


class AppState(Enum):
    LINE = 0
    CIRCLE = 1
    RANDOM = 2
    MOUSE = 3
    MEGALOVANIA = 4
    STOPPED = 5

def get_time():
    return time.time_ns() * 1e-9

# make the main application
class AcousticLevitationApp(QtWidgets.QMainWindow):
    def __init__(self):
        super(AcousticLevitationApp, self).__init__()

        # create an instance of the ui
        self.ui = Ui_AcousticLevitationWindow()
        self.ui.setupUi(self)

        # set the title of the window
        self.setWindowTitle("Acoustic Levitation")

        # set the window size
        self.resize(1234, 898)

        self.showMaximized()
        self.show()

        self.wait_before_sending = 1/1000
        self.side_length = 17.8 # cm, board length
        self.mouse_x = self.side_length / 2
        self.mouse_y = self.side_length / 2
        self.mouse_z = 0.5
        self.board_x = self.side_length / 2  # cm
        self.board_y = self.side_length / 2
        self.board_z = 0.5

        self.state = AppState.STOPPED
        self.setMouseTracking(True)

         # circle stuff
        self.circle_radius = 5
        self.circle_frequency = 1

        # line stuff
        self.amplitude = 5
        self.line_freq = 1

        self.redis = redis.StrictRedis(host='localhost', port=6379, db=0)
        self.last_sent = get_time()
        self.thread: Optional[Thread] = None
        self.stop_flag = Event()
        self.update_label()

        self.ui.line_button.clicked.connect(self.line_pressed)
        self.ui.circle_button.clicked.connect(self.circle_pressed)
        self.ui.random_button.clicked.connect(self.random_pressed)
        self.ui.mouse_button.clicked.connect(self.mouse_pressed)
        self.ui.megalovania_button.clicked.connect(self.megalovania_pressed)
        self.ui.stop_button.clicked.connect(self.stop_pressed)
        
    
    def update_label(self):
        # could do string enum but I don't have time
        if self.state == AppState.LINE:
            self.ui.mode_label.setText("Mode: Line")
        elif self.state == AppState.CIRCLE:
            self.ui.mode_label.setText("Mode: Circle")
        elif self.state == AppState.RANDOM:
            self.ui.mode_label.setText("Mode: Random")
        elif self.state == AppState.MOUSE:
            self.ui.mode_label.setText("Mode: Mouse (Click)")
        elif self.state == AppState.MEGALOVANIA:
            self.ui.mode_label.setText("Mode: Megalovania")
        elif self.state == AppState.STOPPED:
            self.ui.mode_label.setText("Mode: Stopped")
        self.ui.madein1week.setText(f"x={self.board_x:.2f}, y={self.board_y:.2f}, z={self.board_z:.2f}")
    
    def line_pressed(self):
        self.state = AppState.LINE
        self.stop_current_thread()
        self.update_label()
        time.sleep(0.5)

        # move to start of line and start sending
        self.move_to(self.side_length/2, self.side_length/2, self.board_z)
        self.stop_flag = Event()

        self.thread = Thread(target=self.line_pattern, args=(self.stop_flag,), daemon=True)
        self.thread.start()

    def circle_pressed(self):
        self.stop_current_thread()
        self.state = AppState.CIRCLE
        self.update_label()

        time.sleep(0.5)

        self.move_to(self.side_length/2 + self.circle_radius, self.side_length/2, self.board_z)
        self.stop_flag = Event()
        self.thread = Thread(target=self.circle_pattern, args=(self.stop_flag,), daemon=True)
        self.thread.start()
    
    def random_pressed(self):
        self.stop_current_thread()
        self.state = AppState.RANDOM
        self.update_label()
        time.sleep(0.5)

        self.stop_flag = Event()
        self.thread = Thread(target=self.random_pattern, args=(self.stop_flag,), daemon=True)
        self.thread.start()
    
    def mouse_pressed(self):
        self.stop_current_thread()
        self.state = AppState.MOUSE
        self.update_label()
        time.sleep(0.5)
        
        self.stop_flag = Event()
        self.thread = Thread(target=self.follow_mouse, args=(self.stop_flag,), daemon=True)
        self.thread.start()
    
    def megalovania_pressed(self):
        print("Starting Megalovania")
        self.stop_current_thread()
        self.state = AppState.MEGALOVANIA
        self.update_label()
            
        midi_file = "/Users/kevinlinxc/src/ultrasonic-holography/music/megalovania.mid" 
        # midi_file = Path(__file__).resolve().parent / 'green-hill-zone.mid'  # Replace 'your_midi_file.mid' with the path to your MIDI file

        # midi_file = Path(__file__).resolve().parent / 'badapple.mid'  # Replace 'your_midi_file.mid' with the path to your MIDI file
        r = redis.Redis(host='localhost', port=6379, db=0, decode_responses=True)
        # turn it off
        self.stop_flag = Event()
        self.thread = Thread(target=print_notes, args=(midi_file, r, self.stop_flag), daemon=True)
        self.thread.start()

    def stop_pressed(self):
        print("Stopping")
        self.stop_current_thread()
        self.state = AppState.STOPPED
        self.update_label()

    
    def stop_current_thread(self):
        self.stop_flag.set()
        if self.thread is not None:
            self.thread.join()

    
    def send_positions(self):
        # Send board position to Redis
        curr_time = get_time()
        base_position = [self.board_x/100, self.board_y/100, self.board_z/100]
        # positions = [list(map(sum, zip(base_position, diff))) for diff in diffs]
        positions = [base_position]
        positions_repr = repr(positions).encode('utf-8')
        self.redis.publish("positions", positions_repr)
        message = {"type": "p", "command": repr(positions)}
        msg_json = ormsgpack.packb(message)
        # msg_json = json.dumps(message)
        self.redis.publish("commands", msg_json)
        self.last_sent = curr_time
    
    def move_to(self, x, y, z):
        # calculate number of steps based on distance
        euclidean_distance = ((self.board_x - x) ** 2 + (self.board_y - y) ** 2 + (self.board_z - z) ** 2) ** 0.5
        # divide by 0.1 to get number of steps
        num_steps = int(euclidean_distance / 0.1) * 10
        # print(num_steps)
        # print(f"Moving from {self.board_x}, {self.board_y}, {self.board_z} to {x}, {y}, {z}")
        i = 0
        while i < num_steps and euclidean_distance > 0.0001:
            # crazy accidental exponential slowing near the end
            self.board_x = self.board_x + i * (x - self.board_x) / num_steps
            self.board_y = self.board_y + i * (y - self.board_y) / num_steps
            self.board_z = self.board_z + i * (z - self.board_z) / num_steps
            euclidean_distance = ((self.board_x - x) ** 2 + (self.board_y - y) ** 2 + (self.board_z - z) ** 2) ** 0.5
            self.send_positions()
            self.update_label()
            # if i % 10 == 0:
            #     print(f"{self.board_x}, {self.board_y}, {self.board_z}")
            #     print(i)
            time.sleep(0.01)
            i += 1

        self.update_label()
    
    
    def circle_pattern(self, stop_flag):
        print("Starting circle pattern")
        while not stop_flag.is_set():
            divisions = 180
            for i in range(divisions):
                if stop_flag.is_set():
                    return
                self.board_x = self.side_length/2 + self.circle_radius * np.cos(i * 2 * np.pi / divisions)
                self.board_y = self.side_length/2 + self.circle_radius * np.sin(i * 2 * np.pi / divisions)
                self.update_label()
                self.send_positions()
                time.sleep(0.01)

    
    def line_pattern(self, stop_flag):
        print("Starting line pattern")
        while not stop_flag.is_set():
            divisions = 100 * 2
            for i in range(divisions):
                if stop_flag.is_set():
                    return
                self.board_x = self.side_length/2 + self.amplitude * np.sin(i * 2 * np.pi / divisions)
                self.board_y = self.side_length/2
                self.update_label()
                self.send_positions()
                time.sleep(0.01)


    def random_pattern(self, stop_flag):
        print("Starting random pattern")
        while True:
            if stop_flag.is_set():
                return
            # print("Top of loop")
            new_x = random.random() * self.side_length
            new_y = random.random() * self.side_length
            # clip x and y at 1 and self.side_length - 1
            new_x = max(2, min(self.side_length - 2, new_x))
            new_y = max(2, min(self.side_length - 2, new_y))
            self.move_to(new_x, new_y, self.board_z)
            time.sleep(0.1)

    def follow_mouse(self, stop_flag):
        print("Starting follow mouse")
        mouse_x, mouse_y = get_normalized_mouse_position()
        # change normalization to -board_length/2 to board_length/2
        initial_x = self.side_length * (mouse_x)
        initial_y = self.side_length * (mouse_y)
        self.move_to(initial_x, initial_y, self.board_z)
        while True:
            if stop_flag.is_set():
                return
            mouse_x, mouse_y = get_normalized_mouse_position()
            # change normalization to -board_length/2 to board_length/2
            self.board_x = self.side_length * (mouse_x)
            self.board_y = self.side_length * (mouse_y)
            self.send_positions()
            self.update_label()


    def ImageUpdateSlot(self, Image):
        self.ui.big_label.setPixmap(QPixmap.fromImage(Image))

    
if __name__ == "__main__":
    app = QtWidgets.QApplication(sys.argv)
    AcousticLevitationWindow = AcousticLevitationApp()
    sys.exit(app.exec_())