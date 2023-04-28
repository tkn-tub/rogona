import os

# 1)

study = "study5"

setups = ["50_d_1.2", "75_d_1.6", "100_d_2.4"]

dy = ["0", "5", "10", "15", "20"]

# for s in setups:
# 	os.system("cargo run --release addv variants/study5/sim/1_{s}_ base_configs/study5/1/sim_{s}.yaml dy_cm 0 5 21".format(s = s))
# 	os.system("cargo run --release addl variants/{study}/recon/1_{s}_ base_configs/{study}/1/recon_{s}.yaml base_configs/{study}/1/learn_{s}.yaml dy_cm 0 5 20.5".format(study = study, s = s))


""" 
$Env:RUST_LOG="info"
$Env:BINARY_MSG="true"
$Env:SIM_ONLY="true"

$Env:RECON_PRINT="true"
$Env:RECON_FULL_PRINT="true"
 """

for s in setups:
	for dy_ in dy:
		#os.system("cargo run --release Air variants/{study}/sim/1_{s}_dy_cm_{dy}.yaml Threshold variants/{study}/recon/1_{s}_dy_cm_{dy}.yaml".format(study = study, s = s, dy = dy_))
		os.system("cargo run --release Air variants/{study}/sim/1_{s}_dy_cm_{dy}.yaml Threshold variants/{study}/recon/1_{s}_dy_cm_{dy}.yaml study_results/study5/Calc_diagrams/full_1_{s}_dy_cm_{dy} >> out.txt".format(study = study, s = s, dy = dy_))
		
""" 
setups = ["50_d_1.2", "75_d_1.6", "100_d_2.4"]
dy = ["0", "5", "10", "15", "20"]

setups = ["50_d_1.2"]
dy = ["5"]

sim = (["1_{set}_".format(set = set) for set in setups])
s = []
for sim in sim:
	s.append([sim + "dy_cm_{dy}".format(dy = dy) for dy in dy])
sim = [item for sublist in s for item in sublist]

for sim in sim: \
	runfile('D:/Dokumente/Uni/BA/rebecca.pampu.ba/visualizer.py', args= sim)

 """


# .\rogona_ab_molcom.exe Air variants/study5/sim/1_50_d_1.2_dy_cm_5.yaml Threshold variants/study5/recon/1_50_d_1.2_dy_cm_5.yaml study_results/study5/Calc_diagrams/full_1_50_d_1.2_dy_cm_5
# .\rogona_ab_molcom.exe Air variants/study5/sim/1_50_d_1.2_dy_cm_5.yaml Threshold variants/study5/recon/1_50_d_1.2_dy_cm_5.yaml study_results/study5/Calc_diagrams/1_50_d_1.2_dy_cm_5