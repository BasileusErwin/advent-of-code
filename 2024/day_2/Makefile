.PHONY: all test clean

CXX = clang++

SOURCE_DIR=src

OBJECTS=$(SOURCE_DIR)/day_2.o
LIBS= -lssl -lcrypto

CFLAGS=-g -Wall -O0
ifeq ($(RELEASE), 2)
	CFLAGS=-O3
endif


day_2: $(OBJECTS)
	$(CXX) $(OBJECTS) -o day_2 $(LIBS)

$(OBJECTS): $(SOURCE_DIR)/day_2.cpp
	$(CXX) $(CFLAGS) -c $(SOURCE_DIR)/day_2.cpp -o $(SOURCE_DIR)/day_2.o

clean:
	rm -f $(OBJECTS) day_2
