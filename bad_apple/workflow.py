# streamlit app that helps us tune parameters getting positions of a particle such that it would trace the outline of bad apple
# some parameters might be the time delta of the particle, the distance that the particle can travel in some time, the amount of time given to one frame
# in general, the strategy I want to pursue is to find the biggest contour of each frame, split it into points for the particle to follow, and then find the closest point from the next frame's 
# contour for the particle to move to. 
import streamlit
import cv2
from pathlib import Path
import numpy as np

def get_bounding_rect_area(contour):
    rect = cv2.minAreaRect(contour)
    width = rect[1][0]
    height = rect[1][1]
    return width * height


def draw_biggest_external_contour(frame, border_size=60, min_distance=0, canny_low=50, canny_high=150, contour_number=0, dilate_kernel_size=20):
    # Add a white border to the frame, so that a fully black frame will still have an outline
    bordered_frame = cv2.copyMakeBorder(frame, border_size, border_size, border_size, border_size, cv2.BORDER_CONSTANT, value=(255, 255, 255))
    
    # Convert the frame to grayscale
    gray_frame = cv2.cvtColor(bordered_frame, cv2.COLOR_BGR2GRAY)

    kernel = cv2.getStructuringElement(cv2.MORPH_ELLIPSE, (dilate_kernel_size, dilate_kernel_size))
    morphed = cv2.morphologyEx(gray_frame, cv2.MORPH_CLOSE, kernel)
    # dilate and erode to remove noise
    
    # Run Canny edge detection
    edges = cv2.Canny(morphed, canny_low, canny_high)
    
    # Find contours
    contours, _ = cv2.findContours(edges, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_NONE)
    
    # Find the largest n contours
    selected_contour = sorted(contours, key=get_bounding_rect_area,  reverse=True)[contour_number]
    main_contours = [selected_contour]
    # main_contours = sorted(contours, key=cv2.contourArea, reverse=True)[:contour_number]
    # print the contour area    
    # Draw the contour on the frame
    # cv2.drawContours(bordered_frame, [main_contour], -1, (0, 255, 0), 2)
    
    # Extract points along the contour that are farther than min_distance away from each other
    contour_points = []
    for main_contour in main_contours:
        for i in range(len(main_contour)):
            point = main_contour[i][0]
            # if i == 0 or np.linalg.norm(np.array(point) - np.array(contour_points[-1])) > min_distance:
            contour_points.append(tuple(point))
    
    # Draw circles on the frame at the extracted points
    for point in contour_points:
        cv2.circle(bordered_frame, point, 8, (0, 255, 0), -1)
    
    return contour_points, bordered_frame, edges, get_bounding_rect_area(selected_contour), morphed

if __name__ == "__main__":
    streamlit.set_page_config(layout="wide")
    frames_path = Path(__file__).parent.resolve() / 'frames'

    # load the video
    col1, col2 = streamlit.columns(2)
    frame_number = col1.number_input('Frame number', min_value=0, max_value=6572, value=0)
    dilate_kernel_size = col2.number_input('Dilate kernel size', min_value=0, max_value=100, value=0)

    canny_low = col1.slider('Canny low', min_value=0, max_value=255, value=50)
    canny_high = col2.slider('Canny high', min_value=0, max_value=255, value=150)
    contour_number = col1.number_input('Contour number', min_value=0, max_value=5, value=0)
    contour_area_display = col2.empty()
    image1_placeholder = col1.empty()
    image2_placeholder = col2.empty()
    image3_placeholder = col1.empty()
    image4_placeholder = col2.empty()

    img_path = frames_path / f'{frame_number}.png'
    frame = cv2.imread(str(img_path))

    # draw every point of contour onto the image
    points, image_with_contours, edges, contour_area, morphed = draw_biggest_external_contour(frame, canny_low=canny_low, canny_high=canny_high, contour_number=contour_number, dilate_kernel_size=dilate_kernel_size)

    image1_placeholder.image(edges)
    image2_placeholder.image(image_with_contours)
    image3_placeholder.image(morphed)
    contour_area_display.write(f'Contour area: {contour_area}')