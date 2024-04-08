import pickle
import redis
import time
import cv2

# make a video overlaying the points on badapple.mp4 frames so I can see if the points are good. Also puttext of the frame number so I can go fix it

cap = cv2.VideoCapture("badapple.mp4")
frame_width = int(cap.get(3))
frame_height = int(cap.get(4))
frame_rate = cap.get(5)
frame_count = int(cap.get(7))
print(frame_width, frame_height, frame_rate, frame_count)
video_maker = cv2.VideoWriter("badapple_pickle.mp4", cv2.VideoWriter_fourcc(*'mp4v'), frame_rate, (frame_width, frame_height))
r = redis.StrictRedis(host='localhost', port=6379, db=0)

all_points = []

# positions are on 960x720, want to squish them down to 10x10cm. Division factor is 96 * 100, and then normalize by shifting left by 0.05 (5cm)

index = 0
ret, frame = cap.read()
# actually 960 + 120 = 1080
while ret:
    print(f"Frame {index}")

    with open(f"points-custom/{index}.pkl", "rb") as file:
        points = pickle.load(file)
        if index == 0:
            blank_points = points
        if len(points) == 0: # deal with the blank frames which I didn't add blank frames to
            points = blank_points
    # subtract offset from points
    points = [(point[0] - 60, point[1]-60) for point in points]
    num_points = len(points)
    for ind, point in enumerate(points):
        cv2.circle(frame, point, 8, (255-int(255*ind/num_points), int(255*ind/num_points), 0), -1)
    cv2.putText(frame, f"Frame {index}", (10, 50), cv2.FONT_HERSHEY_SIMPLEX, 1, (255, 255, 255), 2, cv2.LINE_AA)
    video_maker.write(frame)
    ret, frame = cap.read()
    index += 1

    