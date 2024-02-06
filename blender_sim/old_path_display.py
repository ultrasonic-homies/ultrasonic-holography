# old blender script that keyframes between a list of locations.
# Probably scrapped, using blender as a live display instead
import bpy
import numpy as np

# Clear existing mesh objects
bpy.ops.object.select_all(action='DESELECT')
bpy.ops.object.select_by_type(type='MESH')
bpy.ops.object.delete()

# Define the dimensions of the array
num_rows = 10
num_columns = 10
spacing = 10/1000  # Adjust this to control the spacing between cylinders

# Create a cylinder template
bpy.ops.mesh.primitive_cylinder_add(vertices=32, radius=5/1000, depth=7/1000, location=(0, 0, 0))
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


# Set the start and end frames for the animation
start_frame = 1
end_frame = 400  # Adjust the end frame as needed

# Create a new sphere (the ball)
bpy.ops.mesh.primitive_uv_sphere_add(radius=0.005, location=(0, 0, 0))

# Select the ball object
ball = bpy.context.object
ball.name = 'sphere'

################################
# create a list of (x, y, z) tuples (in meters) here to animate them
time_inc = 0.01 # secs
start_x = 0.05 # 5cm
start_y = 0.05 # 5cm
start_z = 0.14 # 14cm
freq = 0.5
period = 1/freq
spacings = int(period/time_inc)
ts = np.linspace(0, 1/freq, spacings)
positions = []
for t in ts:
    x = start_x + 0.02 * np.sin(2 * np.pi *freq * 2*t)
    y = start_y + 0.02 * np.cos(2 * np.pi *freq * 2*t)
    z = start_z #+ 0.02 * np.sin(2 * np.pi *freq*t)
    positions.append((x, y, z))
##################################

n_positions = len(positions)

# create position keyframes
for index, position in enumerate(positions):
    ball.location = position
    ball.keyframe_insert(data_path="location", frame=start_frame + index/n_positions * (end_frame-start_frame))

# Set the interpolation mode for the keyframes
for fc in ball.animation_data.action.fcurves:
    if fc.data_path == "location":
        for keyframe in fc.keyframe_points:
            keyframe.interpolation = 'BEZIER'