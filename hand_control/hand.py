import cv2

import mediapipe as mp
from mediapipe import solutions
import math
# For drawing landmarks on an image, we will use the following function:

import os
import sys
import time
from mediapipe.framework.formats import landmark_pb2
import numpy as np
from mediapipe.tasks.python import vision

MARGIN = 10  # pixels
FONT_SIZE = 1
FONT_THICKNESS = 1
HANDEDNESS_TEXT_COLOR = (88, 205, 54) # vibrant green

BaseOptions = mp.tasks.BaseOptions
GestureRecognizer = mp.tasks.vision.GestureRecognizer
GestureRecognizerOptions = mp.tasks.vision.GestureRecognizerOptions
GestureRecognizerResult = mp.tasks.vision.GestureRecognizerResult
VisionRunningMode = mp.tasks.vision.RunningMode
mp_drawing = mp.solutions.drawing_utils
global_img, global_results = None, None

model_path = "gesture_recognizer.task"
if not os.path.exists(model_path):
    print("Go download pose_landmarker_full from "
          "https://developers.google.com/mediapipe/solutions/vision/gesture_recognizer#models")
    sys.exit(1)


def draw_landmarks_on_image(rgb_image, detection_result):
  hand_landmarks_list = detection_result.hand_landmarks
  handedness_list = detection_result.handedness
  annotated_image = np.copy(rgb_image)

  # Loop through the detected hands to visualize.
  for idx in range(len(hand_landmarks_list)):
    hand_landmarks = hand_landmarks_list[idx]
    handedness = handedness_list[idx]

    # Draw the hand landmarks.
    hand_landmarks_proto = landmark_pb2.NormalizedLandmarkList()
    hand_landmarks_proto.landmark.extend([
      landmark_pb2.NormalizedLandmark(x=landmark.x, y=landmark.y, z=landmark.z) for landmark in hand_landmarks
    ])
    solutions.drawing_utils.draw_landmarks(
      annotated_image,
      hand_landmarks_proto,
      solutions.hands.HAND_CONNECTIONS,
      solutions.drawing_styles.get_default_hand_landmarks_style(),
      solutions.drawing_styles.get_default_hand_connections_style())

    # Get the top left corner of the detected hand's bounding box.
    height, width, _ = annotated_image.shape
    x_coordinates = [landmark.x for landmark in hand_landmarks]
    y_coordinates = [landmark.y for landmark in hand_landmarks]
    text_x = int(min(x_coordinates) * width)
    text_y = int(min(y_coordinates) * height) - MARGIN

    # Draw handedness (left or right hand) on the image.
    cv2.putText(annotated_image, f"{handedness[0].category_name}",
                (text_x, text_y), cv2.FONT_HERSHEY_DUPLEX,
                FONT_SIZE, HANDEDNESS_TEXT_COLOR, FONT_THICKNESS, cv2.LINE_AA)

  return annotated_image


# Create a pose landmarker instance with the live stream mode:
def print_result(result: GestureRecognizerResult, output_image: mp.Image, timestamp_ms: int):
    global global_img
    # print('hand landmarker result: {}'.format(result))
    # convert image to cv2 format
    image = output_image.numpy_view()
    image = image.copy()
    # make image from numpy array
    lml = []
    xl = []
    yl = []
    if len(result.hand_landmarks) == 0:
        global_img = cv2.cvtColor(image, cv2.COLOR_RGB2BGR)
        return
    for id, lm in enumerate(result.hand_landmarks[0]):
        h, w, _ = image.shape
        xc, yc = int(lm.x * w), int(lm.y * h)
        lml.append([id, xc, yc])
        xl.append(xc)
        yl.append(yc)
    global_img = cv2.cvtColor(global_img, cv2.COLOR_RGB2BGR)
    x1, y1 = lml[4][1], lml[4][2]
    x2, y2 = lml[8][1], lml[8][2]
    cx, cy = (x1 + x2) // 2, (y1 + y2) // 2
    cv2.circle(image, (x1, y1), 10, (255, 0, 128), cv2.FILLED)
    cv2.circle(image, (x2, y2), 10, (255, 0, 128), cv2.FILLED)
    cv2.line(image, (x1, y1), (x2, y2), (255, 0, 128), 3)
    distance = math.hypot(x2 - x1, y2 - y1)
    img = draw_landmarks_on_image(image, result)
    cv2.putText(img, str(int(distance)), (cx + 30, cy), cv2.FONT_HERSHEY_COMPLEX, 1, (255, 0, 128), 3)
    global_img = img


options = GestureRecognizerOptions(
    base_options=BaseOptions(model_asset_path=model_path),
    running_mode=VisionRunningMode.LIVE_STREAM,
    result_callback=print_result,
    num_hands=2)

with vision.GestureRecognizer.create_from_options(options) as landmarker:
    cap = cv2.VideoCapture(1)
    while cap.isOpened():
        ret, frame = cap.read()
        if not ret:
            print("Unable to capture video")
            break

        frame_rgb = cv2.cvtColor(frame, cv2.COLOR_BGR2RGB)
        mp_image = mp.Image(image_format=mp.ImageFormat.SRGB, data=frame_rgb)
        # get current timestamp in ms
        timestamp_ms = int(time.time_ns() / 1000000)
        landmarker.recognize_async(mp_image, timestamp_ms=timestamp_ms)
        if global_img is not None:
            cv2.imshow('MediaPipe Pose', global_img)
            cv2.waitKey(1)


    cap.release()
    cv2.destroyAllWindows()
