<!-- Rule: in source code, a // comment and a /* */ comment in a C file are checked, and a preprocessor directive is not. -->
#include <stdio.h>

// The retry limit is load-bearing.
/* The loop is just a loop. */
int main(void) {
	return 0;
}
