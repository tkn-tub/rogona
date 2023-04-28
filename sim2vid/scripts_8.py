import os
import numpy as np
import sys

script_animation_path = "./scripts/postprocessing_mamoko2020air_animation.py"
script_vid2rgb_path = "./scripts/postprocessing_mamoko2020air_video_rgb.py"
script_rgb2sig_path = "./scripts/postprocessing_mamoko2020air_signal_from_rgb.py"

cases = list(range(1,10,1))
learns = ["L_{c}".format(c = c) for c in cases]
dist = ["60", "65", "83", "103", "128", "150", "156", "170", "137"]

params = zip(learns, dist)

for (l, d) in params:

	positions_dir = "../variants/study7/liv/" + l
	
	video_output = "./video_8/" + l + ".mp4"


	command = "python3 {animation_script} -i {pos_dir} -o {vid_out} --video-width 1280 --video-height 768 --tx-rx-distance-cm {dist}".format(animation_script = script_animation_path, pos_dir = positions_dir, vid_out = video_output, dist = d)
	os.system(command)
	# print(command)

	rgb_csv = "./sig_8/" + l + ".rgb.csv"

	command = "python3 {rgb_script} -i {vid_out} -o {rgb_out} --fps 480".format(rgb_script = script_vid2rgb_path, vid_out = video_output, rgb_out = rgb_csv)
	os.system(command)
	# print(command)

	sig_csv = "./sig_8/" + l + ".sig.csv"

	command = "python3 {sig_script} -i {rgb_in} -o {sig_out}".format(sig_script = script_rgb2sig_path, rgb_in = rgb_csv, sig_out = sig_csv)
	os.system(command)
	# print(command)


os.system("echo -en '\007'")
