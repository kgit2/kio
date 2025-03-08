#ifndef RIO_H
#define RIO_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct FFIByteArray {
  uint8_t *buffer;
  uintptr_t len;
  uintptr_t capacity;
} FFIByteArray;

typedef enum FFIValue_Tag {
  Int8,
  UInt8,
  Short,
  UShort,
  Int32,
  UInt32,
  Int64,
  UInt64,
  Long,
  ULong,
  Float,
  Double,
  Boolean,
  String,
  Array,
  COpaquePointer,
  Unit,
} FFIValue_Tag;

typedef struct FFIValue {
  FFIValue_Tag tag;
  union {
    struct {
      int8_t int8;
    };
    struct {
      uint8_t u_int8;
    };
    struct {
      int16_t short_;
    };
    struct {
      uint16_t u_short;
    };
    struct {
      int32_t int32;
    };
    struct {
      uint32_t u_int32;
    };
    struct {
      int64_t int64;
    };
    struct {
      uint64_t u_int64;
    };
    struct {
      intptr_t long_;
    };
    struct {
      uintptr_t u_long;
    };
    struct {
      float float_;
    };
    struct {
      double double_;
    };
    struct {
      bool boolean;
    };
    struct {
      char *string;
    };
    struct {
      struct FFIByteArray array;
    };
    struct {
      void *c_opaque_pointer;
    };
  };
} FFIValue;

typedef enum FFIResult_Tag {
  Ok,
  Err,
} FFIResult_Tag;

typedef struct FFIResult {
  FFIResult_Tag tag;
  union {
    struct {
      struct FFIValue ok;
    };
    struct {
      struct FFIValue err;
    };
  };
} FFIResult;

struct FFIResult stderr_init(void);

struct FFIResult stderr_write(void *stderr_ptr, struct FFIByteArray array_buffer);

struct FFIResult stderr_write_all(void *stderr_ptr, struct FFIByteArray array_buffer);

struct FFIResult stderr_flush(void *stderr_ptr);

void free_stderr(void *stderr_ptr);

struct FFIResult stdin_init(void);

struct FFIResult stdin_read(void *stdin_ptr, struct FFIByteArray array_buffer);

struct FFIResult stdin_read_to_end(void *stdin_ptr);

void free_stdin(void *stdin_ptr);

struct FFIResult stdout_init(void);

struct FFIResult stdout_write(void *stdout_ptr, struct FFIByteArray array_buffer);

struct FFIResult stdout_write_all(void *stdout_ptr, struct FFIByteArray array_buffer);

struct FFIResult stdout_flush(void *stdout_ptr);

void free_stdout(void *stdout_ptr);

/**
 * # Safety
 */
void free_array_buffer(struct FFIByteArray array_buffer);

/**
 * # Safety
 */
void free_string(char *string_ptr);

bool is_oK(const struct FFIResult *self);

bool is_err(const struct FFIResult *self);

struct FFIValue unwrap(struct FFIResult self);

struct FFIValue unwrap_err(struct FFIResult self);

struct FFIResult file_open(char *path);

struct FFIResult file_create(char *path);

#endif /* RIO_H */
