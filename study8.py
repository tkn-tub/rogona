import os
study = 'study7'

cases = list(range(1,10,1))
learns = ["L_{c}".format(c = c) for c in cases]

""" 
$Env:RUST_LOG="info"
$Env:BINARY_MSG="true"
$Env:SIM_ONLY="true"
$Env:SIM2VID="true" """

for l in learns:
	# os.system("cargo run --release Air variants/{study}/sim/{l}.yaml Threshold variants/{study}/recon/{l}.yaml study_results/{study}/Calc_diagrams/full_{l} > study_results/{study}/{l}.txt".format(study = study, l = l))
	os.system("cargo run --release Air variants/{study}/sim/{l}.yaml".format(study = study, l = l))
