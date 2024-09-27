file_examples/prova: file_examples/prova.o 

	g++ file_examples/prova.o  -o file_examples/prova.exe

file_examples/prova.o: file_examples/prova.cpp
	g++ -c file_examples/prova.cpp -o file_examples/prova.o

clean:
	rm file_examples/prova.o file_examples/prova.exe 

