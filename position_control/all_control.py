"""
control the position of the particle using the mouse.
"""
import sys
import redis
from PyQt5.QtWidgets import QApplication, QWidget, QLabel
from PyQt5.QtCore import Qt
import time
from enum import Enum
from threading import Thread, Event
from typing import Optional
import numpy as np
import random

sonic_surface = False
def get_time():
    return time.time_ns() * 1e-9

class AppState(Enum):
    MOUSE = 0
    CIRCLE = 1
    LINE = 2
    RANDOM = 3

class MyWidget(QWidget):
    def __init__(self):
        super().__init__()
        if sonic_surface:
            self.wait_before_sending = 0.01
        else:
            self.wait_before_sending = 1/1000

        self.setGeometry(100, 100, 800, 800)  # Set window size
        self.setWindowTitle('Mouse Motion Translation')
        self.side_length = 10.0 if sonic_surface else 17.8
        self.mouse_x = self.side_length / 2
        self.mouse_y = self.side_length / 2
        self.mouse_z = 0.5
        self.board_x = self.side_length / 2  # cm
        self.board_y = self.side_length / 2
        self.board_z = 0.5

        # moue stuff
        self.state = AppState.MOUSE
        self.tracking = False
        self.setMouseTracking(True)  # Capture mouse movement without clicking

        # circle stuff
        self.circle_radius = 5 if not sonic_surface else 2.5
        self.circle_frequency = 1

        # line stuff
        self.amplitude = 5 if not sonic_surface else 2.5
        self.line_freq = 1

        # add label to show tracking status
        self.mode_label = QLabel(self)
        self.mode_label.move(10, 10)
        self.position_label = QLabel(self)
        self.position_label.move(10, 30)
        # set size to be bigger
        self.mode_label.resize(800, 50)
        self.position_label.resize(800, 50)
        self.update_label()
        self.redis = redis.StrictRedis(host='localhost', port=6379, db=0)
        self.last_sent = get_time()

        self.send_positions()
        self.thread: Optional[Thread] = None
        self.stop_flag = Event()




    def update_label(self):
        if self.state == AppState.MOUSE:
            self.mode_label.setText(f"Mode (>M/C/L/R) Tracking (T): {'ON' if self.tracking else 'OFF'}")
            self.position_label.setText(f"x={self.board_x:.2f}, y={self.board_y:.2f}, z={self.board_z:.2f}")
        elif self.state == AppState.CIRCLE:
            self.mode_label.setText(f"Mode (M/>C/L/R)")
            self.position_label.setText(f"x={self.board_x:.2f}, y={self.board_y:.2f}, z={self.board_z:.2f}")
        elif self.state == AppState.LINE:
            self.mode_label.setText(f"Mode (M/C/>L/R)")
            self.position_label.setText(f"x={self.board_x:.2f}, y={self.board_y:.2f}, z={self.board_z:.2f}")
        elif self.state == AppState.RANDOM:
            self.mode_label.setText(f"Mode (M/C/L/>R)")
            self.position_label.setText(f"x={self.board_x:.2f}, y={self.board_y:.2f}, z={self.board_z:.2f}")


    def mouseMoveEvent(self, event):
        # save the mouse position for when changing back from another mode
        if sonic_surface:
            event_x = (event.x()) * (self.side_length / self.width())  # Scale to 0cm-10cm range
        else:
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
        if sonic_surface:
            self.board_x = event_x  # Scale to 0cm-10cm range
        else:
            self.board_x = event_x  # Scale to 0cm-10cm range
        self.board_y = event_y # Scale to 0cm-10cm range

        self.update_label()
        self.send_positions()
    
    def send_positions(self):
        # Send board position to Redis
        curr_time = get_time()
        if sonic_surface:
            if curr_time - self.last_sent < self.wait_before_sending:
                return
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
    
    # keyboard event, turn tracking on and off with t release
    def keyReleaseEvent(self, event):
        self.stop_flag.set()
        if self.thread is not None:
            self.thread.join()
        if event.key() == Qt.Key_T and self.state == AppState.MOUSE:
            if not self.tracking:
                self.move_to(self.mouse_x, self.mouse_y, self.mouse_z)
            self.tracking = not self.tracking
            self.update_label()
        elif event.key() == Qt.Key_M:
            self.state = AppState.MOUSE
            self.update_label()
            time.sleep(0.5)
            self.move_to(self.mouse_x, self.mouse_y, self.mouse_z)

        elif event.key() == Qt.Key_C:
            self.state = AppState.CIRCLE
            self.update_label()
            time.sleep(0.5)

            # move to circle start
            self.move_to(self.side_length/2 + self.circle_radius, self.side_length/2, self.board_z)
            self.stop_flag = Event()
            self.thread = Thread(target=self.circle_pattern, args=(self.stop_flag,), daemon=True)
            self.thread.start()

        elif event.key() == Qt.Key_L:
            self.state = AppState.LINE
            self.update_label()
            time.sleep(0.5)


            # move to start of line and start sending
            self.move_to(self.side_length/2, self.side_length/2, self.board_z)
            self.stop_flag = Event()

            self.thread = Thread(target=self.line_pattern, args=(self.stop_flag,), daemon=True)
            self.thread.start()
        
        elif event.key() == Qt.Key_R:
            self.state = AppState.RANDOM
            self.update_label()
            time.sleep(0.5)
            self.stop_flag = Event()
            self.thread = Thread(target=self.random_pattern, args=(self.stop_flag,), daemon=True)
            self.thread.start()

    

    def wheelEvent(self, event):
        # always allow scroll so we can move circle or line higher
        sensitivity = 0.004
        # Capture mouse wheel (scrolling) event
        self.board_z += event.angleDelta().y() * sensitivity   # Assuming each step corresponds to 1 unit of z movement
        # Ensure z coordinate stays within 0cm-10cm range
        self.board_z = max(0, min(13, self.board_z))
        self.update_label()
        self.send_positions()

if __name__ == '__main__':
    app = QApplication(sys.argv)
    widget = MyWidget()
    widget.show()
    sys.exit(app.exec_())