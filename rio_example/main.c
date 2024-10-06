#include <stdio.h>
#include "rio.h"

int main(void) {
    unsigned long tw = create_stdout_ptr();
    printf("%lu\n", tw);
    return 0;
}
