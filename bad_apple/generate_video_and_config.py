# this script makes one pass through the bad apple frames and generates a video with the points it finds
# it also saves the points of each frame to its own pickle file (i.pkl) and then saves the config it used for each frame to the conifg.pkl file.
# this is basically used to say "from this index onwards, use these settings"
# the goal is to have a config file at the end with all the correct paramaters to make good a good particle path for the bad apple video
# workflow.py is used to fine tune the parameters of each frame
from pathlib import Path
import cv2
import numpy as np
import pickle

min_distance = 60

def get_bounding_rect_area(contour):
    # calculate area using the minimum bounding rectangle instead of whatever cv2.contourArea does
    rect = cv2.minAreaRect(contour)
    width = rect[1][0]
    height = rect[1][1]
    return width * height


def draw_biggest_external_contour(frame, border_size=60, border_color=(255, 255, 255), min_distance=min_distance, canny_low=50, canny_high=150, contour_number=[0], dilate_kernel_size=20, add_border_after=False, halve=True):
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
    
    # Find the largest n contours
    if len(contours) == 0:
        return [], frame, 0
    main_contours = []
    # add all contours with area greater than 100
    for contour in contours:
        if get_bounding_rect_area(contour) > 200:
            main_contours.append(contour)
    num_contours = len(main_contours)

    # main_contours = sorted(contours, key=cv2.contourArea, reverse=True)[:contour_number]
    # print the contour area    
    # Draw the contour on the frame
    # cv2.drawContours(bordered_frame, [main_contour], -1, (0, 255, 0), 2)
    
    # Extract points along the contour that are farther than min_distance away from each other
    contour_points = []
    for main_contour in main_contours:
        for i in range(len(main_contour)):
            point = main_contour[i][0]
            # if i is 0 or if the the point is far enough from the previous point
            if i == 0 or np.linalg.norm(np.array(point) - np.array(contour_points[-1])) > min_distance:
                contour_points.append(tuple(point))
    if add_border_after:
        frame = cv2.copyMakeBorder(frame, border_size, border_size, border_size, border_size, cv2.BORDER_CONSTANT, value=(255, 255, 255))
        # edit contour_points to account for the border
        contour_points = [(point[0]+border_size, point[1]+border_size) for point in contour_points]

    # reduce duplicates 
    new_contour_points = []
    for i in range(len(contour_points)):
        if i == 0 or all(np.linalg.norm(np.array(contour_points[i]) - np.array(point)) > min_distance/3 for point in new_contour_points):
            new_contour_points.append(contour_points[i])
    contour_points = new_contour_points

    # Draw circles on the frame at the extracted points

    for point in contour_points:
        cv2.circle(frame, point, 8, (0, 255, 0), -1)

    
    return contour_points, frame, num_contours



if __name__ == "__main__":
    start_number = 3777
    import pickle
    frames_path = Path(__file__).parent.resolve() / 'frames'
    points_path = Path(__file__).parent.resolve() / 'points-custom'

    config_file = "config.pkl"

    config = pickle.load(open(config_file, "rb"))
    # video writer

    fourcc = cv2.VideoWriter_fourcc(*'avc1')
    # get one frame size
    frame = cv2.imread(str(frames_path / '0.png'))
    height, width, _ = frame.shape
    out = cv2.VideoWriter(f'badapple_contour_{min_distance}_custom.mp4', fourcc, 30, (width+120, height+120))


    for i in range(start_number, 6572):
        print(i)
        frame = cv2.imread(str(frames_path / f'{i}.png'))
        # if the border of frame is mostly black, make the broder colour black, otherwise white
        # add up every pixel on the borders
        height, width, _ = frame.shape
        top_border = frame[:1, :, :]
        bottom_border = frame[-1:, :, :]
        left_border = frame[:, :1, :]
        right_border = frame[:, -1:, :]
        average_border_color = (np.sum(top_border) + np.sum(bottom_border) + np.sum(left_border) + np.sum(right_border)) / (2 * height + 2* width) / 3 
        if average_border_color < 125:
            border_color = (0, 0, 0)
        else:
            border_color = (255, 255, 255)
        points, bordered_frame, num_contours = draw_biggest_external_contour(frame, border_color=border_color)
        # pickle points to points_path
        with open(points_path / f'{i}.pkl', 'wb') as f:
            pickle.dump(points, f)
        # put i text on the frame 
        if num_contours == 0:
            contour_number = [0]
        else:
            contour_number = list(range(num_contours))
        cv2.putText(bordered_frame, f"Frame {i}", (10, 70), cv2.FONT_HERSHEY_SIMPLEX, 1, (0, 0, 0), 2, cv2.LINE_AA)
        config[i] = {'dilate_kernel_size': 20, 'canny_low': 50, 'canny_high': 150, 'contour_number': contour_number, 'min_distance': min_distance, 'border_color': border_color, 'add_border_after': False, "halve": True}     
        # out.write(bordered_frame)
    
    # save the config
    with open(config_file, 'wb') as file:
        pickle.dump(config, file)

    out.release()
    print(f"Made video: {out.__repr__}")
