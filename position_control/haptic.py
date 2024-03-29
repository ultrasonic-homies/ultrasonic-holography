from PIL import Image, ImageDraw, ImageFont
import cv2
import numpy as np
import redis
import threading
import time
import math

def get_bounding_rect_area(contour):
    # calculate area using the minimum bounding rectangle instead of whatever cv2.contourArea does
    rect = cv2.minAreaRect(contour)
    width = rect[1][0]
    height = rect[1][1]
    return width * height


def draw_biggest_external_contour(frame, 
                                  border_size=0, 
                                  border_color=(255, 255, 255), 
                                  min_distance=80, 
                                  canny_low=50, 
                                  canny_high=150, 
                                  dilate_kernel_size=10):
    # Add a white border to the frame, so that a fully black frame will still have an outline
    frame = cv2.copyMakeBorder(frame, border_size, border_size, border_size, border_size, cv2.BORDER_CONSTANT, value=border_color)
    
    # Convert the frame to grayscale
    gray_frame = cv2.cvtColor(frame, cv2.COLOR_BGR2GRAY)

    kernel = cv2.getStructuringElement(cv2.MORPH_ELLIPSE, (dilate_kernel_size, dilate_kernel_size))
    morphed = cv2.morphologyEx(gray_frame, cv2.MORPH_CLOSE, kernel)
    # dilate and erode to remove noise
    
    # Run Canny edge detection
    edges = cv2.Canny(morphed, canny_low, canny_high)
    
    # Find contours
    contours, _ = cv2.findContours(edges, cv2.RETR_LIST, cv2.CHAIN_APPROX_NONE) # cv2.RETR_EXTERNAL for just external, 
    
    # Find the largest n contours
    if len(contours) == 0:
        return [], frame, 0
    main_contours = []
    # add all contours with area greater than 100
    for contour in contours:
        if get_bounding_rect_area(contour) > 200:
            main_contours.append(contour)
    
    # Extract points along the contour that are farther than min_distance away from each other
    contour_points = []
    for main_contour in main_contours:
        for i in range(len(main_contour)):
            point = main_contour[i][0]
            # if i is 0 or if the the point is far enough from the previous point
            if i == 0 or np.linalg.norm(np.array(point) - np.array(contour_points[-1])) > min_distance:
                contour_points.append(tuple(point))

    # reduce duplicates 
    new_contour_points = []
    for i in range(len(contour_points)):
        if i == 0 or all(np.linalg.norm(np.array(contour_points[i]) - np.array(point)) > min_distance/3 for point in new_contour_points):
            new_contour_points.append(contour_points[i])
    contour_points = new_contour_points

    # Draw circles on the frame at the extracted points

    for point in contour_points:
        cv2.circle(frame, point, 8, (0, 255, 0), -1)

    
    return contour_points, frame

def output_pwm(points, r, frequency, duty_cycle, image_width, image_height, stop_event):
    period = 1 / frequency
    # scale down the points, send them over redis to the positions channel. Send it at the frequency and duty cycle
    # points is a list of tuples, scale it down to 0.1 x 0.1
    # flip y because y = 0 is at the top
    scaled_points = [(point[0] / image_width * 0.1, (image_height - point[1]) / image_height * 0.1) for point in points]
    positions = [list(point) for point in scaled_points]
    positions = [[point[0], point[1], 0.01] for point in positions]
    msg_packed = repr(positions).encode('utf-8')
    # modulate sending at the frequency and duty cycle. On is positions, off is a list with just a corner
    high_position = [[point[0], point[1], 0] for point in positions] # same but higher
    off_packed  = repr(high_position).encode('utf-8')
    while True:
        if stop_event.is_set():
            return
        r.publish('positions', msg_packed)
        time.sleep(period * duty_cycle)
        r.publish('positions', off_packed)
        time.sleep(period * (1 - duty_cycle))

def output_sinusoid(points, r, frequency, amplitude, image_width, image_height, stop_event):
    period = 1 / frequency
    # scale down the points, send them over redis to the positions channel. Send it at the frequency and duty cycle
    # points is a list of tuples, scale it down to 0.1 x 0.1
    # flip y because y = 0 is at the top
    scaled_points = [(point[0] / image_width * 0.1, (image_height - point[1]) / image_height * 0.1) for point in points]
    positions = [list(point) for point in scaled_points]
    spacings = math.floor(100/frequency)
    while True:
        if stop_event.is_set():
            return
        for i in range(spacings):
            if stop_event.is_set():
                return
            positions = [[point[0], point[1], amplitude * np.sin(i * 2 * np.pi / spacings) + amplitude/2] for point in scaled_points]
            msg_packed = repr(positions).encode('utf-8')
            r.publish('positions', msg_packed)
            time.sleep(period / spacings)
    

if __name__ == "__main__":
    window_name = "Haptic Feedback Input, Esc to quit"
    r = redis.StrictRedis(host='localhost', port=6379, db=0)
    font_size = 500  # Increase the font size for a much bigger text
    font_filepath = "Lato-Black.ttf"
    text_color = (0, 0, 0, 255)  # RGBA format, where the last value is alpha (transparency)
    padding = 50  # Padding size in pixels

    # Initialize the font and mask image
    font = ImageFont.truetype(font_filepath, size=font_size)
    text = "A"  # Initial text
    left, top, right, bottom = font.getbbox(text)
    text_width = right - left
    text_height = bottom
    padded_width = text_width + 2 * padding
    padded_height = text_height + 2 * padding
    mask_image = Image.new("RGBA", (padded_width, padded_height), (255, 255, 255, 255))  # Transparent background
    mask_draw = ImageDraw.Draw(mask_image)

    # Calculate the position to center the text horizontally
    text_x = (padded_width - text_width) // 2
    text_y = 0

    mask_draw.text((text_x, text_y), text, font=font, fill=text_color)  # Draw text with padding
    img = np.array(mask_image)
    contour_points, frame = draw_biggest_external_contour(img)

    # Display the initial image using OpenCV
   
    amplitude = 0.04 # 50% duty cycle
    frequency = 4 # hz
    stop_event = threading.Event()

    current_thread = threading.Thread(target=output_sinusoid, args=(contour_points, r, frequency, amplitude, padded_width, padded_height, stop_event), daemon=True)
    current_thread.start()

    last_letter = ord('A')
    cv2.putText(frame, f"Freq: {frequency:.2f} Hz, Amplitude: {amplitude:.2f}", (10, 30), cv2.FONT_HERSHEY_SIMPLEX, 0.5, (0, 0, 0), 1, cv2.LINE_AA)
    cv2.imshow(window_name, frame)
    key = cv2.waitKey(0) & 0xFF

    while key != 27: # the ESC key
        print(key)
        if key == 63232: # up arrow, might be different on other platforms
            # up arrow, increase duty cycle by 10
            amplitude = min(amplitude + 0.005, 0.5) 
            key = last_letter
        elif key == 63233: # down arrow
            # if key is down arrow, decrease duty cycle by 10
            amplitude = max(amplitude - 0.005, 0)
            key = last_letter
        elif key == 63235: # right arrow
            # if key is right arrow, increase frequency by double
            frequency += 1
            key = last_letter
        elif key == 63234: # left arrow
            # if key is left arrow, decrease frequency by half
            frequency -=1
            key = last_letter
        if key != 255 and key != 1:  # default if no key is pressed, also 1 sometimes shows up for the first button?
            text = chr(key)

            # Clear the previous text in the mask image
            mask_draw.rectangle((0, 0, padded_width, padded_height), fill=(255, 255, 255, 255))

            # Calculate the position to center the new text horizontally
            left, top, right, bottom = font.getbbox(text)
            text_width = right - left
            text_height = bottom
            text_x = (padded_width - text_width) // 2
            text_y = 0

            # Draw the new text in the mask image with padding
            mask_draw.text((text_x, text_y), text, font=font, fill=text_color)

            # Convert the updated mask image to a numpy array for OpenCV processing
            img = np.array(mask_image)
            contour_points, frame = draw_biggest_external_contour(img)
            stop_event.set()
            current_thread.join()
            stop_event.clear()
            current_thread = threading.Thread(target=output_sinusoid, args=(contour_points, r, frequency, amplitude, padded_width, padded_height, stop_event), daemon=True)
            current_thread.start()

            # display the frequency and duty cycle
            cv2.putText(frame, f"Freq: {frequency:.2f} Hz, Amplitude: {amplitude:.2f}", (10, 30), cv2.FONT_HERSHEY_SIMPLEX, 0.5, (0, 0, 0), 1, cv2.LINE_AA)

            # Display the updated image using OpenCV
            cv2.imshow(window_name, frame)
            last_letter = key
        key = cv2.waitKeyEx(0)

    cv2.destroyAllWindows()
