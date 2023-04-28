import os

# 1) 

setups = ["50_d_1.2", "50_d_1.6", "75_d_1.6"]

#for s in setups:
#	os.system("cargo run --release addv variants/study4/sim/1_{s}_ base_configs/study4/1/sim_{s}.yaml off_f 0.0 0.05 0.51".format(s = s))

off = ["0", "0.05", "0.1", "0.15", "0.2", "0.25", "0.3", "0.35", "0.4", "0.45", "0.5"]

""" 
$Env:RUST_LOG="info"
$Env:BINARY_MSG="true"
$Env:SIM_ONLY="true"
 """

# for s in setups:
# 	for o in off:
# 		os.system("cargo run --release Air variants/study4/sim/1_{s}_off_f_{o}.yaml".format(s = s, o = o))

""" 

setups = ["50_d_1.2", "50_d_1.6", "75_d_1.6"]
off = ["0", "0.05", "0.1", "0.15", "0.2", "0.25", "0.3", "0.35", "0.4", "0.45", "0.5"]

sim = (["1_{set}_".format(set = set) for set in setups])
s = []
for sim in sim:
	s.append([sim + "off_f_{o} {o}".format(o = o) for o in off])
sim = [item for sublist in s for item in sublist]

for sim in sim: \
	runfile('D:/Dokumente/Uni/BA/rebecca.pampu.ba/visualizer.py', args= sim)

 """

# 2) 

setups = ["50_d_1.6_dy_5", "75_d_2.0_dy_5", "75_d_2.4_dy_8", "75_d_2.8_dy_10", "100_d_3.2_dy_10"]

# for s in setups:
# 	os.system("cargo run --release addv variants/study4/sim/2_{s}_ base_configs/study4/2/sim_{s}.yaml off_n 0.0 0.05 0.51".format(s = s))
# 	os.system("cargo run --release addv variants/study4/sim/2_{s}_ base_configs/study4/2/sim_{s}.yaml off_f 0.05 0.05 0.51".format(s = s))


off = ["0.00", "0.05", "0.10", "0.15", "0.20", "0.25", "0.30", "0.35", "0.40", "0.45", "0.50"]

setups = ["75_d_2.8_dy_10"]
off = ["0.55", "0.60", "0.65", "0.70"]

for s in setups:
	for o in off:
		#os.system("cargo run --release Air variants/study4/sim/2_{s}_off_f_{o}.yaml".format(s = s, o = o))
		os.system("cargo run --release Air variants/study4/sim/2_{s}_off_n_{o}.yaml".format(s = s, o = o))



""" 
off_n = ["0.00", "0.05", "0.10", "0.15", "0.20", "0.25", "0.30", "0.35", "0.40", "0.45", "0.50", "0.55", "0.60", "0.65", "0.70"]
setups = ["75_d_2.8_dy_10"]
setups = ["50_d_1.6_dy_5", "75_d_2.0_dy_5", "75_d_2.4_dy_8", "75_d_2.8_dy_10", "100_d_3.2_dy_10"]
off_f = ["0.05", "0.10", "0.15", "0.20", "0.25", "0.30", "0.35", "0.40", "0.45", "0.50"]
off_n = ["0.00", "0.05", "0.10", "0.15", "0.20", "0.25", "0.30", "0.35", "0.40", "0.45", "0.50"]
off_n.reverse()

sim = (["2_{set}_".format(set = set) for set in setups])
s_n = []
s_f = []
for s in sim:
	s_n.append([s + "off_n_{o}".format(o = o) for o in off_n])
	s_f.append([s + "off_f_{o}".format(o = o) for o in off_f])
s_f = [item for sublist in s_f for item in sublist]
s_n.append(s_f)
sim = [item for sublist in s_n for item in sublist]

for sim in sim: \
	runfile('D:/Dokumente/Uni/BA/rebecca.pampu.ba/visualizer.py', args= sim) 
	
	"""