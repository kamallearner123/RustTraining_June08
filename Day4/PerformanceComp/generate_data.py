import random

num_rows = 5000000
filename = "data.csv"

print(f"Generating {filename} with {num_rows} rows...")

with open(filename, "w") as f:
    f.write("id,value1,value2\n")
    for i in range(num_rows):
        v1 = random.uniform(0.0, 100.0)
        v2 = random.uniform(0.0, 100.0)
        f.write(f"{i},{v1:.4f},{v2:.4f}\n")

print("Done generating data.")
