import csv
import matplotlib.pyplot as pp

log_filename = input("Insert log file path: ")

indices = []
makespans = []

with open(log_filename) as logfile:
    cases = logfile.read().strip().rstrip("---")
    cases = cases.split("---")

    index = 0
    for case in cases:
        data = csv.reader(case.split('\n'))
        for row in data:
            if not row == []:
                indices.append(int(row[0]))
                makespans.append(int(row[1]))
        plt = pp.figure(index, figsize=(16, 9))
        pp.title(f"Case {index}")
        pp.xlabel("Iterations")
        pp.ylabel("Makespan")
        pp.plot(indices, makespans)
        plt.show()
        indices.clear()
        makespans.clear()
        index += 1
