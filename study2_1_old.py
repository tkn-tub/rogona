import os

D = ["160", "200", "240", "280", "320"]

TP = ["0.03", "0.04", "0.05", "0.06", "0.07", "0.08"]

TSP = ["0.02", "0.03", "0.04", "0.05", "0.06", "0.07"]

name = ["d_{d}".format(d = d) for d in D]

pause = []
spray = []
for name in name:
	pause.append([name+"_t_pause_{tp}".format(tp = tp) for tp in TP])
	spray.append([name+"_t_spray_{tsp}".format(tsp = tsp) for tsp in TSP])

pause = [item for sublist in pause for item in sublist]
spray = [item for sublist in spray for item in sublist]

""" 
$Env:RUST_LOG="info"
$Env:BINARY_MSG="true"
Remove-Item Env:SIM2VID
$Env:SIM_ONLY="true"
 """

for pause in pause:
	os.system("cargo run --release Air variants/study2/sim/sim_{pause}.yaml".format(pause = pause))

for spray in spray:
	os.system("cargo run --release Air variants/study2/sim/sim_{spray}.yaml".format(spray = spray))