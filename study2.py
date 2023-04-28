import os

setups = ["20_30", "20_55", "20_80", "45_30", "45_55", "70_30"]
D = ["1.6", "2", "2.4", "2.8", "3.2"]

sim = []
for setup in setups:
	#os.system("cargo run --release addv variants/study2/sim/{setup}_ base_configs/study2/sim_{setup}.yaml d 1.6 0.4 3.21".format(setup = setup))
	sim.append([setup+"_d_{d}".format(d = d) for d in D])

sim = [item for sublist in sim for item in sublist]

for sim in sim:
	os.system("cargo run --release Air variants/study2/sim/{sim}.yaml".format(sim = sim))


#_____________________________________________________
# For visualisation:
""" 
setups = ["20_30", "20_55", "20_80", "45_30", "45_55", "70_30"]
D = ["1.6", "2", "2.4", "2.8", "3.2"]
sim = []
for setup in setups:
	sim.append([setup+"_d_{d}".format(d = d) for d in D])

sim = [item for sublist in sim for item in sublist]

for sim in sim: \
	runfile('D:/Dokumente/Uni/BA/rebecca.pampu.ba/visualizer.py', args= sim)
 """