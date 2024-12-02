.PHONY: all test clean

CXX = clang++

SOURCE_DIR=src

OBJECTS=$(SOURCE_DIR)/day_1.o
LIBS= -lssl -lcrypto

CFLAGS=-g -Wall -O0
ifeq ($(RELEASE), 1)
	CFLAGS=-O3
endif


day_1: $(OBJECTS)
	$(CXX) $(OBJECTS) -o day_1 $(LIBS)

$(OBJECTS): $(SOURCE_DIR)/day_1.cpp
	$(CXX) $(CFLAGS) -c $(SOURCE_DIR)/day_1.cpp -o $(SOURCE_DIR)/day_1.o

clean:
	rm -f $(OBJECTS) day_1
