"""
Control position of bead using keyboard control. Strictly worse than the
mouse control, but let's you only send 1 position at a time which is useful for debugging.
"""
from PyQt5.QtWidgets import QApplication, QWidget, QLabel
from PyQt5.QtCore import Qt
import sys
import redis
import time

sonic_surface = False

def get_time():
    return time.time_ns() * 1e-9

class MyWidget(QWidget):
    def __init__(self):
        super().__init__()
        self.setGeometry(100, 100, 800, 800)  # Set window size
        self.setWindowTitle('Arrow Key Motion Translation')
        self.side_length = 10.0 if sonic_surface else 16.8
        self.mouse_x = 0
        self.mouse_y = 0
        self.mouse_z = 0
        # start at center
        self.board_x = self.side_length / 2  # cm
        self.board_y = self.side_length / 2
        self.board_z = 0.5
        self.tracking = False
        # add label to show tracking status
        self.label = QLabel(self)
        self.label.move(10, 10)
        # set size to be bigger
        self.label.resize(800, 50)
        self.update_label()
        self.redis = redis.StrictRedis(host='localhost', port=6379, db=0)
        self.redis.publish("positions", repr([[self.board_x / 100, self.board_y / 100, self.board_z / 100]]).encode('utf-8'))  # initial position
        self.last_sent = get_time()
        if sonic_surface:
            self.wait_before_sending = 0.01  # serial port limits us to 100 Hz?
        self.num_sends = 0

    def update_label(self):
        self.label.setText(f"Tracking (T): {'ON' if self.tracking else 'OFF'}, x={self.board_x:.5f}, y={self.board_y:.5f}, z={self.board_z:.5f}")

    def send_positions(self):
        # Send board position to Redis
        curr_time = get_time()
        if sonic_surface: # skip sending if we're on the sonic surface and we're sending too fast
            if curr_time - self.last_sent < self.wait_before_sending:
                return
        dist = 0.001
        diffs = [[0, dist, 0], [0, -dist, 0], [dist, 0, 0], [-dist, 0, 0]]
        base_position = [self.board_x / 100, self.board_y / 100, self.board_z / 100]
        # positions = [list(map(sum, zip(base_position, diff))) for diff in diffs]
        positions = [base_position]
        print(f"Sending msg {self.num_sends} {positions}")
        self.num_sends += 1

        msg_packed = repr(positions).encode('utf-8')
        self.redis.publish("positions", msg_packed)
        self.last_sent = curr_time

    # keyboard event, turn tracking on and off with T key
    def keyPressEvent(self, event):
        key = event.key()
        if key == Qt.Key_Up:
            self.board_y += 0.1
        elif key == Qt.Key_Down:
            self.board_y -= 0.1
        elif key == Qt.Key_Left:
            self.board_x -= 0.1
        elif key == Qt.Key_Right:
            self.board_x += 0.1
        elif key == Qt.Key_T:
            self.tracking = not self.tracking
        self.board_x = max(0, min(self.side_length, self.board_x))  # Ensure x coordinate stays within 0cm-10cm range
        self.board_y = max(0, min(self.side_length, self.board_y))  # Ensure y coordinate stays within 0cm-10cm range
        self.update_label()
        self.send_positions()

if __name__ == '__main__':
    app = QApplication(sys.argv)
    widget = MyWidget()
    widget.show()
    sys.exit(app.exec_())
