.PHONY: all test clean

CXX = clang++

SOURCE_DIR=src

OBJECTS=$(SOURCE_DIR)/day_3.o

day_3: $(OBJECTS)
	$(CXX) $(OBJECTS) -o day_3

$(OBJECTS): $(SOURCE_DIR)/day_3.cpp
	$(CXX) -c $(SOURCE_DIR)/day_3.cpp -o $(OBJECTS)

clean:
	rm -f $(OBJECTS) day_3
