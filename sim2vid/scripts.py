import os
import numpy as np
import sys

script_animation_path = "./scripts/postprocessing_mamoko2020air_animation.py"
script_vid2rgb_path = "./scripts/postprocessing_mamoko2020air_video_rgb.py"
script_rgb2sig_path = "./scripts/postprocessing_mamoko2020air_signal_from_rgb.py"

D = ["160", "240", "320"]
D2 = [80, 120, 160]

DY = ["0", "8"]
DY2 = [0, 8]

MPS = ["800", "1000", "1200", "1400"]

name = ["d_{d}_".format(d = d) for d in D]
dist = D2

n = []
di = []
for name in name:
	n.append([name+"dy_{dy}_".format(dy = dy) for dy in DY])

for dist in dist:
	di.append([dist+x for x in DY2])

name = [item for sublist in n for item in sublist]
dist = [item for sublist in di for item in sublist]

n = []
di = []
for name in name:
	n.append([name+"mps_{mps}".format(mps = mps) for mps in MPS])

for dist in dist:
	di.append([dist for mps in MPS])

name = [item for sublist in n for item in sublist]
dist = [item for sublist in di for item in sublist]

param = list(zip(name, dist))

for (name, dist) in param:

	positions_dir = "../variants/study1/liv/" + name
	dist = str(dist)
	video_output = "./video/" + name + ".mp4"


	command = "python3 {animation_script} -i {pos_dir} -o {vid_out} --video-width 1280 --video-height 768 --tx-rx-distance-cm {dist}".format(animation_script = script_animation_path, pos_dir = positions_dir, vid_out = video_output, dist = dist)
	#os.system(command)
	print(command)

	rgb_csv = "./sig/" + name + ".rgb.csv"

	command = "python3 {rgb_script} -i {vid_out} -o {rgb_out} --fps 480".format(rgb_script = script_vid2rgb_path, vid_out = video_output, rgb_out = rgb_csv)
	#os.system(command)
	print(command)

	sig_csv = "./sig/" + name + ".sig.csv"

	command = "python3 {sig_script} -i {rgb_in} -o {sig_out}".format(sig_script = script_rgb2sig_path, rgb_in = rgb_csv, sig_out = sig_csv)
	#os.system(command)
	print(command)


os.system("echo -en '\007'")
