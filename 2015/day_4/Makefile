.PHONY: all test clean

CXX = clang++

SOURCE_DIR=src

OBJECTS=$(SOURCE_DIR)/day_4.o
LIBS= -lssl -lcrypto

CFLAGS=-g -Wall -O0
ifeq ($(RELEASE), 1)
	CFLAGS=-O3
endif


day_4: $(OBJECTS)
	$(CXX) $(OBJECTS) -o day_4 $(LIBS)

$(OBJECTS): $(SOURCE_DIR)/day_4.cpp
	$(CXX) $(CFLAGS) -c $(SOURCE_DIR)/day_4.cpp -o $(SOURCE_DIR)/day_4.o

clean:
	rm -f $(OBJECTS) day_4
