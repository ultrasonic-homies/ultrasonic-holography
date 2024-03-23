"""Modified hand.py: when the user pinches, their hand's position shows up on blender. When they release the pinch, that discreet position is sent to the board for the board to interpolate."""
import math
import os
import sys
import time

import cv2
import mediapipe as mp
from mediapipe import solutions
from mediapipe.framework.formats import landmark_pb2
from mediapipe.tasks.python import vision
import redis
from mediapipe.python.solutions.hands import HandLandmark
import numpy as np
from pathlib import Path
# For drawing landmarks on an image, we will use the following function:


MARGIN = 10  # pixels
FONT_SIZE = 1
FONT_THICKNESS = 1
HANDEDNESS_TEXT_COLOR = (88, 205, 54)  # vibrant green

BaseOptions = mp.tasks.BaseOptions
HandLandmarker = mp.tasks.vision.HandLandmarker
HandLandmarkerOptions = mp.tasks.vision.HandLandmarkerOptions
HandLandmarkerResult = mp.tasks.vision.HandLandmarkerResult
VisionRunningMode = mp.tasks.vision.RunningMode
mp_drawing = mp.solutions.drawing_utils
global_img, global_results = None, None
global_y_scale = None
pinching = False
current_position = None
redis_inst = redis.StrictRedis(host='localhost', port=6379, db=0)

model_path = Path(__file__).resolve().parent / "hand_landmarker.task"
if not os.path.exists(model_path):
    print("Go download hand_landmarker_full from "
          "https://developers.google.com/mediapipe/solutions/vision/hand_landmarker/#models")
    sys.exit(1)


def get_board_position_from_hand_positions(x_normalised, y_normalised, pinky_to_index_distance):
    # x position on picture maps to x position on board, y on picture maps to z on board, pinky to index distance
    # scale maps to y on board
    board_x_max = 0.1
    board_y_max = 0.1
    board_z_max = 0.1
    # flip x normalized and y normalized because camera is flipped and y starts at 0 at top
    x_pos = (1-x_normalised) * board_x_max
    global global_y_scale
    # if pinky_to_index_distance is same as global_y_scale we should be at half of board_y_max.
    # Closer(larger ptdi)is larger y,further (smaller ptdi) is smaller y
    y_pos = (pinky_to_index_distance / global_y_scale) * (board_y_max / 2)
    z_pos = (1-y_normalised) * board_z_max
    return x_pos, y_pos, z_pos


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
def mp_callback(result: HandLandmarkerResult, output_image: mp.Image, timestamp_ms: int):
    global global_img, pinching
    # print('hand landmarker result: {}'.format(result))
    # convert image to cv2 format
    image = output_image.numpy_view()
    image = image.copy()  # need to copu to edit
    img = cv2.cvtColor(image, cv2.COLOR_RGB2BGR)
    # add landmarks
    img = draw_landmarks_on_image(img, result)


    if len(result.hand_landmarks) == 0:
        img = cv2.flip(img, 1)
        global_img = img
        return
    handedness = result.handedness
    print(handedness[0][0].display_name)
    h, w, _ = image.shape

    # info:
    # use thumb and index tip distance to see if pinching. If pinching, use centroid of pinch as the z and x axis,
    # and use the scale of the pinky to index finger when the pinch happens as the y axis while pinched
    thumb_tip_x_norm = result.hand_landmarks[0][HandLandmark.THUMB_TIP].x
    thumb_tip_y_norm = result.hand_landmarks[0][HandLandmark.THUMB_TIP].y
    index_tip_x_norm = result.hand_landmarks[0][HandLandmark.INDEX_FINGER_TIP].x
    index_tip_y_norm = result.hand_landmarks[0][HandLandmark.INDEX_FINGER_TIP].y
    index_knuckle_x_norm = result.hand_landmarks[0][HandLandmark.INDEX_FINGER_DIP].x
    index_knuckle_y_norm = result.hand_landmarks[0][HandLandmark.INDEX_FINGER_DIP].y
    pinky_tip_x_norm = result.hand_landmarks[0][HandLandmark.PINKY_TIP].x
    pinky_tip_y_norm = result.hand_landmarks[0][HandLandmark.PINKY_TIP].y

    x_thumb_tip, y_thumb_tip = int(thumb_tip_x_norm * w), int(thumb_tip_y_norm * h)
    x_index_tip, y_index_tip = int(index_tip_x_norm * w), int(index_tip_y_norm * h)
    x_index_knuckle, y_index_knuckle = int(index_knuckle_x_norm * w), int(index_knuckle_y_norm * h)
    x_pinky_tip, y_pinky_tip = int(pinky_tip_x_norm * w), int(pinky_tip_y_norm * h)
    cx, cy = (x_thumb_tip + x_index_tip) // 2, (y_thumb_tip + y_index_tip) // 2
    cv2.circle(img, (x_thumb_tip, y_thumb_tip), 10, (255, 0, 128), cv2.FILLED)
    cv2.circle(img, (x_index_tip, y_index_tip), 10, (255, 0, 128), cv2.FILLED)
    # use index tip to knuckle as a reference
    index_one_knuckle_distance = math.hypot(x_index_knuckle - x_index_tip, y_index_knuckle - y_index_tip)
    pinch_distance = math.hypot(x_index_tip - x_thumb_tip, y_index_tip - y_thumb_tip)

    pinky_to_index_distance = math.hypot(x_index_tip - x_pinky_tip, y_index_tip - y_pinky_tip)
    if pinch_distance > index_one_knuckle_distance * 2:
        if pinching:  # transitioning from not pinching to pinching
            # send to board over different redis topic
            x_pos, y_pos, z_pos = get_board_position_from_hand_positions(cx/w, cy/h, pinky_to_index_distance)
            # send to blender over redis
            position = [x_pos, y_pos, z_pos]
            positions = [position]  # in our system we send lists of positions for multiple trap compatibility
            msg_packed = repr(positions).encode('utf-8')
            redis_inst.publish('board_positions', msg_packed)
        pinching = False
        cv2.line(img, (x_thumb_tip, y_thumb_tip), (x_index_tip, y_index_tip), (255, 0, 128), 3)
        cv2.putText(img, str(int(pinch_distance)), (cx + 30, cy), cv2.FONT_HERSHEY_COMPLEX, 1, (255, 0, 128), 3)
    else:
        if not pinching: # transitioning from pinching to not pinching
            global global_y_scale
            global_y_scale = pinky_to_index_distance
        pinching = True
        cv2.circle(img, (cx, cy), int(pinch_distance), (255, 255, 128), 2)
        x_pos, y_pos, z_pos = get_board_position_from_hand_positions(cx/w, cy/h, pinky_to_index_distance)
        # send to blender over redis
        position = [x_pos, y_pos, z_pos]
        positions = [position]  # in our system we send lists of positions for multiple trap compatibility
        msg_packed = repr(positions).encode('utf-8')
        redis_inst.publish('positions', msg_packed)
        # add coordinates to frame
        cv2.putText(img, f"({x_pos:.2f}, {y_pos:.2f}, {z_pos:.2f})", (10, 30), cv2.FONT_HERSHEY_COMPLEX, 1, (255, 255, 128), 3)
    # flip across y axis so x axis isn't mirrored in a video
    img = cv2.flip(img, 1)
    global_img = img


options = HandLandmarkerOptions(
    base_options=BaseOptions(model_asset_path=model_path),
    running_mode=VisionRunningMode.LIVE_STREAM,
    result_callback=mp_callback,
    num_hands=1)

with HandLandmarker.create_from_options(options) as landmarker:
    cap = cv2.VideoCapture(1)
    position = [0, 0, 0.005]
    positions = [position]  # in our system we send lists of positions for multiple trap compatibility
    msg_packed = repr(positions).encode('utf-8')
    redis_inst.publish('positions', msg_packed)
    while cap.isOpened():
        ret, frame = cap.read()
        time.sleep(0.01)
        if not ret:
            print("Unable to capture video")
            break

        frame_rgb = cv2.cvtColor(frame, cv2.COLOR_BGR2RGB)
        mp_image = mp.Image(image_format=mp.ImageFormat.SRGB, data=frame_rgb)
        # get current timestamp in ms
        timestamp_ms = int(time.time_ns() / 1000000)
        landmarker.detect_async(mp_image, timestamp_ms=timestamp_ms)
        if global_img is not None:
            cv2.imshow('MediaPipe Pose', global_img)
            cv2.waitKey(1)

    cap.release()
    cv2.destroyAllWindows()
