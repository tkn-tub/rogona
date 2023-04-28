import csv

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

for name in name:

	filename = "./sig/" + name + ".sig.csv"

	liv_file = "./livs/" + name + ".txt"

	#from https://www.geeksforgeeks.org/working-csv-files-python/

	fields = []
	rows = []

	with open(filename, 'r') as csvfile:
		# creating a csv reader object
		csvreader = csv.reader(csvfile)
		# extracting field names through first row
		fields = next(csvreader)
		# extracting each data row one by one
		for row in csvreader:
			rows.append(row)

	# Quote end

	livs = []

	for row in rows:
		livs.append(row[3])


	with open(liv_file, 'w') as f:
		for liv in livs:
			f.write(str(liv)+";")