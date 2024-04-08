import json
from pathlib import Path
import time
import redis

json_path = Path(__file__).resolve().parent / "bad-apple-tsp.json"

r = redis.StrictRedis(host='localhost', port=6379, db=0)
with open(json_path, "r") as file:
    points_dict = json.load(file)
start_time = time.time_ns() * 1e-9
original_width = 960
original_height = 720
for index in range(6572):
    current_time = time.time_ns() * 1e-9
    diff = current_time - start_time
    print(f"Frame {index}, time {diff}")
    for point in points_dict[str(index)]:
        scaled = [point[0] *0.10 / original_width, point[1] * 0.10 / original_height, 0.01]
        positions = [scaled]
        msg_packed = repr(positions).encode('utf-8')
        r.publish("positions", msg_packed)
        time.sleep(0.01)

