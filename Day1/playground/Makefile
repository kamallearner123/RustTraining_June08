CC=gcc
CFLAGS=-g -O0 -Wall
ASAN_FLAGS=-fsanitize=address

all: invalid_memory crash_sim invalid_memory_asan

invalid_memory: invalid_memory/main.c
	$(CC) $(CFLAGS) -o invalid_memory.out invalid_memory/main.c

invalid_memory_asan: invalid_memory/main.c
	$(CC) $(CFLAGS) $(ASAN_FLAGS) -o invalid_memory_asan.out invalid_memory/main.c

crash_sim: crash_sim/main.c
	$(CC) $(CFLAGS) -o crash_sim.out crash_sim/main.c

clean:
	rm -f *.out core.*
