# streamlit app to help find the paramaters that make good points that form the contour around the bad apple frames.
# saves the paramaters that make good points for each frame to a pickle file, and the set of points to pickle files in a folder. 
# run streamlit run workflow.py, increment the frame number to move through the frames, and wherever the default is bad, save new paramaters using ctrl + shift + x
import streamlit
import cv2
from pathlib import Path
import numpy as np
import pickle
import os
from streamlit_shortcuts import add_keyboard_shortcuts

global current_points

current_points = []

def get_bounding_rect_area(contour):
    rect = cv2.minAreaRect(contour)
    width = rect[1][0]
    height = rect[1][1]
    return width * height


def draw_biggest_external_contour(frame, border_size=60, border_color=(255, 255, 255), min_distance=40, canny_low=50, canny_high=150, contour_number=[0], dilate_kernel_size=20, add_border_after=False, halve=False, set_global=True):
    # Add a white border to the frame, so that a fully black frame will still have an outline
    if not add_border_after:
        frame = cv2.copyMakeBorder(frame, border_size, border_size, border_size, border_size, cv2.BORDER_CONSTANT, value=border_color)
    
    # Convert the frame to grayscale
    gray_frame = cv2.cvtColor(frame, cv2.COLOR_BGR2GRAY)

    kernel = cv2.getStructuringElement(cv2.MORPH_ELLIPSE, (dilate_kernel_size, dilate_kernel_size))
    morphed = cv2.morphologyEx(gray_frame, cv2.MORPH_CLOSE, kernel)
    # dilate and erode to remove noise
    
    # Run Canny edge detection
    edges = cv2.Canny(morphed, canny_low, canny_high)
    
    # Find contours
    contours, _ = cv2.findContours(edges, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_NONE)
    
    if len(contours) == 0:
        return [], frame, edges, 0, morphed
    # Find the largest n contours
    main_contours = []
    for index in contour_number:
        if index >= len(contours):
            break
        selected_contour = sorted(contours, key=get_bounding_rect_area,  reverse=True)[index]
        main_contours.append(selected_contour)

    # main_contours = sorted(contours, key=cv2.contourArea, reverse=True)[:contour_number]
    # print the contour area    
    # Draw the contour on the frame
    # cv2.drawContours(bordered_frame, [main_contour], -1, (0, 255, 0), 2)
    
    # Extract points along the contour that are farther than min_distance away from each other
    contour_points = []
    for main_contour in main_contours:
        for i in range(len(main_contour)):
            point = main_contour[i][0]
            # if i is 0 or if the the point is far enough from the last point
            if i == 0 or np.linalg.norm(np.array(point) - np.array(contour_points[-1])) > min_distance:
                contour_points.append(tuple(point))
    if add_border_after:
        frame = cv2.copyMakeBorder(frame, border_size, border_size, border_size, border_size, cv2.BORDER_CONSTANT, value=(255, 255, 255))
        # edit contour_points to account for the border
        contour_points = [(point[0]+border_size, point[1]+border_size) for point in contour_points]

    if halve:
        # delete any points that are too close to one another
        new_contour_points = []
        for i in range(len(contour_points)):
            if i == 0 or all(np.linalg.norm(np.array(contour_points[i]) - np.array(point)) > min_distance/3 for point in new_contour_points):
                new_contour_points.append(contour_points[i])
        contour_points = new_contour_points
    if set_global:
        global current_points
        current_points = contour_points
    # Draw circles on the frame at the extracted points

    for point in contour_points:
        cv2.circle(frame, point, 8, (0, 255, 0), -1)

    
    return contour_points, frame, edges, get_bounding_rect_area(selected_contour), morphed

def load_config():
    config_file = "config.pkl"
    return {} if not os.path.exists(config_file) else pickle.load(open(config_file, 'rb'))

if __name__ == "__main__":

    streamlit.set_page_config(layout="wide")
    frames_path = Path(__file__).parent.resolve() / 'frames'
    points_path = Path(__file__).parent.resolve() / 'points-custom'
    config_file = "config.pkl"
    config = load_config()

    def save_callback(frame_number, dilate_kernel_size, canny_low, canny_high, contours, min_distance, border_color, add_border_after, halve):
        #  pickle the global points to the file
        with open(points_path / f"{frame_number}.pkl", 'wb') as file:
            pickle.dump(current_points, file)
        config[frame_number] = {'dilate_kernel_size': dilate_kernel_size, 'canny_low': canny_low, 'canny_high': canny_high, 'contour_number': contours, 'min_distance': min_distance, 'border_color': border_color, 'add_border_after': add_border_after, 'halve': halve}
        # pickle the config to the file
        with open(config_file, 'wb') as file:
            pickle.dump(config, file)

    # load the video
    col1, col2, col3 = streamlit.columns(3)
    frame_number = col1.number_input('Frame number', min_value=0, max_value=6572, value=0)
    halve = col1.checkbox('Remove dupes', value=config[frame_number]['halve'] if frame_number in config else False)
    load_from_file = col1.checkbox('Load defaults from config file', value=False)
    default_contour_number = max(config[frame_number]['contour_number']) if (frame_number in config and load_from_file) else 0
    contour_number = col1.number_input('Contour number', min_value= 0 , max_value = 50, value=default_contour_number)
    contours = [0] if contour_number == 0 else [i for i in range(contour_number+1)]
    dilate_kernel_size = col1.number_input('Dilate kernel size', min_value=0, max_value=100, value=config[frame_number]['dilate_kernel_size'] if frame_number in config else 20)

    canny_low = col1.slider('Canny low', min_value=0, max_value=255, value=config[frame_number]['canny_low'] if frame_number in config else 50)
    canny_high = col1.slider('Canny high', min_value=0, max_value=255, value=config[frame_number]['canny_high'] if frame_number in config else 150)

    min_distance = col1.number_input('Min distance', min_value=0, max_value=100, value=50) # don't use default, carry through
    white_border = col1.checkbox('White border', value=True)
    if white_border:
        border_color = (255, 255, 255)
    else:
        border_color = (0, 0, 0)
    add_border_after = col1.checkbox('Add border after', value=config[frame_number]['add_border_after'] if frame_number in config else False)
    col1.button('Save points and config', on_click=save_callback, key="Save points and config", args=(frame_number, dilate_kernel_size, canny_low, canny_high, contours, min_distance, border_color, add_border_after, halve))


    contour_area_display = col2.empty()
    image1_placeholder = col2.empty()
    image2_placeholder = col3.empty()

    img_path = frames_path / f'{frame_number}.png'
    frame = cv2.imread(str(img_path))



    # draw every point of contour onto the image
    points, image_with_contours, edges, contour_area, morphed = draw_biggest_external_contour(frame.copy(),
                                                                                               min_distance=min_distance, 
                                                                                               border_color=border_color,
                                                                                               canny_low=canny_low, 
                                                                                               canny_high=canny_high, 
                                                                                               contour_number=contours, 
                                                                                               dilate_kernel_size=dilate_kernel_size, 
                                                                                               add_border_after=add_border_after,
                                                                                               halve=halve)


    image1_placeholder.image(image_with_contours)
    with open(f"points-custom/{frame_number}.pkl", "rb") as file:
        points2 = pickle.load(file)
    image2_with_contours = frame.copy()
    for point in points2:
        cv2.circle(image2_with_contours, (point[0]-60, point[1]-60), 8, (0, 255, 0), -1)

    image2_placeholder.image(image2_with_contours)

    contour_area_display.write(f'number of points shown: {len(current_points)}, points on file: {len(points2)}')
    if len(current_points) != len(points2):
        col3.warning("Not same!")
    # add keyboard shortcut to press save button with f4
    streamlit.write(config[frame_number])
    add_keyboard_shortcuts({
        'Ctrl+Shift': 'Save points and config',
    })
