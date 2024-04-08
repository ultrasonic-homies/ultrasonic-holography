import pickle
import redis
import time
r = redis.StrictRedis(host='localhost', port=6379, db=0)

all_points = []

frame_skip = 4

slow_factor = 4
# positions are on 960x720, want to squish them down to 10x10cm. Division factor is 96 * 100, and then normalize by shifting left by 0.05 (5cm)

def is_counter_clockwise(points):
    summed = 0
    for i in range(len(points)):
        point1 = points[i]
        point2 = points[(i + 1) % len(points)]
        summed += (point2[0] - point1[0]) * (point2[1] + point1[1])  #(x2 − x1)(y2 + y1)
    return summed < 0

# actually 960 + 120 = 1080
time_step = 1/30*frame_skip
for i in range(0, 6573, frame_skip):
    print(f"Frame {i}")
    with open(f"points-custom/{i}.pkl", "rb") as file:
        points = pickle.load(file)
        if i == 0:
            blank_points = points
        if len(points) == 0: # deal with the blank frames which I didn't add blank frames to
            points = blank_points
        if is_counter_clockwise(points):
            points = points[::-1]
        start_time = time.time_ns() * 1e-9
        time_it_should_take = time_step * slow_factor
        time_between_positions = time_it_should_take / len(points)
        for point in points:
            scaled = (point[0] / 10800, point[1] / 10800)
            position = [scaled[0], 0.1-scaled[1], 0.01] # play at 1cm
            positions = [position]  # in our system we send lists of positions for multiple trap compatibility
            msg_packed = repr(positions).encode('utf-8')
            r.publish("positions", msg_packed)
            time.sleep(time_between_positions)
        # if time_delta < 0:
        #     print(f"Frame {i} took {time_delta} seconds to process")
        #     time.sleep(-time_delta)

    