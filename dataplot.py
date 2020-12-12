import csv
import matplotlib.pyplot as pp

log_filename = input("Insert log file filename: ")

indices = []
makespans = []

with open(log_filename) as logfile:
    data = csv.reader(logfile)
    next(data, None)
    for row in data:
        indices.append(int(row[0]))
        makespans.append(int(row[1]))

pp.plot(indices, makespans)