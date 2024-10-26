
prova: prova.o stats.h math/math.o math/math.h
	g++ -o prova prova.o  math/math.o 

prova.o: prova.cpp
	g++ -c prova.cpp 

math.o: math/math.h math/math.cpp
	g++ -c math/math.h 

clean:
	rm  math/math.o
	rm prova

prova: prova.o stats.h math/math.o
	g++ -o prova prova.o  math/math.o 

prova.o: prova.cpp
	g++ -c prova.cpp 

math.o: math/math.h math/math.cpp
	g++ -c math/math.h 

clean:
	rm  math/math.o
	rm prova
