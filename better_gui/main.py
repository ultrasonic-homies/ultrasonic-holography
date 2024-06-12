# qt5-tools designer 
# pyuic5 -x .\better_gui\better_gui.ui -o .\better_gui\better_gui.py
# use the ui from better_gui.py
from better_gui import Ui_AcousticLevitationWindow
# import the necessary modules
from PyQt5 import QtWidgets
import sys
from PyQt5.QtCore import QThread
from PyQt5.QtGui import QImage
from PyQt5.QtCore import Qt, pyqtSignal
from PyQt5.QtGui import QPixmap
import os
import cv2



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
        self.show()

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
        for i in range(1, 10):
            cap = cv2.VideoCapture(i)
            if cap.isOpened():
                print()
                break
        while self.ThreadActive:
            ret, frame = cap.read()
            if ret:
                width = 
                img = cv2.cvtColor(frame, cv2.COLOR_BGR2RGB)
                print(img.shape)
                ConvertToQtFormat = QImage(img.data, img.shape[1], img.shape[0], QImage.Format_RGB888)
                Pic = ConvertToQtFormat.scaled(1000, 1000, Qt.KeepAspectRatio)
                self.ImageUpdate.emit(Pic)
    def stop(self):
        self.ThreadActive = False
        self.quit()
    
if __name__ == "__main__":
    app = QtWidgets.QApplication(sys.argv)
    AcousticLevitationWindow = AcousticLevitationApp()
    sys.exit(app.exec_())