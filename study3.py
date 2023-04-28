import os

study = "study3_full"
DY = list(range(0, 21, 1))

D = [1.2, 1.6]


for d in D:
	os.system("cargo run --release addv variants/{study}/sim/50_d_{d}_ base_configs/{study}/sim_50_d_{d}.yaml dy_cm 0 1 20.5".format(study = study, d = d))

	for dy in DY:
		os.system("cargo run --release Air variants/{study}/sim/50_d_{d}_dy_cm_{dy}.yaml".format(study = study, d = d, dy = dy))

""" 
D = [1.2, 1.6]
DY = list(range(0, 21, 1))

sim = (["50_d_{d}_".format(d = d) for d in D])
s = []
for sim in sim:
	s.append([sim + "dy_cm_{dy}".format(dy = dy) for dy in DY])
sim = [item for sublist in s for item in sublist]

for sim in sim: \
	runfile('D:/Dokumente/Uni/BA/rebecca.pampu.ba/visualizer.py', args= sim)
 """

D = [1.6, 2.0, 2.4, 2.8]


for d in D:
	os.system("cargo run --release addv variants/{study}/sim/75_d_{d}_ base_configs/{study}/sim_75_d_{d}.yaml dy_cm 0 1 20.5".format(study = study, d = d))

	for dy in DY:
		os.system("cargo run --release Air variants/{study}/sim/75_d_{d}_dy_cm_{dy}.yaml".format(study = study, d = d, dy = dy))

""" 
D = [1.6, 2.0, 2.4, 2.8]
DY = list(range(0, 21, 1))

sim = (["75_d_{d}_".format(d = d) for d in D])
s = []
for sim in sim:
	s.append([sim + "dy_cm_{dy}".format(dy = dy) for dy in DY])
sim = [item for sublist in s for item in sublist]

for sim in sim: \
	runfile('D:/Dokumente/Uni/BA/rebecca.pampu.ba/visualizer.py', args= sim)
 """

D = [2.8, 3.2]


for d in D:
	os.system("cargo run --release addv variants/{study}/sim/100_d_{d}_ base_configs/{study}/sim_100_d_{d}.yaml dy_cm 0 1 20.5".format(study = study, d = d))

	for dy in DY:
		os.system("cargo run --release Air variants/{study}/sim/100_d_{d}_dy_cm_{dy}.yaml".format(study = study, d = d, dy = dy))

""" 
D = [2.8, 3.2]
DY = list(range(0, 21, 1))

sim = (["100_d_{d}_".format(d = d) for d in D])
s = []
for sim in sim:
	s.append([sim + "dy_cm_{dy}".format(dy = dy) for dy in DY])
sim = [item for sublist in s for item in sublist]

for sim in sim: \
	runfile('D:/Dokumente/Uni/BA/rebecca.pampu.ba/visualizer.py', args= sim)
 """