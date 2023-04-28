import os


D = ["160", "240", "320"]

DY = ["0", "8"]

name = ["d_{d}_".format(d = d) for d in D]

n = []
for name in name:
	n.append([name+"dy_{dy}".format(dy = dy) for dy in DY])

name = [item for sublist in n for item in sublist]

for name in name:
	os.system("cargo run --release addv variants/study1/sim/" + name + "_ base_configs/study1/sim_" + name + ".yaml mps 800 200 1401")
	os.system("cargo run --release addl variants/study1/recon/" + name + "_ base_configs/study1/recon_" + name + ".yaml base_configs/study1/learn_" + name + ".yaml mps 800 200 1401")
	
#MPS = ["800", "1000", "1200", "1400"]


#n = []
#for name in name:
#	n.append([name+"_mps_{mps}".format(mps = mps) for mps in MPS])

#name = [item for sublist in n for item in sublist]

#for name in name:
#	print(name)
#	os.system("md .\\variants\\study1\\liv\\" + name)