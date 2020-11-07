
CXXFLAGS=-O3 -fopenmp -std=c++11 -Wall
RM=rm -f
EXEC=mandelbrot

all: $(EXEC)

$(EXEC):
	$(CXX) $(CXXFLAGS) $(EXEC).cpp -c -o $(EXEC).o
	$(CXX) $(CXXFLAGS) $(EXEC).o -o $(EXEC)

run: 
	./$(EXEC) < $(EXEC).in

clean:
	$(RM) $(EXEC).o $(EXEC)
