import csv

cases = list(range(1,10,1))
learns = ["L_{c}".format(c = c) for c in cases]

for l in learns:

	filename = "./sig_8/" + l + ".sig.csv"

	liv_file = "./livs/" + l + ".txt"

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