#ifndef RIO_H
#define RIO_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

FFIResult file_open(char *path);

FFIResult file_create(char *path);

FFIResult file_read(FFIHandle file_handle, FFIByteArray array_buffer);

FFIResult file_read_to_end(FFIHandle file_handle);

FFIResult file_write(FFIHandle file_handle, FFIByteArray buffer);

FFIResult file_write_all(FFIHandle file_handle, FFIByteArray buffer);

FFIResult file_flush(FFIHandle file_handle);

FFIResult stderr_init(void);

FFIResult stderr_write(FFIHandle stderr_handle, FFIByteArray array_buffer);

FFIResult stderr_write_all(FFIHandle stderr_handle, FFIByteArray array_buffer);

FFIResult stderr_flush(FFIHandle stderr_handle);

void free_stderr(FFIHandle stderr_handle);

FFIResult stdin_init(void);

FFIResult stdin_read(FFIHandle stdin_handle, FFIByteArray array_buffer);

FFIResult stdin_read_to_end(FFIHandle stdin_handle);

void free_stdin(FFIHandle stdin_handle);

FFIResult stdout_init(void);

FFIResult stdout_write(FFIHandle stdout_handle, FFIByteArray array_buffer);

FFIResult stdout_write_all(FFIHandle stdout_handle, FFIByteArray array_buffer);

FFIResult stdout_flush(FFIHandle stdout_handle);

void free_stdout(FFIHandle stdout_handle);

#endif  /* RIO_H */
