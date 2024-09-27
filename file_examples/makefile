prova: prova.o stats.o 
	g++ prova.o stats.o  -o prova.exe

prova.o: prova.cpp
	g++ -c prova.cpp -o prova.o

stats.o: stats.cpp
	g++ -c stats.cpp -o stats.o

clean:
	rm *.o *.exe