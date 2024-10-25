ARG0 := $(-std=g++20)

prova: prova.o stats.h math/math.o math/math.h
	g++ -o prova prova.o  math/math.o $(ARG0)

prova.o: prova.cpp
	g++ -c prova.cpp $(ARG0)

math.o: math/math.h math/math.cpp
	g++ -c math/math.h $(ARG0)

clean:
	rm  math/math.o
	rm prova
ARG0 := $(-std=g++20)

prova: prova.o stats.h math/math.o math/math.h
	g++ -o prova prova.o  math/math.o $(ARG0)

prova.o: prova.cpp
	g++ -c prova.cpp $(ARG0)

math.o: math/math.h math/math.cpp
	g++ -c math/math.h $(ARG0)

clean:
	rm  math/math.o
	rm prova
