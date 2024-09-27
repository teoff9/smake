# SIMPLE MAKE
Creates the makefile of a simple cpp file assuming the header files are in the same location as the source code of the dependencies and the dependecies to compile are just

 `#include "path/to/lib.h"`
 
Smake will generate the makefile with the following available commands:

* file_name : compile the depencies and links them with the main in file_name
* dependecy_name : compile the single dependency
* clean : remove the *.o and *.exe files created

## Usage
Create the smake shell command using the smake shell file adding your path to smake in the release folder of the cargo project. In the same folder as file_name.cpp or writing /path/to/file_name.cpp

`smake file_name.cpp`

## Future functions
* Better make clean
* Logging
* Better error handling
* Parameters involving the dependencies