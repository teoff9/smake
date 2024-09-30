prova: prova.o stats.o 
	g++ prova.o stats.o  -o prova

prova.o: prova.cpp
	g++ -c prova.cpp -o prova.o
stats.o: stats.cpp
	g++ -c stats.cpp -o stats.o

clean:
	rm *.o prova stats.o 

