
        // This is the main comment section
        #include <vector>
        #include "utility.h"
        
        /* A comment that explains the following includes
           #include "hidden_lib.h" // This should not be included
        */

        // Testing multiple includes in one line
        #include "lib1.h", "lib2.h", "lib3.h" // Should be extracted

        // Single-line include with inline comment
        #include "my_lib.h" // This is a comment

        #include "/absolute/path/to/another_lib.h" // Valid path

        /* This block of comments will 
           also include a commented out line:
           #include "not_included.h" 
        */

        // An empty include (should not match)
        #include ""

        // Valid include with space in filename
        #include "lib with space.h"

        #include "library_with_comment.h" /* 
            #include "this_should_not_be_included.h" 
        */

        // Commented out includes
        //#include "commented_lib1.h"
        //#include "commented_lib2.h"

        #include <iostream>  // Standard library, should be included

        // Last comment
        // #include "should_not_include.h" - this is commented out
#include "stats.h"
    

using namespace std;

int main() {
    cout << "Hello world!" << endl;
    cout << media << endl;
    return 0;
}