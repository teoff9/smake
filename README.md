# SIMPLE MAKE
Creates the makefile of a simple cpp file. After parsing the file_name.cpp to get the dependecies, generates the makefile with the following available commands:

* file_name : compile the depencies and links them with the main in file_name
* dependecy_name : compile the single dependency
* clean : remove the *.o and *.exe files created

## Usage
In the same folder as file_name.cpp or writing /path/to/file_name.cpp

`smake file_name.cpp`

## Future functions
If it's useful, I will add more functionalities.