import redis


r = redis.StrictRedis(host='localhost', port=6379, db=0)

positions =  [[0.047000000000000014, 0.04500000000000002, 0.005]]

msg_packed = repr(positions).encode('utf-8')
r.publish("positions", msg_packed)
