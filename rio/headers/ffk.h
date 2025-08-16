#ifndef FFK_H
#define FFK_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum FFIFileType {
  IsFile,
  IsDirectory,
  IsSymlink,
  Other,
} FFIFileType;

typedef enum FFIHandlerType {
  Stdin,
  Stdout,
  Stderr,
  Path,
  File,
  Metadata,
  ReadDir,
  DirEntry,
} FFIHandlerType;

typedef struct FFIString {
  char *buffer;
  uintptr_t len;
} FFIString;

typedef struct FFIBytes {
  uint8_t *buffer;
  uintptr_t len;
  uintptr_t capacity;
} FFIBytes;

typedef struct FFIHandler {
  uint64_t index;
  enum FFIHandlerType handle_type;
} FFIHandler;

typedef struct FFIVec {
  struct FFIValue *items;
  uintptr_t len;
  uintptr_t capacity;
} FFIVec;

typedef enum FFIValue_Tag {
  Int8,
  UInt8,
  Short,
  UShort,
  Int32,
  UInt32,
  Int64,
  UInt64,
  ISize,
  USize,
  Float,
  Double,
  Boolean,
  String,
  Bytes,
  Handle,
  Vec,
  FileType,
  Error,
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
      intptr_t i_size;
    };
    struct {
      uintptr_t u_size;
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
      struct FFIString string;
    };
    struct {
      struct FFIBytes bytes;
    };
    struct {
      struct FFIHandler handle;
    };
    struct {
      struct FFIVec vec;
    };
    struct {
      enum FFIFileType file_type;
    };
    struct {
      struct FFIString error;
    };
  };
} FFIValue;

typedef enum FFIResult_Tag {
  Ok,
  Err,
  None,
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

bool is_ok(const struct FFIResult *self);

bool is_err(const struct FFIResult *self);

struct FFIValue result_unwrap(struct FFIResult self);

struct FFIValue unwrap_err(struct FFIResult self);

/**
 * # Safety
 */
void free_ffi_bytes(struct FFIBytes self);

/**
 * # Safety
 */
void free_ffi_string(struct FFIString self);

/**
 * # Safety
 */
void free_ffi_vec(struct FFIVec self);

#endif  /* FFK_H */
