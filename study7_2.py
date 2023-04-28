import os

study = 'study7'

cases = list(range(1,10,1))

learns = ["L_{c}".format(c = c) for c in cases]


for l in learns:
	# os.system("cargo run --release Air variants/{study}/sim/{l}.yaml Threshold variants/{study}/recon/{l}.yaml study_results/{study}/Calc_diagrams/full_{l} > study_results/{study}/{l}.txt".format(study = study, l = l))
	# os.system("cargo run --release Air variants/{study}/sim/{l}_ITR.yaml Threshold variants/{study}/recon/{l}_ITR.yaml study_results/{study}/Calc_diagrams/ITR_{l} > study_results/{study}/ITR_{l}.txt".format(study = study, l = l))
	os.system("cargo run --release Air variants/{study}/sim/{l}_ITR.yaml > study_results/{study}/ITR_{l}.txt".format(study = study, l = l))


applies = ["A_{c}".format(c = c) for c in cases]
no_of_applies = list(range(0,4,1))
a = []
ra = []
for appl in applies:
	a.append([appl for no in no_of_applies])
	ra.append([appl + "_{no}".format(no = no) for no in no_of_applies])
applies = [item for sublist in a for item in sublist]
recon_applies = [item for sublist in ra for item in sublist]

config = list(zip(applies, recon_applies)) 

# for (a, ra) in config:
# 	# os.system("cargo run --release Air variants/{study}/sim/{a}.yaml Threshold variants/{study}/recon/{ra}.yaml study_results/{study}/Calc_diagrams/full_{ra} > study_results/{study}/{ra}.txt".format(study = study, a = a, ra = ra))
# 	os.system("cargo run --release Air variants/{study}/sim/{a}.yaml Threshold variants/{study}/recon/{ra}.yaml study_results/{study}/Calc_diagrams/{ra} > study_results/{study}/{ra}.txt".format(study = study, a = a, ra = ra))

""" 
# learns = ["L_{c}_ITR".format(c = c) for c in cases]
cases = list(range(1,10,1))

for c in cases:
	runfile('D:/Dokumente/Uni/BA/rebecca.pampu.ba/visualizer.py', args= "L_{c}_ITR A_{c}_0".format(c = c))
 """

# Threshold Variation

# for i in range(2):
# 	for l in learns:
# 		os.system("cargo run --release Air variants/{study}/sim/{l}.yaml Threshold variants/{study}/recon/{l}.yaml > study_results/{study}/{l}.txt".format(study = study, l = l))
# 	for (a, ra) in config:
# 		os.system("cargo run --release Air variants/{study}/sim/{a}.yaml Threshold variants/{study}/recon/{ra}.yaml study_results/{study}/Calc_diagrams/{ra} > study_results/{study}/{ra}.txt".format(study = study, a = a, ra = ra))