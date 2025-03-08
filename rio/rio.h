#ifndef RIO_H
#define RIO_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct ArrayBuffer {
  uintptr_t len;
  uint8_t *buffer;
} ArrayBuffer;

typedef enum TypeWrapper_Tag {
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
} TypeWrapper_Tag;

typedef struct TypeWrapper {
  TypeWrapper_Tag tag;
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
      struct ArrayBuffer array;
    };
    struct {
      void *c_opaque_pointer;
    };
  };
} TypeWrapper;

typedef enum FFIResult_Tag {
  Ok,
  Err,
} FFIResult_Tag;

typedef struct FFIResult {
  FFIResult_Tag tag;
  union {
    struct {
      struct TypeWrapper ok;
    };
    struct {
      struct TypeWrapper err;
    };
  };
} FFIResult;

bool is_oK(const struct FFIResult *self);

bool is_err(const struct FFIResult *self);

struct TypeWrapper unwrap(struct FFIResult self);

struct TypeWrapper unwrap_err(struct FFIResult self);

struct FFIResult stderr_init(void);

struct FFIResult stderr_write(void *stderr_ptr, struct ArrayBuffer array_buffer);

struct FFIResult stderr_write_all(void *stderr_ptr, struct ArrayBuffer array_buffer);

struct FFIResult stderr_flush(void *stderr_ptr);

void free_stderr(void *stderr_ptr);

struct FFIResult stdin_init(void);

/**
 * # Safety
 */
struct FFIResult stdin_read(void *stdin_ptr, struct ArrayBuffer array_buffer);

/**
 * # Safety
 */
struct FFIResult stdin_read_to_end(void *stdin_ptr);

void free_stdin(void *stdin_ptr);

struct FFIResult stdout_init(void);

/**
 * # Safety
 */
struct FFIResult stdout_write(void *stdout_ptr, struct ArrayBuffer array_buffer);

/**
 * # Safety
 */
struct FFIResult stdout_write_all(void *stdout_ptr, struct ArrayBuffer array_buffer);

/**
 * # Safety
 */
struct FFIResult stdout_flush(void *stdout_ptr);

/**
 * # Safety
 */
void free_stdout(void *stdout_ptr);

/**
 * # Safety
 */
void free_array_buffer(struct ArrayBuffer array_buffer);

/**
 * # Safety
 */
void free_string(char *string_ptr);

#endif /* RIO_H */
