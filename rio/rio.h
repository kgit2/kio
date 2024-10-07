#ifndef RIO_H
#define RIO_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct RioWriteRef {
  void *inner;
} RioWriteRef;

typedef struct RioReadRef {
  void *inner;
} RioReadRef;

typedef enum RioResult_Tag {
  UnitVariant,
  Int8Variant,
  UInt8Variant,
  ShortVariant,
  UShortVariant,
  Int32Variant,
  UInt32Variant,
  Int64Variant,
  UInt64Variant,
  LongVariant,
  ULongVariant,
  FloatVariant,
  DoubleVariant,
  BooleanVariant,
  ByteArrayVariant,
  StringVariant,
  RioWriteRefVariant,
  RioReadRefVariant,
  ErrorVariant,
} RioResult_Tag;

typedef struct RioResult {
  RioResult_Tag tag;
  union {
    struct {
      int8_t int8_variant;
    };
    struct {
      uint8_t u_int8_variant;
    };
    struct {
      int16_t short_variant;
    };
    struct {
      uint16_t u_short_variant;
    };
    struct {
      int32_t int32_variant;
    };
    struct {
      uint32_t u_int32_variant;
    };
    struct {
      int64_t int64_variant;
    };
    struct {
      uint64_t u_int64_variant;
    };
    struct {
      intptr_t long_variant;
    };
    struct {
      uintptr_t u_long_variant;
    };
    struct {
      float float_variant;
    };
    struct {
      double double_variant;
    };
    struct {
      bool boolean_variant;
    };
    struct {
      const int8_t *byte_array_variant;
    };
    struct {
      const char *string_variant;
    };
    struct {
      struct RioWriteRef rio_write_ref_variant;
    };
    struct {
      struct RioReadRef rio_read_ref_variant;
    };
    struct {
      const char *error_variant;
    };
  };
} RioResult;

void init_backtrace(void);

/**
 * # Safety
 * Will drop the string
 */
void drop_c_char(char *string);

/**
 * # Returns
 * - [`RioResult::ErrorVariant`] if `byte_array` is null
 * - [`RioResult::RioReadRefVariant`]::<[`RioRead::Stdin`]> otherwise
 */
struct RioResult read_ref_new_byte_array(int8_t *byte_array, uintptr_t size);

struct RioReadRef read_ref_new_stdin(void);

/**
 * # Returns
 * - [`RioResult::ErrorVariant`] if `file` is null
 * - [`RioResult::RioReadRefVariant`]::<[`RioRead::File`]> otherwise
 */
struct RioResult read_ref_new_child_stdout(void *child_stdout);

/**
 * # Returns
 * - [`RioResult::ErrorVariant`] if `file` is null
 * - [`RioResult::RioReadRefVariant`]::<[`RioRead::File`]> otherwise
 */
struct RioResult read_ref_new_child_stderr(void *child_stderr);

/**
 * # Returns
 * - [`RioResult::ErrorVariant`] if [`RioWriteRef::inner`] is null
 * - [`RioResult::RioReadRefVariant`]::<[`RioRead::BufReader`]> otherwise
 */
struct RioResult read_ref_new_buf_reader(struct RioReadRef self);

void read_ref_drop(struct RioReadRef self);

/**
 * # Returns
 * - [`RioResult::ErrorVariant`] if `buf` is null
 * - [`RioResult::UnitVariant`] otherwise
 */
struct RioResult read_ref_read(struct RioReadRef *self, int8_t *buf, uintptr_t size);

/**
 * # Returns
 * - [`RioResult::ByteArrayVariant`]::<[`std::ptr::null`]> if `size` is null
 * - [`RioResult::ByteArrayVariant`] if `read_to_string` was successful
 * - [`RioResult::ErrorVariant`] otherwise
 */
struct RioResult read_ref_read_to_string(struct RioReadRef *self, uintptr_t *size);

struct RioWriteRef write_ref_new_stdout(void);

struct RioWriteRef write_ref_new_stderr(void);

/**
 * # Safety
 * The caller must ensure that the byte_array is valid for the duration of the TypedWriteRef.
 * # Returns
 * - [`RioResult::ErrorVariant`] if `byte_array` is null
 * - [`RioResult::RioWriteRefVariant`]::<[`RioWrite::ByteArray`]> otherwise
 */
struct RioResult write_ref_new_byte_array(int8_t *byte_array, uintptr_t len);

/**
 * # Returns
 * - [`RioResult::ErrorVariant`] if `child_stdin` is null
 * - [`RioResult::RioWriteRefVariant`]::<[`RioWrite::File`]> otherwise
 */
struct RioResult write_ref_new_child_stdin(void *child_stdin);

/**
 * # Returns
 * - [`RioResult::ErrorVariant`] if [`RioWriteRef::inner`] is null
 * - [`RioResult::RioWriteRefVariant`]::<[`RioWrite::BufWriter`]> otherwise
 */
struct RioResult write_ref_new_buf_writer(struct RioWriteRef self);

/**
 * # Returns
 * - [`RioResult::ErrorVariant`] if [`RioWriteRef::inner`] is null
 * - [`RioResult::RioWriteRefVariant`]::<[`RioWrite::LineWriter`]> otherwise
 */
struct RioResult write_ref_new_line_writer(struct RioWriteRef self);

void write_ref_drop(struct RioWriteRef self);

/**
 * # Returns
 * - [`RioResult::ErrorVariant`] if [`RioWriteRef::inner`] is null
 * - [`RioResult::ErrorVariant`] if `buf` is null
 * - [`RioResult::UnitVariant`] if write was successful.
 */
struct RioResult write_ref_write(struct RioWriteRef *self, const int8_t *buf, uintptr_t size);

/**
 * # Returns
 * - [`RioResult::ErrorVariant`] if [`RioWriteRef::inner`] is null
 * - [`RioResult::UnitVariant`] if flush was successful.
 */
struct RioResult write_ref_flush(struct RioWriteRef *self);

/**
 * # Returns
 * - [`RioResult::ErrorVariant`] if [`RioWriteRef::inner`] is null
 * - [`RioResult::ErrorVariant`] if `buf` is null
 * - [`RioResult::UnitVariant`] if `write_all` was successful.
 */
struct RioResult write_ref_write_all(struct RioWriteRef *self, const char *buf, uintptr_t size);

#endif  /* RIO_H */
