#ifndef RIO_H
#define RIO_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include "ffk.h"

FFIResult file_open(char *path);

FFIResult file_create(char *path);

FFIResult file_read(const FFIHandle *file_handle, FFIByteArray buffer);

FFIResult file_read_to_end(const FFIHandle *file_handle);

FFIResult file_write(const FFIHandle *file_handle, FFIByteArray buffer);

FFIResult file_write_all(const FFIHandle *file_handle, FFIByteArray buffer);

FFIResult file_flush(const FFIHandle *file_handle);

void free_file(const FFIHandle *file_handle);

FFIResult stderr_init(void);

FFIResult stderr_write(const FFIHandle *stderr_handle, FFIByteArray buffer);

FFIResult stderr_write_all(const FFIHandle *stderr_handle, FFIByteArray buffer);

FFIResult stderr_flush(const FFIHandle *stderr_handle);

void free_stderr(const FFIHandle *stderr_handle);

FFIResult stdin_init(void);

FFIResult stdin_read(const FFIHandle *stdin_handle, FFIByteArray buffer);

FFIResult stdin_read_to_end(const FFIHandle *stdin_handle);

void free_stdin(const FFIHandle *stdin_handle);

FFIResult stdout_init(void);

FFIResult stdout_write(const FFIHandle *stdout_handle, FFIByteArray buffer);

FFIResult stdout_write_all(const FFIHandle *stdout_handle, FFIByteArray buffer);

FFIResult stdout_flush(const FFIHandle *stdout_handle);

void free_stdout(const FFIHandle *stdout_handle);

FFIResult path_init(FFIByteArray buffer);

FFIResult path_push(const FFIHandle *handle, FFIByteArray buffer);

FFIResult path_pop(const FFIHandle *handle);

FFIResult path_set_file_name(const FFIHandle *handle, FFIByteArray buffer);

FFIResult path_set_extension(const FFIHandle *handle, FFIByteArray buffer);

FFIResult path_clear(const FFIHandle *handle);

FFIResult path_clone(const FFIHandle *handle);

FFIResult path_eq(const FFIHandle *handle, const FFIHandle *other);

FFIResult path_hash(const FFIHandle *handle);

FFIResult path_compare(const FFIHandle *handle, const FFIHandle *other);

FFIResult path_to_string(const FFIHandle *handle);

FFIResult path_to_string_lossy(const FFIHandle *handle);

FFIResult path_file_name(const FFIHandle *handle);

FFIResult path_extension(const FFIHandle *handle);

FFIResult path_parent(const FFIHandle *handle);

FFIResult path_exists(const FFIHandle *handle);

FFIResult path_is_file(const FFIHandle *handle);

FFIResult path_is_dir(const FFIHandle *handle);

FFIResult path_is_absolute(const FFIHandle *handle);

FFIResult path_is_relative(const FFIHandle *handle);

FFIResult path_canonicalize(const FFIHandle *handle);

FFIResult path_metadata(const FFIHandle *handle);

FFIResult path_read_dir(const FFIHandle *handle);

FFIResult path_components(const FFIHandle *handle);

#endif  /* RIO_H */
