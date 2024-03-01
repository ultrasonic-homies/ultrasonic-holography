import cv2
from pathlib import Path
frames_path = Path('.') / 'frames'
print(frames_path)
video = cv2.VideoCapture('badapple.mp4')
ret, frame = video.read()
i = 0
while ret:
    # threshold so we dont have any gray pixels
    frame = cv2.threshold(frame, 127, 255, cv2.THRESH_BINARY)[1]
    cv2.imwrite(str(frames_path / f'{i}.png'), frame)
    ret, frame = video.read()
    i += 1
    print(i)
