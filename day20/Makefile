CC=clang++
CFLAGS=-std=c++20 -Wall
SOURCES=src/main.cpp
OBJECTS=$(SOURCES:.cpp=.o)
EXECUTABLE=target/day20

all: $(SOURCES) $(EXECUTABLE)

$(EXECUTABLE): $(OBJECTS)
	$(CC) $(OBJECTS) -o $@

.cpp.o:
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -rf $(EXECUTABLE) $(OBJECTS)
