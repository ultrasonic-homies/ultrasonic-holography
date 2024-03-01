from pathlib import Path
import cv2


def get_bounding_rect_area(contour):
    # calculate area using the minimum bounding rectangle instead of whatever cv2.contourArea does
    rect = cv2.minAreaRect(contour)
    width = rect[1][0]
    height = rect[1][1]
    return width * height


def draw_biggest_external_contour(frame, border_size=60, min_distance=0, canny_low=50, canny_high=150, contour_number=0, dilate_kernel_size=50):
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
    if len(contours) == 0:
        # white frame
        return [], bordered_frame
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
    
    return contour_points, bordered_frame

frames_path = Path(__file__).parent.resolve() / 'frames'

# video writer

fourcc = cv2.VideoWriter_fourcc(*'avc1')
# get one frame size
frame = cv2.imread(str(frames_path / '0.png'))
height, width, _ = frame.shape
out = cv2.VideoWriter('badapple_contours.mp4', fourcc, 30, (width+120, height+120))


for i in range(6572):
    print(i)
    frame = cv2.imread(str(frames_path / f'{i}.png'))
    points, bordered_frame = draw_biggest_external_contour(frame)
    # put i text on the frame 
    cv2.putText(bordered_frame, f"Frame {i}", (10, 70), cv2.FONT_HERSHEY_SIMPLEX, 1, (0, 0, 0), 2, cv2.LINE_AA)
    out.write(bordered_frame)

out.release()
print(f"Made video: {out.__repr__}")

    
