# bad apple

Goal: play bad apple using a single particle moving really fast

Methodology:

1. Split video into frames (split_into_frames.py)
2. Figure out how to get list of positions that form the contour of the frames of the video. Save the parameters needed for each frame. (generate_video_and_config.py)
Example of parameters include how much we dilate/erode by, minimum distance between contour points, what kind of border we put around the video etc.
3. Deal with the frames that arne't processed well by going through all of the frames and tuning the parameters for each frame. (streamlit run workflow.py)

At the end, I have config.pkl which contains the parameters for each frame, and 6000 pkl files, 1 for each frame, just containing the list of points that form the contour of the frame.

Next steps: 
TSP for each frame, because some points are out of order, mainly those that came from different contours in the samse frame.
Then, optimize between frames which point we are starting at.