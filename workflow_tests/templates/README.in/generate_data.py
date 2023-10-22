import csv
from faker import Faker
import random

# Create a Faker object
fake = Faker()
Faker.seed(20231018)

# Number of records to generate
num_records = 1000

# Generate random names and IQ scores and write to CSV file
with open('random_names_iq_scores.csv', 'w', newline='') as csvfile:
    fieldnames = ['Name', 'IQ Score']
    writer = csv.DictWriter(csvfile, fieldnames=fieldnames)
    
    writer.writeheader()
    for _ in range(num_records):
        name = fake.name()
        iq_score = fake.random_int(70, 140)
        writer.writerow({'Name': name, 'IQ Score': iq_score})

print('CSV file generated successfully.')
