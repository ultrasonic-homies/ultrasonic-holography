# note: this script isn't run using python, it's run in the scripting window of blender. I just put it here for
# version control, since blender doesn't have history.
import bpy
import numpy as np
import redis
import random
import queue
import time
import threading
import ast

# Clear existing mesh objects
bpy.ops.object.select_all(action='DESELECT')
bpy.ops.object.select_by_type(type='MESH')
bpy.ops.object.delete()

# Define the dimensions of the array
num_rows = 10
num_columns = 10
spacing = 10 / 1000  # Adjust this to control the spacing between cylinders

# Create a cylinder template
bpy.ops.mesh.primitive_cylinder_add(vertices=32, radius=5 / 1000, depth=7 / 1000, location=(0, 0, 0))
cylinder_template = bpy.context.object

# Create the array of cylinders
for row in range(num_rows):
    for col in range(num_columns):
        # Create a new cylinder by duplicating the template
        new_cylinder = cylinder_template.copy()
        bpy.context.collection.objects.link(new_cylinder)
        new_cylinder.location.x = col * spacing
        new_cylinder.location.y = row * spacing

# Select and link the template to the scene
bpy.context.collection.objects.unlink(cylinder_template)
bpy.data.objects.remove(cylinder_template)
bpy.ops.mesh.primitive_uv_sphere_add(radius=0.01, location=(0, 0, 1))
ball = bpy.context.object
ball.name = 'sphere'

# This function can safely be called in another thread.
# The function will be executed when the timer runs the next time.
location_queue = queue.Queue()

r = redis.StrictRedis(host='localhost', port=6379, db=0)
# Create a pubsub instance and subscribe to the 'positions' channel
pubsub = r.pubsub()
pubsub.subscribe('positions')


# context = zmq.Context()
# socket = context.socket(zmq.SUB)
# socket.connect("tcp://localhost:5498")

# Subscribe to all messages
# socket.setsockopt_string(zmq.SUBSCRIBE, '')

def read_to_queue():
    for message in pubsub.listen():
        if message['type'] == 'message':
            position_str = message['data'].decode('utf-8')
            position = ast.literal_eval(position_str)
            location_queue.put((position))


def use_locations_from_queue():
    while not location_queue.empty():
        location = location_queue.get()
        ball.location = location
        bpy.context.view_layer.update()
    return 0.01


bpy.app.timers.register(use_locations_from_queue)
my_thread = threading.Thread(target=read_to_queue)
my_thread.start()
