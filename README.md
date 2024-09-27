# SIMPLE MAKE
Creates the makefile of a simple cpp file assuming the header files are in the same location as the source code of the dependencies and the dependecies to compile are just:

 `#include "path/to/lib.h"`
 
Smake will generate the makefile with the following available commands:

* file_name : compile the depencies and links them with the main in file_name
* dependecy_name : compile the single dependency
* clean : remove the *.o and *.exe files created

## Installation 
Install Rust using:

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

After the installation clone the smake repo or download the .zip file and decompress it. Now open in a terminal the repository folder and run:

`cargo build --release`

After this, the smake executable will be found under:

 `/path/to/smake/target/release/`

Now, copy the smake shell file found in the repository somewhere in your home repo under a repo named, for example, scripts. Now add at the bottom of the file `~/.bashrc` the following line using the path to your scripts folder:

`export PATH="/path/to/scripts:$PATH"`

After this, you just need to edit the `smake` shell file adding the path to where the rust executable is 

`/path/to/smake/target/release/smake`

After opening a new shell session, you'll be able to use `smake`.



## Usage
Write in a shell session, for now (v0.0.n) smake will accept only one program:

`smake relative/path/to/file_name.cpp`

## Future functions
* Better error handling
* Being able to update a makefile
* Parameters involving the dependencies