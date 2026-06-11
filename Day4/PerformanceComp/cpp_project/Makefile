CXX = g++
CXXFLAGS = -Wall -Wextra -O3 -std=c++17

TARGET = process_data

all: $(TARGET)

$(TARGET): main.cpp
	$(CXX) $(CXXFLAGS) -o $(TARGET) main.cpp

clean:
	rm -f $(TARGET)

.PHONY: all clean
