import seaborn
import csv

with open("dr-iq-scores.csv") as f:
    records = [(l[0], l[1]) for l in csv.reader(f)]

seaborn.histplot(records)
