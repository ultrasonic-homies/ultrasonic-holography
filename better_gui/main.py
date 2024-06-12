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
import os
import cv2
from enum import Enum
import redis
import time
from threading import Thread, Event
from typing import Optional

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
        self.obs_thread = OBSThread()

        self.obs_thread.start()
        self.obs_thread.ImageUpdate.connect(self.ImageUpdateSlot)
        self.showFullScreen()
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
        self.ui.big_label.setMouseTracking(True)

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
        print("line pressed")
        self.update_label()

    def circle_pressed(self):
        print("circle pressed")
        self.state = AppState.CIRCLE
        self.update_label()
    
    def random_pressed(self):
        self.state = AppState.RANDOM
        self.update_label()
    
    def mouse_pressed(self):
        self.state = AppState.MOUSE
        self.update_label()
    
    def megalovania_pressed(self):
        self.state = AppState.MEGALOVANIA
        self.update_label()
    
    def stop_pressed(self):
        self.state = AppState.STOPPED
        self.update_label()

    
    


    def ImageUpdateSlot(self, Image):
        self.ui.big_label.setPixmap(QPixmap.fromImage(Image))



class OBSThread(QThread):
    """
    Thread to pull from obs virtual camera and display in the GUI. Note that OBS virtual cam forces the image to have
    16/9 aspect ratio, so we just have to cope by putting the thing of interest on the left and cropping to a square with opencv.
    """
    ImageUpdate = pyqtSignal(QImage)
    def run(self):
        self.ThreadActive = True
        for i in range(0, 10):
            cap = cv2.VideoCapture(i)
            if cap.isOpened():
                print()
                break
        while self.ThreadActive:
            ret, frame = cap.read()
            if frame is None:
                raise Exception("No secondary/virtual camera found, is OBS running?")
            if ret:
                img = cv2.cvtColor(frame, cv2.COLOR_BGR2RGB)
                height = img.shape[0]

                # crop to square with dimensions heightxheight
                img = img[:, :height, :].copy()
                pic = QImage(img.data, img.shape[1], img.shape[0], QImage.Format_RGB888)
                self.ImageUpdate.emit(pic)


    def stop(self):
        self.ThreadActive = False
        self.quit()
    
if __name__ == "__main__":
    app = QtWidgets.QApplication(sys.argv)
    AcousticLevitationWindow = AcousticLevitationApp()
    sys.exit(app.exec_())