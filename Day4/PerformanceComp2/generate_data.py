import random
import string
import os

def generate_words(filename, num_words, unique_pool_size):
    print(f"Generating {num_words} words for {filename}...")
    
    # Generate a pool of unique words
    pool = []
    for _ in range(unique_pool_size):
        length = random.randint(3, 10)
        word = ''.join(random.choices(string.ascii_lowercase, k=length))
        pool.append(word)
        
    # Write random words from the pool to the file
    with open(filename, 'w') as f:
        for _ in range(num_words):
            f.write(random.choice(pool) + '\n')
            
    print("Data generation complete.")

if __name__ == "__main__":
    file_path = "words.txt"
    if not os.path.exists(file_path):
        # Generate 10 million words from a pool of 50,000 unique words
        generate_words(file_path, 10_000_000, 50_000)
    else:
        print(f"Data file '{file_path}' already exists.")
