package io

import kotlinx.cinterop.*
import rio.*
import rio.FFIResult_Tag.*
import kotlin.native.ref.createCleaner

actual object Stdin : Read {
    private val handle: Lazy<CValue<FFIHandle>> = lazy {
        stdin_init().useContents {
            val handle = ok.handle
            return@lazy cValue<FFIHandle> {
                index = handle.index
                handle_type = handle.handle_type
            }
        }
    }

    val cleaner = createCleaner(handle) { handle ->
        if (handle.isInitialized()) {
            // 销毁 stdin
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
        val buffer = cValue<FFIByteArray> {
            buffer = buf.asUByteArray().refTo(0).getPointer(this@memScoped)
            this.len = len.toULong()
            this.capacity = buf.size.toULong()
        }

        // 调用 Rust FFI 方法
        val result: CValue<FFIResult> = stdin_read(handle.value.ptr, buffer)
        result.useContents {
            when (tag) {
                Ok -> {
                    Result.success(ok.u_long.toInt())
                }

                Err -> {
                    val errorMessage = err.string?.toKStringFromUtf8()
                    free_string(err.string)
                    Result.failure(Exception(errorMessage))
                }

                else -> {
                    Result.failure(Exception("Unknown error"))
                }
            }
        }
    }

    override fun readToEnd(buf: MutableList<UByte>): Result<Int> = memScoped {
        val result = stdin_read_to_end(handle.value.ptr)
        return result.useContents {
            when (tag) {
                Ok -> {
                    val size = ok.array.len.toInt()
                    val data = ok.array.buffer
                    for (i in 0 until size) {
                        data?.get(i)?.toUByte()?.let {
                            buf.add(it)
                        }
                    }
                    val buffer = cValue<FFIByteArray> {
                        buffer = ok.array.buffer
                        len = ok.array.len
                        capacity = ok.array.capacity
                    }
                    free_byte_array(buffer)

                    Result.success(size)
                }

                Err -> {
                    val errorMessage = err.string?.toKStringFromUtf8()
                    free_string(err.string)

                    Result.failure(Exception(errorMessage))
                }

                else -> {
                    Result.failure(Exception("Unknown error"))
                }
            }
        }
    }
}
