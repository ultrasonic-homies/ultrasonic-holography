import pickle
import redis
import time
r = redis.StrictRedis(host='localhost', port=6379, db=0)

all_points = []

# positions are on 960x720, want to squish them down to 10x10cm. Division factor is 96 * 100, and then normalize by shifting left by 0.05 (5cm)

def is_counter_clockwise(points):
    summed = 0
    for i in range(len(points)):
        point1 = points[i]
        point2 = points[(i + 1) % len(points)]
        summed += (point2[0] - point1[0]) * (point2[1] + point1[1])  #(x2 − x1)(y2 + y1)
    return summed < 0

# actually 960 + 120 = 1080
for i in range(0, 6572):
    print(f"Frame {i}")
    with open(f"points-custom/{i}.pkl", "rb") as file:
        points = pickle.load(file)
        if i == 0:
            blank_points = points
        if len(points) == 0: # deal with the blank frames which I didn't add blank frames to
            points = blank_points
        scaled = [(point[0] / 10800, point[1] / 10800) for point in points]
        positions = [[scaled_pt[0], 0.1 - scaled_pt[1], 0.378/100] for scaled_pt in scaled]
        msg_packed = repr(positions).encode('utf-8')
        r.publish("positions", msg_packed)
        time.sleep(1/30)

    