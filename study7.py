import os

study = 'study7'

cases = list(range(1,10,1))

learns = ["L_{c}".format(c = c) for c in cases]

""" 
$Env:RECON_PRINT="true"
 """

for l in learns:
	# os.system("cargo run --release Air variants/{study}/sim/{l}.yaml Threshold variants/{study}/recon/{l}.yaml study_results/{study}/Calc_diagrams/full_{l} > study_results/{study}/{l}.txt".format(study = study, l = l))
	os.system("cargo run --release Air variants/{study}/sim/{l}.yaml Threshold variants/{study}/recon/{l}.yaml study_results/{study}/Calc_diagrams/{l} > study_results/{study}/{l}.txt".format(study = study, l = l))


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
	# os.system("cargo run --release Air variants/{study}/sim/{a}.yaml Threshold variants/{study}/recon/{ra}.yaml study_results/{study}/Calc_diagrams/full_{ra} > study_results/{study}/{ra}.txt".format(study = study, a = a, ra = ra))
	# os.system("cargo run --release Air variants/{study}/sim/{a}.yaml Threshold variants/{study}/recon/{ra}.yaml study_results/{study}/Calc_diagrams/{ra} > study_results/{study}/{ra}.txt".format(study = study, a = a, ra = ra))
	

# cases = list(range(1,10,1))
# no_of_applies = list(range(0,4,1))
# l = []
# a = []
# ra = []
# for c in cases:
# 	a.append(["A_" + str(c) for no in no_of_applies])
# 	l.append(["L_" + str(c) for no in no_of_applies])
# 	ra.append(["A_" + str(c) + "_{no}".format(no = no) for no in no_of_applies])
# learns = [item for sublist in l for item in sublist]
# applies = [item for sublist in a for item in sublist]
# recon_applies = [item for sublist in ra for item in sublist]

# config = list(zip(applies, recon_applies, learns)) 
# for (a, ra, l) in config:
# 	os.system("cargo run --release Air variants/{study}/sim/{l}.yaml Threshold variants/{study}/recon/{ra}.yaml study_results/{study}/Calc_diagrams/{ra} > study_results/{study}/{ra}.txt".format(study = study, a = a, ra = ra, l = l))

""" 
# learns = ["L_{c}".format(c = c) for c in cases]
# applies = ["A_{c}".format(c = c) for c in cases]
cases = list(range(1,10,1))
no_of_applies = list(range(0,4,1))
no_of_applies = [0]
l = []
a = []
ra = []
for c in cases:
	a.append(["A_" + str(c) for no in no_of_applies])
	l.append(["L_" + str(c) for no in no_of_applies])
	ra.append(["A_" + str(c) + "_{no}".format(no = no) for no in no_of_applies])
learns = [item for sublist in l for item in sublist]
applies = [item for sublist in a for item in sublist]
recon_applies = [item for sublist in ra for item in sublist]

config = list(zip(applies, recon_applies, learns)) 
for (a, ra, l) in config:
	runfile('D:/Dokumente/Uni/BA/rebecca.pampu.ba/visualizer.py', args= "{a} {ra} {l}".format(a = a, ra = ra, l = l))

	runfile('D:/Dokumente/Uni/BA/rebecca.pampu.ba/visualizer.py', args= "{l} {ra} {l}".format(a = a, ra = ra, l = l))
 """

# Threshold Variation

""" 
$Env:APPEND="true"
$Env:CSV_PRINT="true"
"""

# for i in range(299):
# 	for l in learns:
# 		os.system("cargo run --release Air variants/{study}/sim/{l}.yaml Threshold variants/{study}/recon/{l}.yaml > study_results/{study}/{l}.txt".format(study = study, l = l))
# 	for (a, ra) in config:
# 		os.system("cargo run --release Air variants/{study}/sim/{a}.yaml Threshold variants/{study}/recon/{ra}.yaml study_results/{study}/Calc_diagrams/{ra} > study_results/{study}/{ra}.txt".format(study = study, a = a, ra = ra))