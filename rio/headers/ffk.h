#ifndef FFK_H
#define FFK_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum FFIHandleType {
  Stdin,
  Stdout,
  Stderr,
  Path,
  File,
  Metadata,
  ReadDir,
  DirEntry,
} FFIHandleType;

typedef struct FFIByteArray {
  uint8_t *buffer;
  uintptr_t len;
  uintptr_t capacity;
} FFIByteArray;

typedef struct FFIHandle {
  uint64_t index;
  enum FFIHandleType handle_type;
} FFIHandle;

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
  Handle,
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
      struct FFIHandle handle;
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

void free_byte_array(struct FFIByteArray buffer);

/**
 * # Safety
 */
void free_string(char *string_ptr);

bool is_oK(const struct FFIResult *self);

bool is_err(const struct FFIResult *self);

struct FFIValue unwrap(struct FFIResult self);

struct FFIValue unwrap_err(struct FFIResult self);

#endif  /* FFK_H */
