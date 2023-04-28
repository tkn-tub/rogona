import os


D = ["160", "240", "320"]

DY = ["0", "8"]

MPS = ["800", "1000", "1200", "1400"]


name = ["d_{d}_".format(d = d) for d in D]

n = []
for name in name:
	n.append([name+"dy_{dy}_".format(dy = dy) for dy in DY])

name = [item for sublist in n for item in sublist]
n = []
for name in name:
	n.append([name+"mps_{mps}".format(mps = mps) for mps in MPS])

name = [item for sublist in n for item in sublist]

# Powershell - set manually
# os.system("$Env:RUST_LOG=\"true\"")
# os.system("$Env:BINARY_MSG=\"true\"")
# os.system("$Env:SIM2VID=\"true\"")
# could also set SIM_ONLY=true then the command could end before Threshold.


for name in name:
	sim = "variants/study1/sim/{name}.yaml".format(name = name)
	recon = "variants/study1/recon/{name}.yaml".format(name = name)
	# Powershell
	os.system("cargo run --release Air {sim_config} Threshold {recon_config}".format(sim_config = sim, recon_config = recon))
	# Unix
	#os.system("RUST_LOG=\"info\" BINARY_MSG=\"true\" SIM2VID=\"true\" cargo run --release Air {sim_config} Threshold {recon_config}".format(sim_config = sim, recon_config = recon))


#Todo: apply configs abrufen
#Todo: plots ordner

