# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file '.\better_gui\better_gui.ui'
#
# Created by: PyQt5 UI code generator 5.15.9
#
# WARNING: Any manual changes made to this file will be lost when pyuic5 is
# run again.  Do not edit this file unless you know what you are doing.


from PyQt5 import QtCore, QtGui, QtWidgets


class Ui_AcousticLevitationWindow(object):
    def setupUi(self, AcousticLevitationWindow):
        AcousticLevitationWindow.setObjectName("AcousticLevitationWindow")
        AcousticLevitationWindow.resize(1234, 898)
        AcousticLevitationWindow.setAutoFillBackground(False)
        AcousticLevitationWindow.setStyleSheet("background-color: qradialgradient(spread:repeat, cx:0.5, cy:0.5, radius:0.077, fx:0.5, fy:0.5, stop:0 rgba(0, 169, 255, 147), stop:0.497326 rgba(0, 0, 0, 147), stop:1 rgba(0, 169, 255, 147))")
        self.centralwidget = QtWidgets.QWidget(AcousticLevitationWindow)
        self.centralwidget.setObjectName("centralwidget")
        self.horizontalLayout = QtWidgets.QHBoxLayout(self.centralwidget)
        self.horizontalLayout.setContentsMargins(0, 0, 0, 0)
        self.horizontalLayout.setSpacing(3)
        self.horizontalLayout.setObjectName("horizontalLayout")
        self.verticalLayout = QtWidgets.QVBoxLayout()
        self.verticalLayout.setSpacing(0)
        self.verticalLayout.setObjectName("verticalLayout")
        self.master_gui_title = QtWidgets.QLabel(self.centralwidget)
        sizePolicy = QtWidgets.QSizePolicy(QtWidgets.QSizePolicy.Preferred, QtWidgets.QSizePolicy.Preferred)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.master_gui_title.sizePolicy().hasHeightForWidth())
        self.master_gui_title.setSizePolicy(sizePolicy)
        self.master_gui_title.setMaximumSize(QtCore.QSize(400, 127))
        self.master_gui_title.setStyleSheet("")
        self.master_gui_title.setText("")
        self.master_gui_title.setPixmap(QtGui.QPixmap(".\\better_gui\\acoustic-levitation.png"))
        self.master_gui_title.setScaledContents(True)
        self.master_gui_title.setAlignment(QtCore.Qt.AlignCenter)
        self.master_gui_title.setObjectName("master_gui_title")
        self.verticalLayout.addWidget(self.master_gui_title)
        self.madein1week = QtWidgets.QLabel(self.centralwidget)
        font = QtGui.QFont()
        font.setFamily("Comic Sans MS")
        font.setPointSize(15)
        font.setBold(True)
        font.setWeight(75)
        self.madein1week.setFont(font)
        self.madein1week.setStyleSheet("color: white;")
        self.madein1week.setAlignment(QtCore.Qt.AlignCenter)
        self.madein1week.setObjectName("madein1week")
        self.verticalLayout.addWidget(self.madein1week)
        self.label = QtWidgets.QLabel(self.centralwidget)
        font = QtGui.QFont()
        font.setFamily("Comic Sans MS")
        font.setPointSize(15)
        self.label.setFont(font)
        self.label.setLayoutDirection(QtCore.Qt.LeftToRight)
        self.label.setStyleSheet("padding: 10px 0px 5px 85px ;\n"
"color: white;")
        self.label.setObjectName("label")
        self.verticalLayout.addWidget(self.label)
        spacerItem = QtWidgets.QSpacerItem(40, 20, QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Minimum)
        self.verticalLayout.addItem(spacerItem)
        self.pushButton = QtWidgets.QPushButton(self.centralwidget)
        sizePolicy = QtWidgets.QSizePolicy(QtWidgets.QSizePolicy.Minimum, QtWidgets.QSizePolicy.Expanding)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.pushButton.sizePolicy().hasHeightForWidth())
        self.pushButton.setSizePolicy(sizePolicy)
        font = QtGui.QFont()
        font.setFamily("Comic Sans MS")
        font.setPointSize(25)
        self.pushButton.setFont(font)
        self.pushButton.setStyleSheet("QPushButton {\n"
"    background-color: qlineargradient(spread:pad, x1:0, y1:0, x2:0, y2:1, stop:0 rgb(239, 71, 111),     stop:1 rgb(239, 130, 171));\n"
"    border-radius: 20px;\n"
"    border-image: none;\n"
"    border: none;\n"
"    color: black;\n"
"    padding: 5px 20px;\n"
"}\n"
"\n"
"QPushButton:hover {\n"
"    border: 2px solid #C9D6EA;\n"
"}")
        self.pushButton.setObjectName("pushButton")
        self.verticalLayout.addWidget(self.pushButton)
        spacerItem1 = QtWidgets.QSpacerItem(20, 10, QtWidgets.QSizePolicy.Minimum, QtWidgets.QSizePolicy.Minimum)
        self.verticalLayout.addItem(spacerItem1)
        self.circle_mode = QtWidgets.QPushButton(self.centralwidget)
        sizePolicy = QtWidgets.QSizePolicy(QtWidgets.QSizePolicy.Minimum, QtWidgets.QSizePolicy.MinimumExpanding)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.circle_mode.sizePolicy().hasHeightForWidth())
        self.circle_mode.setSizePolicy(sizePolicy)
        font = QtGui.QFont()
        font.setPointSize(25)
        self.circle_mode.setFont(font)
        self.circle_mode.setStyleSheet("QPushButton {\n"
"    background-color: qlineargradient(spread:pad, x1:0, y1:0, x2:0, y2:1, stop:0 rgb(255, 209, 102),     stop:1 rgb(255, 255, 152));\n"
"    border-radius: 20px;\n"
"    border-image: none;\n"
"    border: none;\n"
"    color: black;\n"
"}\n"
"\n"
"QPushButton:hover {\n"
"    border: 2px solid #C9D6EA;\n"
"}")
        self.circle_mode.setObjectName("circle_mode")
        self.verticalLayout.addWidget(self.circle_mode)
        spacerItem2 = QtWidgets.QSpacerItem(20, 10, QtWidgets.QSizePolicy.Minimum, QtWidgets.QSizePolicy.Minimum)
        self.verticalLayout.addItem(spacerItem2)
        self.pushButton_2 = QtWidgets.QPushButton(self.centralwidget)
        sizePolicy = QtWidgets.QSizePolicy(QtWidgets.QSizePolicy.Minimum, QtWidgets.QSizePolicy.Expanding)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.pushButton_2.sizePolicy().hasHeightForWidth())
        self.pushButton_2.setSizePolicy(sizePolicy)
        font = QtGui.QFont()
        font.setFamily("Comic Sans MS")
        font.setPointSize(25)
        self.pushButton_2.setFont(font)
        self.pushButton_2.setStyleSheet("QPushButton {\n"
"    background-color: qlineargradient(spread:pad, x1:0, y1:0, x2:0, y2:1, stop:0 rgb(63, 188, 151),     stop:1 rgb(63, 238, 201));\n"
"    border-radius: 20px;\n"
"    border-image: none;\n"
"    border: none;\n"
"    color: black;\n"
"}\n"
"\n"
"QPushButton:hover {\n"
"    border: 2px solid #C9D6EA;\n"
"}")
        self.pushButton_2.setObjectName("pushButton_2")
        self.verticalLayout.addWidget(self.pushButton_2)
        spacerItem3 = QtWidgets.QSpacerItem(20, 10, QtWidgets.QSizePolicy.Minimum, QtWidgets.QSizePolicy.Minimum)
        self.verticalLayout.addItem(spacerItem3)
        self.pushButton_3 = QtWidgets.QPushButton(self.centralwidget)
        self.pushButton_3.setEnabled(True)
        sizePolicy = QtWidgets.QSizePolicy(QtWidgets.QSizePolicy.Minimum, QtWidgets.QSizePolicy.Expanding)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.pushButton_3.sizePolicy().hasHeightForWidth())
        self.pushButton_3.setSizePolicy(sizePolicy)
        font = QtGui.QFont()
        font.setFamily("Comic Sans MS")
        font.setPointSize(25)
        self.pushButton_3.setFont(font)
        self.pushButton_3.setStyleSheet("QPushButton {\n"
"    background-color: qlineargradient(spread:pad, x1:0, y1:0, x2:0, y2:1, stop:0 rgb(29, 139, 178),     stop:1 rgb(29, 189, 228));\n"
"    border-radius: 20px;\n"
"    border-image: none;\n"
"    border: none;\n"
"    color: black;\n"
"}\n"
"\n"
"QPushButton:hover {\n"
"    border: 2px solid #C9D6EA;\n"
"}")
        self.pushButton_3.setObjectName("pushButton_3")
        self.verticalLayout.addWidget(self.pushButton_3)
        spacerItem4 = QtWidgets.QSpacerItem(20, 10, QtWidgets.QSizePolicy.Minimum, QtWidgets.QSizePolicy.Minimum)
        self.verticalLayout.addItem(spacerItem4)
        self.pushButton_4 = QtWidgets.QPushButton(self.centralwidget)
        sizePolicy = QtWidgets.QSizePolicy(QtWidgets.QSizePolicy.Minimum, QtWidgets.QSizePolicy.Expanding)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.pushButton_4.sizePolicy().hasHeightForWidth())
        self.pushButton_4.setSizePolicy(sizePolicy)
        font = QtGui.QFont()
        font.setFamily("Comic Sans MS")
        font.setPointSize(25)
        self.pushButton_4.setFont(font)
        self.pushButton_4.setStyleSheet("QPushButton {\n"
"    background-color: qlineargradient(spread:pad, x1:0, y1:0, x2:0, y2:1, stop:0 rgb(149, 81, 166),     stop:1 rgb(149, 131, 216));\n"
"    border-radius: 20px;\n"
"    border-image: none;\n"
"    border: none;\n"
"    color: white;\n"
"}\n"
"\n"
"QPushButton:hover {\n"
"    border: 2px solid #C9D6EA;\n"
"}")
        self.pushButton_4.setObjectName("pushButton_4")
        self.verticalLayout.addWidget(self.pushButton_4)
        self.horizontalLayout.addLayout(self.verticalLayout)
        self.line = QtWidgets.QFrame(self.centralwidget)
        self.line.setFrameShape(QtWidgets.QFrame.VLine)
        self.line.setFrameShadow(QtWidgets.QFrame.Sunken)
        self.line.setObjectName("line")
        self.horizontalLayout.addWidget(self.line)
        self.big_label = QtWidgets.QLabel(self.centralwidget)
        sizePolicy = QtWidgets.QSizePolicy(QtWidgets.QSizePolicy.Maximum, QtWidgets.QSizePolicy.Maximum)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.big_label.sizePolicy().hasHeightForWidth())
        self.big_label.setSizePolicy(sizePolicy)
        self.big_label.setMaximumSize(QtCore.QSize(1000, 1000))
        self.big_label.setObjectName("big_label")
        self.horizontalLayout.addWidget(self.big_label)
        self.horizontalLayout.setStretch(0, 2)
        self.horizontalLayout.setStretch(1, 1)
        self.horizontalLayout.setStretch(2, 20)
        AcousticLevitationWindow.setCentralWidget(self.centralwidget)
        self.menubar = QtWidgets.QMenuBar(AcousticLevitationWindow)
        self.menubar.setGeometry(QtCore.QRect(0, 0, 1234, 22))
        self.menubar.setObjectName("menubar")
        AcousticLevitationWindow.setMenuBar(self.menubar)
        self.statusbar = QtWidgets.QStatusBar(AcousticLevitationWindow)
        self.statusbar.setObjectName("statusbar")
        AcousticLevitationWindow.setStatusBar(self.statusbar)

        self.retranslateUi(AcousticLevitationWindow)
        QtCore.QMetaObject.connectSlotsByName(AcousticLevitationWindow)

    def retranslateUi(self, AcousticLevitationWindow):
        _translate = QtCore.QCoreApplication.translate
        AcousticLevitationWindow.setWindowTitle(_translate("AcousticLevitationWindow", "MainWindow"))
        self.madein1week.setText(_translate("AcousticLevitationWindow", "(made in 1 week)"))
        self.label.setText(_translate("AcousticLevitationWindow", "Current Mode: Stopped"))
        self.pushButton.setText(_translate("AcousticLevitationWindow", "Line Path"))
        self.circle_mode.setText(_translate("AcousticLevitationWindow", "Circle Path"))
        self.pushButton_2.setText(_translate("AcousticLevitationWindow", "Random Path"))
        self.pushButton_3.setText(_translate("AcousticLevitationWindow", "Mouse Control"))
        self.pushButton_4.setText(_translate("AcousticLevitationWindow", "STOP"))
        self.big_label.setText(_translate("AcousticLevitationWindow", "TextLabel"))


if __name__ == "__main__":
    import sys
    app = QtWidgets.QApplication(sys.argv)
    AcousticLevitationWindow = QtWidgets.QMainWindow()
    ui = Ui_AcousticLevitationWindow()
    ui.setupUi(AcousticLevitationWindow)
    AcousticLevitationWindow.show()
    sys.exit(app.exec_())
