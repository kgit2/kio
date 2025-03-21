package io

import handleError
import kotlinx.cinterop.*
import rio.*
import rio.FFIResult_Tag.*
import kotlin.native.ref.createCleaner

actual object Stdin : Read {
    private val internal: Lazy<CValue<FFIHandle>> = lazy {
        stdin_init().useContents {
            val handle = ok.handle
            return@lazy cValue<FFIHandle> {
                index = handle.index
                handle_type = handle.handle_type
            }
        }
    }

    val cleaner = createCleaner(internal) { handle ->
        if (handle.isInitialized()) {
            free_stdin(handle.value)
        }
    }

    override fun read(buf: ByteArray, len: Int): Result<Int> = memScoped {
        if (buf.isEmpty()) {
            return Result.failure(Exception("buf is empty"))
        }

        if (buf.size < len) {
            return Result.failure(Exception("buf size is less than len"))
        }

        // 获取 ByteArray 指针
        val buffer = cValue<FFIBytes> {
            buffer = buf.asUByteArray().refTo(0).getPointer(this@memScoped)
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

    override fun readToEnd(buf: MutableList<UByte>): Result<Int> = memScoped {
        val result = stdin_read_to_end(internal.value.ptr)
        return result.useContents {
            when (tag) {
                Ok -> {
                    val size: Int = ok.bytes.len.convert()
                    val data = ok.bytes.buffer
                    for (i in 0 until size) {
                        data?.get(i)?.toUByte()?.let {
                            buf.add(it)
                        }
                    }
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
