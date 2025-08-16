#ifndef RIO_H
#define RIO_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include "ffk.h"

FFIResult file_open(const FFIString *path);

FFIResult file_create(const FFIString *path);

FFIResult file_read(const FFIHandler *file_handle, FFIBytes *buffer);

FFIResult file_read_to_end(const FFIHandler *file_handle);

FFIResult file_write(const FFIHandler *file_handle, const FFIBytes *buffer);

FFIResult file_write_all(const FFIHandler *file_handle, const FFIBytes *buffer);

FFIResult file_flush(const FFIHandler *file_handle);

void free_file(const FFIHandler *file_handle);

FFIResult metadata_file_type(const FFIHandler *handle);

FFIResult metadata_is_dir(const FFIHandler *handle);

FFIResult metadata_is_file(const FFIHandler *handle);

FFIResult metadata_is_symlink(const FFIHandler *handle);

FFIResult metadata_len(const FFIHandler *handle);

FFIResult metadata_readonly(const FFIHandler *handle);

FFIResult metadata_set_readonly(const FFIHandler *handle, bool readonly);

FFIResult metadata_mode(const FFIHandler *handle);

FFIResult metadata_set_mode(const FFIHandler *handle, uint32_t mode);

void free_metadata(const FFIHandler *handle);

FFIResult read_dir_next(const FFIHandler *handle);

FFIResult read_dir_into_list(const FFIHandler *handle,
                             void (*error_callback)(FFIString, void*),
                             void *payload);

FFIResult dir_entry_path(const FFIHandler *handle);

FFIResult dir_entry_metadata(const FFIHandler *handle);

FFIResult dir_entry_file_type(const FFIHandler *handle);

FFIResult dir_entry_file_name(const FFIHandler *handle);

uint64_t read_dir_size(void);

uint64_t dir_entry_size(void);

void free_read_dir(const FFIHandler *handle);

void free_dir_entry(const FFIHandler *handle);

FFIResult stderr_init(void);

FFIResult stderr_write(const FFIHandler *stderr_handle, const FFIBytes *buffer);

FFIResult stderr_write_all(const FFIHandler *stderr_handle, const FFIBytes *buffer);

FFIResult stderr_flush(const FFIHandler *stderr_handle);

void free_stderr(const FFIHandler *stderr_handle);

FFIResult stdin_init(void);

FFIResult stdin_read(const FFIHandler *stdin_handle, FFIBytes *buffer);

FFIResult stdin_read_to_end(const FFIHandler *stdin_handle);

void free_stdin(const FFIHandler *stdin_handle);

FFIResult stdout_init(void);

FFIResult stdout_write(const FFIHandler *stdout_handle, const FFIBytes *buffer);

FFIResult stdout_write_all(const FFIHandler *stdout_handle, const FFIBytes *buffer);

FFIResult stdout_flush(const FFIHandler *stdout_handle);

void free_stdout(const FFIHandler *stdout_handle);

FFIResult path_init(const FFIString *buffer);

FFIResult path_cwd(void);

FFIResult path_push(const FFIHandler *handle, const FFIString *buffer);

FFIResult path_pop(const FFIHandler *handle);

FFIResult path_set_file_name(const FFIHandler *handle, const FFIString *buffer);

FFIResult path_set_extension(const FFIHandler *handle, const FFIString *buffer);

FFIResult path_clear(const FFIHandler *handle);

FFIResult path_clone(const FFIHandler *handle);

FFIResult path_eq(const FFIHandler *handle, const FFIHandler *other);

FFIResult path_hash(const FFIHandler *handle);

FFIResult path_compare(const FFIHandler *handle, const FFIHandler *other);

FFIResult path_to_string(const FFIHandler *handle);

FFIResult path_to_string_lossy(const FFIHandler *handle);

FFIResult path_file_name(const FFIHandler *handle);

FFIResult path_extension(const FFIHandler *handle);

FFIResult path_parent(const FFIHandler *handle);

FFIResult path_exists(const FFIHandler *handle);

FFIResult path_is_file(const FFIHandler *handle);

FFIResult path_is_dir(const FFIHandler *handle);

FFIResult path_is_absolute(const FFIHandler *handle);

FFIResult path_is_relative(const FFIHandler *handle);

FFIResult path_normalize(const FFIHandler *handle);

FFIResult path_canonicalize(const FFIHandler *handle);

FFIResult path_metadata(const FFIHandler *handle);

FFIResult path_read_dir(const FFIHandler *handle);

FFIResult path_components(const FFIHandler *handle);

void free_path(const FFIHandler *handle);

#endif  /* RIO_H */
