# bad apple

Goal: play bad apple using a single particle moving really fast

Methodology:

1. Split video into frames (`split_into_frames.py`)
2. Figure out how to get list of positions that form the contour of the frames of the video. Save the parameters needed for each frame. (`generate_video_and_config.py`,)
Example of parameters include how much we dilate/erode by, minimum distance between contour points, what kind of border we put around the video etc.
3. Deal with the frames that arne't processed well by going through all of the frames and tuning the parameters for each frame. (`streamlit run workflow.py`)

At the end, I have config.pkl which contains the parameters for each frame, and 6000 pkl files, 1 for each frame, just containing the list of points that form the contour of the frame. I reviewed these frames and touched up ones that were wrong by making a video of the frames (`make_pickles_video.py`)

4. Now, I had the correct points for every frame. However, sometimes these were points from multiple contours and so their order would be wrong. Thus, I used the Travelling Salesman Problem heuristic to order the points, and then something like the curl to orient them in the right direction (`badapple_contour.ipnb`).
I saved these points to bad-apple-tsp.json. Json format was better than pickle because you can read it in text editor.

5. Next step would be to optimize which point to start at for the next frame so that the particle doesn't have to move as far. Another easy problem that would need to be solved would be moving the point between points that are too far apart. Regardless, bad apple was abandoned in favour of getting the new board to work at all.

