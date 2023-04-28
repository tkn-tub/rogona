import os
""" 
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

for name in name: \
    runfile('D:/Dokumente/Uni/BA/rebecca.pampu.ba/visualizer.py', args= name)
 """

# for name in name:
# 	print(name)
# 	os.system("python visualizer.py {name}".format(name = name))
	





