import os

""" 
BINARY_MSG
RUST_LOG
CSV_PRINT
 """

# From study3_full
# 1)

study = "study6"
DY = list(range(0, 21, 1))
#DY = list(range(8, 12, 1))

D = ["50_d_1.2", "50_d_1.6", "75_d_1.6", "75_d_2.0", "75_d_2.4", "100_d_2.8", "100_d_3.2"] 


# for d in D:
# 	#os.system("cargo run --release addv variants/{study}/sim/1_{d}_ base_configs/{study}/1/sim_{d}.yaml dy_cm 0 1 20.5".format(study = study, d = d))
# 	#os.system("cargo run --release addl variants/{study}/recon/1_{d}_ base_configs/{study}/1/recon_{d}.yaml base_configs/{study}/1/learn_{d}.yaml dy_cm 0 1 20.5".format(study = study, d = d))

# 	for dy in DY:
# 		os.system("cargo run --release Air variants/{study}/sim/1_{d}_dy_cm_{dy}.yaml Threshold variants/{study}/recon/1_{d}_dy_cm_{dy}.yaml".format(study = study, d = d, dy = dy))

""" 
D = ["50_d_1.2", "50_d_1.6", "75_d_1.6", "75_d_2.0", "75_d_2.4", "100_d_2.8", "100_d_3.2"]
DY = list(range(1, 21, 1))

D = ["50_d_1.2", "50_d_1.6", "75_d_1.6", "75_d_2.0", "75_d_2.4", "100_d_2.8", "100_d_3.2"]
DY = list(range(1, 21, 1))

sim = (["1_{d}_".format(d = d) for d in D])
s = []
for sim in sim:
	s.append([sim + "dy_cm_{dy}".format(dy = dy) for dy in DY])
sim = [item for sublist in s for item in sublist]

for sim in sim: \
	runfile('D:/Dokumente/Uni/BA/rebecca.pampu.ba/visualizer.py', args= sim)
 """

# From study4
# 2.1)

setups = ["50_d_1.2", "50_d_1.6", "75_d_1.6", "50_d_1.6_dy_5", "75_d_2.0_dy_5", "75_d_2.4_dy_8", "75_d_2.8_dy_10", "100_d_3.2_dy_10"]

for s in setups:
	os.system("cargo run --release addv variants/{study}/sim/2_{s}_ base_configs/{study}/2/sim_{s}.yaml off_f 0.05 0.05 0.51".format(study = study, s = s))
	os.system("cargo run --release addv variants/{study}/sim/2_{s}_ base_configs/{study}/2/sim_{s}.yaml off_n 0.0 0.05 0.51".format(study = study, s = s))
	os.system("cargo run --release addl variants/{study}/recon/2_{s}_ base_configs/{study}/2/recon_{s}.yaml base_configs/{study}/2/learn_{s}.yaml off_f 0.05 0.05 0.51".format(study = study, s = s))
	os.system("cargo run --release addl variants/{study}/recon/2_{s}_ base_configs/{study}/2/recon_{s}.yaml base_configs/{study}/2/learn_{s}.yaml off_n 0.0 0.05 0.51".format(study = study, s = s))

off = ["0.00", "0.05", "0.10", "0.15", "0.20", "0.25", "0.30", "0.35", "0.40", "0.45", "0.50"]

""" 
$Env:RUST_LOG="info"
$Env:BINARY_MSG="true"
($Env:RECON_PRINT="true")
 """

for s in setups:
	for o in off:
		os.system("cargo run --release Air variants/{study}/sim/2_{s}_off_f_{o}.yaml Threshold variants/{study}/recon/2_{s}_off_f_{o}.yaml study_results/{study}/Calc_diagrams/2_{s}_off_f_{o}".format(study = study, s = s, o = o))
		os.system("cargo run --release Air variants/{study}/sim/2_{s}_off_n_{o}.yaml Threshold variants/{study}/recon/2_{s}_off_n_{o}.yaml study_results/{study}/Calc_diagrams/2_{s}_off_f_{o}".format(study = study, s = s, o = o))
		
""" 

setups = ["50_d_1.2", "50_d_1.6", "75_d_1.6", "50_d_1.6_dy_5", "75_d_2.0_dy_5", "75_d_2.4_dy_8", "75_d_2.8_dy_10", "100_d_3.2_dy_10"]
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

# 2.2) 

setups = ["50_d_1.6_dy_5", "75_d_2.0_dy_5", "75_d_2.4_dy_8", "75_d_2.8_dy_10", "100_d_3.2_dy_10"]

# for s in setups:
# 	os.system("cargo run --release addv variants/{study}/sim/2_{s}_ base_configs/{study}/2/sim_{s}.yaml off_n 0.0 0.05 0.51".format(s = s))
# 	os.system("cargo run --release addv variants/{study}/sim/2_{s}_ base_configs/{study}/2/sim_{s}.yaml off_f 0.05 0.05 0.51".format(s = s))


off = ["0.00", "0.05", "0.10", "0.15", "0.20", "0.25", "0.30", "0.35", "0.40", "0.45", "0.50"]

setups = ["75_d_2.8_dy_10"]
off = ["0.55", "0.60", "0.65", "0.70"]

# for s in setups:
# 	for o in off:
# 		os.system("cargo run --release Air variants/{study}/sim/2_{s}_off_f_{o}.yaml".format(s = s, o = o))
# 		os.system("cargo run --release Air variants/{study}/sim/2_{s}_off_n_{o}.yaml".format(s = s, o = o))



""" 
setups = ["50_d_1.6_dy_5", "75_d_2.0_dy_5", "75_d_2.4_dy_8", "75_d_2.8_dy_10", "100_d_3.2_dy_10"]
off_n = ["0.00", "0.05", "0.10", "0.15", "0.20", "0.25", "0.30", "0.35", "0.40", "0.45", "0.50", "0.55", "0.60", "0.65", "0.70"]
setups = ["50_d_1.6_dy_5"]
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

