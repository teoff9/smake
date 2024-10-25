#SMAKE
Simple make file generator written in Rust. Takes a .c or .cpp file and generate the makefile reading its dependencies. You can also add extra arguments or choose the compiler between gcc or g++. It has a serious limitation: the dependency search is not yet recoursive and stops just for the dependecies of the target. In the future I will maybe complete the project with these feature.

##INSTALLATION
Clone the repo, cd in it and type:

'cargo install smake'

then check if your $PATH is updated with where cargo installed the script. Enjoy!

##LICENSE
Feel free to do whatever you want.