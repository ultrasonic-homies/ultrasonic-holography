import sys
import redis
from PyQt5.QtWidgets import QApplication, QWidget, QLabel
from PyQt5.QtCore import Qt
import time

def get_time():
    return time.time_ns() * 1e-9

class MyWidget(QWidget):
    def __init__(self):
        super().__init__()
        self.setGeometry(100, 100, 800, 800)  # Set window size
        self.setWindowTitle('Mouse Motion Translation')
        self.mouse_x = 0
        self.mouse_y = 0
        self.mouse_z = 0
        self.board_x = 5 # cm
        self.board_y = 5
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
        self.redis.publish("positions", repr([[self.board_x, self.board_y, self.board_z]]).encode('utf-8'))  # initial position
        self.last_sent = get_time()
        self.wait_before_sending = 0.01 # serial port limits us us 100 hz?


    def update_label(self):
        self.label.setText(f"Tracking (T): {'ON' if self.tracking else 'OFF'}, x={self.board_x:.5f}, y={self.board_y:.5f}, z={self.board_z:.5f}")

    def mouseMoveEvent(self, event):
        if not self.tracking:
            return
        # Capture mouse movement event

        # Calculate lateral movement translation (x, y)
        self.board_x = event.x()* (10 / self.width())  # Scale to 0cm-10cm range
        self.board_y = (self.height() - event.y()) * (10 / self.height())  # Scale to 0cm-10cm range
        self.update_label()
        self.send_positions()
    
    def send_positions(self):
        # Send board position to Redis
        curr_time = get_time()
        if curr_time - self.last_sent < self.wait_before_sending:
            return
        positions = [[self.board_x/100, self.board_y/100, self.board_z/100]]
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
        self.board_z = max(0, min(10, self.board_z))
        self.update_label()
        self.send_positions()

if __name__ == '__main__':
    app = QApplication(sys.argv)
    widget = MyWidget()
    widget.show()
    sys.exit(app.exec_())