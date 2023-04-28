import os

""" 
BINARY_MSG
RUST_LOG
CSV_PRINT
FRAME_OFFSET="fr_off"
 """


# 1)

study = "paper/1"
version = "1"
# version = "2"
# fr_off = "-1"
# fr_off = "+1"
fr_off = "-3"
# fr_off = "+3"
# fr_off = ""

# fr_off_arr = ["+1", "-3", "+3"]

# for fr_off in fr_off_arr:
# 	os.system("cargo run --release addv variants/{study}/sim/L_{v}{fr}_ base_configs/{study}/sim_L_{v}.yaml off_f 0 0.05 0.61".format(study = study, v = version, fr = fr_off))
# 	os.system("cargo run --release addl variants/{study}/recon/L_{v}{fr}_ base_configs/{study}/recon_L_{v}{fr}.yaml base_configs/{study}/learn_L_{v}{fr}.yaml off_f 0 0.05 0.61".format(study = study, v = version, fr = fr_off))
# 	os.system("cargo run --release addv variants/{study}/sim/A_{v}{fr}_ base_configs/{study}/sim_A_{v}.yaml off_f 0 0.05 0.61".format(study = study, v = version, fr = fr_off))
# 	os.system("cargo run --release adda variants/{study}/recon/A_{v}{fr}_ base_configs/{study}/recon_A_{v}{fr}.yaml false off_f 0 0.05 0.61".format(study = study, v = version, fr = fr_off))

OFF = ["0.00", "0.05", "0.10", "0.15", "0.20", "0.25", "0.30", "0.35", "0.40", "0.45", "0.50", "0.55", "0.60"]

"""
APPEND after one run
"""


for _ in range(10):
	for off in OFF:
		# os.system("cargo run --release Air variants/{study}/sim/L_{v}{fr}_off_f_{off}.yaml Threshold variants/{study}/recon/L_{v}{fr}_off_f_{off}.yaml".format(study = study, off = off, v = version, fr = fr_off))
		# os.system("cargo run --release Air variants/{study}/sim/A_{v}{fr}_off_f_{off}.yaml Threshold variants/{study}/recon/A_{v}{fr}_off_f_{off}_0.yaml".format(study = study, off = off, v = version, fr = fr_off))
		os.system("rogona_ab_molcom.exe Air variants/{study}/sim/L_{v}{fr}_off_f_{off}.yaml Threshold variants/{study}/recon/L_{v}{fr}_off_f_{off}.yaml".format(study = study, off = off, v = version, fr = fr_off))
		os.system("rogona_ab_molcom.exe Air variants/{study}/sim/A_{v}{fr}_off_f_{off}.yaml Threshold variants/{study}/recon/A_{v}{fr}_off_f_{off}_0.yaml".format(study = study, off = off, v = version, fr = fr_off))

"""
no_of_applies = list(range(0,4,1))

cases = ["0.00", "0.05", "0.10", "0.15", "0.20", "0.25", "0.30", "0.35", "0.40", "0.45", "0.50", "0.55", "0.60"]
no_of_applies = [0]
l = []
a = []
ra = []
for c in cases:
	a.append(["A_1_off_f_" + str(c) for no in no_of_applies])
	l.append(["L_1_off_f_" + str(c) for no in no_of_applies])
	ra.append(["A_1_off_f_" + str(c) + "_{no}".format(no = no) for no in no_of_applies])
learns = [item for sublist in l for item in sublist]
applies = [item for sublist in a for item in sublist]
recon_applies = [item for sublist in ra for item in sublist]

config = list(zip(applies, recon_applies, learns)) 
for (a, ra, l) in config:
	runfile('D:/Dokumente/Uni/BA/rebecca.pampu.ba/visualizer.py', args= "{a} {ra} {l}".format(a = a, ra = ra, l = l))

	runfile('D:/Dokumente/Uni/BA/rebecca.pampu.ba/visualizer.py', args= "{l} {ra} {l}".format(a = a, ra = ra, l = l))
"""