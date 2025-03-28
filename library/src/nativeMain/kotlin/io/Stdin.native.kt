package io

import handleError
import kotlinx.cinterop.*
import rio.*
import rio.FFIResult_Tag.*
import kotlin.native.ref.createCleaner

actual object Stdin : Read {
    private var internal: Lazy<CValue<FFIHandle>> = lazy {
        stdin_init().useContents {
            when (tag) {
                Ok -> cValue<FFIHandle> {
                    this.index = ok.handle.index
                    this.handle_type = ok.handle.handle_type
                }
                Err -> throw handleError(err.string)
                else -> throw Exception("Unknown error")
            }
        }
    }

    val cleaner = createCleaner(internal) { handle ->
        if (handle.isInitialized()) {
            memScoped {
                free_stdin(handle.value.ptr)
            }
        }
    }

    override fun read(buf: ByteArray, offset: Int, len: Int): Result<Int> = memScoped {
        if (buf.isEmpty()) {
            return Result.failure(Exception("buf is empty"))
        }

        if (buf.size < len) {
            return Result.failure(Exception("buf size is less than len"))
        }

        // 获取 ByteArray 指针
        val buffer = cValue<FFIBytes> {
            buffer = buf.asUByteArray().refTo(offset).getPointer(this@memScoped)
            this.len = len.convert()
            this.capacity = buf.size.convert()
        }

        // 调用 Rust FFI 方法
        val result: CValue<FFIResult> = stdin_read(internal.value.ptr, buffer)
        result.useContents {
            when (tag) {
                Ok -> Result.success(ok.u_size.convert())
                Err -> Result.failure(handleError(err.string))
                else -> Result.failure(Exception("Unknown error"))
            }
        }
    }

    override fun readToEnd(buf: MutableList<UByte>, offset: Int): Result<Int> = memScoped {
        val result = stdin_read_to_end(internal.value.ptr)
        return result.useContents {
            when (tag) {
                Ok -> {
                    val size: Int = ok.bytes.len.convert()
                    val data = ok.bytes.buffer
                    buf.addAll(offset, List(size) {
                        data?.get(it)?.toUByte()
                    }.filterNotNull())
                    val bytes = cValue<FFIBytes> {
                        buffer = ok.bytes.buffer
                        len = ok.bytes.len
                        capacity = ok.bytes.capacity
                    }
                    free_ffi_bytes(bytes)

                    Result.success(size)
                }

                Err -> Result.failure(handleError(err.string))

                else -> Result.failure(Exception("Unknown error"))
            }
        }
    }
}
