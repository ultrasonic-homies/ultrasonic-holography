"""
control the position of the particle using the mouse.
"""
import sys
import redis
from PyQt5.QtWidgets import QApplication, QWidget, QLabel
from PyQt5.QtCore import Qt
import time

sonic_surface = False

def get_time():
    return time.time_ns() * 1e-9

class MyWidget(QWidget):
    def __init__(self):
        super().__init__()
        self.setGeometry(100, 100, 800, 800)  # Set window size
        self.setWindowTitle('Mouse Motion Translation')
        self.side_length = 10.0 if sonic_surface else 16.8

        self.mouse_x = 0
        self.mouse_y = 0
        self.mouse_z = 0
        self.board_x = self.side_length / 2  # cm
        self.board_y = self.side_length / 2
        self.board_z = 0.5
        self.tracking = False
        self.setMouseTracking(True)  # Capture mouse movement without clicking
        # add label to show tracking status
        self.label = QLabel(self)
        self.label.move(10, 10)
        # set size to be bigger
        self.label.resize(800, 50)
        self.update_label()
        self.redis = redis.StrictRedis(host='localhost', port=6379, db=0)
        self.redis.publish("positions", repr([[self.board_x/100, self.board_y/100, self.board_z/100]]).encode('utf-8'))  # initial position
        self.last_sent = get_time()
        if sonic_surface:
            self.wait_before_sending = 0.01
        else:
            self.wait_before_sending = 1/10000


    def update_label(self):
        self.label.setText(f"Tracking (T): {'ON' if self.tracking else 'OFF'}, x={self.board_x:.5f}, y={self.board_y:.5f}, z={self.board_z:.5f}")

    def mouseMoveEvent(self, event):
        if not self.tracking:
            return
        # Capture mouse movement event

        # Calculate lateral movement translation (x, y)
        if sonic_surface:
            self.board_x = (event.x()) * (self.side_length / self.width())  # Scale to 0cm-10cm range
        else:
            self.board_x = (self.width() - event.x() )* (self.side_length / self.width())  # Scale to 0cm-10cm range
        self.board_y = (self.height() - event.y()) * (self.side_length / self.height())  # Scale to 0cm-10cm range
        self.update_label()
        self.send_positions()
    
    def send_positions(self):
        # Send board position to Redis
        curr_time = get_time()
        if curr_time - self.last_sent < self.wait_before_sending:
            return
        dist = 0.004
        # diffs = [[0, dist, 0], [0, -dist, 0], [dist, 0, 0], [-dist, 0, 0]]
        # diffs = [[0, 0, dist], [0, 0, -dist], [dist, 0, 0], [-dist, 0, 0], [0, dist, 0], [0, -dist, 0]]
        base_position = [self.board_x/100, self.board_y/100, self.board_z/100]  # flipped for rev 1
        # positions = [list(map(sum, zip(base_position, diff))) for diff in diffs]
        positions = [base_position]
        msg_packed = repr(positions).encode('utf-8')
        self.redis.publish("positions", msg_packed)
        self.last_sent = curr_time


    def mousePressEvent(self, event):
        self.update_label()
    
    # keyboard event, turn tracking on and off with t release
    def keyReleaseEvent(self, event):
        if event.key() == Qt.Key_T:
            self.tracking = not self.tracking
            self.update_label()

    def wheelEvent(self, event):
        sensitivity = 0.001
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