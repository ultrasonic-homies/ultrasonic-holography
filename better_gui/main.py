# qt5-tools designer 
# pyuic5 -x .\better_gui\better_gui.ui -o .\better_gui\better_gui.py
# use the ui from better_gui.py
from new_gui import Ui_AcousticLevitationWindow
# import the necessary modules
from PyQt5 import QtWidgets
import sys
from PyQt5.QtCore import QThread
from PyQt5.QtGui import QImage
from PyQt5.QtCore import Qt, pyqtSignal
from PyQt5.QtGui import QPixmap
import cv2
from enum import Enum
import redis
import time
from threading import Thread, Event
from typing import Optional
import random
import numpy as np

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
        self.tracking = False
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
            self.ui.mode_label.setText("Mode: Mouse")
        elif self.state == AppState.MEGALOVANIA:
            self.ui.mode_label.setText("Mode: Megalovania")
        elif self.state == AppState.STOPPED:
            self.ui.mode_label.setText("Mode: Stopped")
    
    def line_pressed(self):
        self.state = AppState.LINE
        self.stop_current_thread()
        self.update_label()
        # move to start of line and start sending
        self.move_to(self.side_length/2, self.side_length/2, self.board_z)
        self.stop_flag = Event()

        self.thread = Thread(target=self.line_pattern, args=(self.stop_flag,), daemon=True)
        self.thread.start()

    def circle_pressed(self):
        self.stop_current_thread()
        self.state = AppState.CIRCLE
        self.update_label()
        self.move_to(self.side_length/2 + self.circle_radius, self.side_length/2, self.board_z)
        self.stop_flag = Event()
        self.thread = Thread(target=self.circle_pattern, args=(self.stop_flag,), daemon=True)
        self.thread.start()
    
    def random_pressed(self):
        self.stop_current_thread()
        self.state = AppState.RANDOM
        self.update_label()
        self.stop_flag = Event()
        self.thread = Thread(target=self.random_pattern, args=(self.stop_flag,), daemon=True)
        self.thread.start()
    
    def mouse_pressed(self):
        self.stop_current_thread()
        self.state = AppState.MOUSE
        self.update_label()
        time.sleep(0.5)
        self.move_to(self.mouse_x, self.mouse_y, self.mouse_z)
    
    def megalovania_pressed(self):
        print("Starting Megalovania")
        self.stop_current_thread()
        self.state = AppState.MEGALOVANIA
        self.update_label()
    
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
        msg_packed = repr(positions).encode('utf-8')
        self.redis.publish("positions", msg_packed)
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
    
    def keyReleaseEvent(self, event):
        if event.key() == Qt.Key_T and self.state == AppState.MOUSE:
            if not self.tracking:
                # we were doing something else, move to the mouse position so we can start moving mouse
                self.move_to(self.mouse_x, self.mouse_y, self.mouse_z)
            self.tracking = not self.tracking


    def ImageUpdateSlot(self, Image):
        self.ui.big_label.setPixmap(QPixmap.fromImage(Image))
    
    def mouseMoveEvent(self, event):
        # save the mouse position for when changing back from another mode
        event_x = (self.width() - event.x() )* (self.side_length / self.width())
        event_y = (self.height() - event.y()) * (self.side_length / self.height())  # Scale to 0cm-10cm range

        self.mouse_x = event_x
        self.mouse_y = event_y
        self.mouse_z = self.board_z
        if not (self.state == AppState.MOUSE and self.tracking):
            return
        # Capture mouse movement event

        # Calculate lateral movement translation (x, y)
                # Calculate lateral movement translation (x, y)
        self.board_x = event_x  # Scale to 0cm-10cm range
        self.board_y = event_y # Scale to 0cm-10cm range

        self.send_positions()



    def stop(self):
        self.ThreadActive = False
        self.quit()
    
if __name__ == "__main__":
    app = QtWidgets.QApplication(sys.argv)
    AcousticLevitationWindow = AcousticLevitationApp()
    sys.exit(app.exec_())