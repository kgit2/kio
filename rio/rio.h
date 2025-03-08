#ifndef RIO_H
#define RIO_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

FFIResult file_open(char *path);

FFIResult file_create(char *path);

FFIResult stderr_init(void);

FFIResult stderr_write(void *stderr_ptr, FFIByteArray array_buffer);

FFIResult stderr_write_all(void *stderr_ptr, FFIByteArray array_buffer);

FFIResult stderr_flush(void *stderr_ptr);

void free_stderr(void *stderr_ptr);

FFIResult stdin_init(void);

FFIResult stdin_read(void *stdin_ptr, FFIByteArray array_buffer);

FFIResult stdin_read_to_end(void *stdin_ptr);

void free_stdin(void *stdin_ptr);

FFIResult stdout_init(void);

FFIResult stdout_write(void *stdout_ptr, FFIByteArray array_buffer);

FFIResult stdout_write_all(void *stdout_ptr, FFIByteArray array_buffer);

FFIResult stdout_flush(void *stdout_ptr);

void free_stdout(void *stdout_ptr);

#endif /* RIO_H */
