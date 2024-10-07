#include <stdio.h>
#include <string.h>
#include "rio.h"

int main(void) {
    init_backtrace();

    RioWriteRef twr = write_ref_new_stdout();
    twr.inner = NULL;
    RioResult line_writer_result = write_ref_new_line_writer(twr);
    if (line_writer_result.tag == ErrorVariant) {
        printf("%s\n", line_writer_result.error_variant);
        return 1;
    }
    RioWriteRef line_writer = line_writer_result.rio_write_ref_variant;
    char *string = "123456\n";
    RioResult tr = write_ref_write(&line_writer, string, strlen(string));
    printf("%d\n", tr.tag);
    if (tr.tag == ULongVariant) {
        printf("%lu\n", tr.u_long_variant);
    }
    write_ref_flush(&line_writer);
    write_ref_drop(line_writer);
    return 0;
}
