package io

import kotlinx.cinterop.*
import rio.*
import rio.FFIResult_Tag.*
import kotlin.native.ref.createCleaner

actual object Stdin : Read {
    private val internalPtr: Lazy<COpaquePointer?> = lazy {
        stdin_init().useContents { ok.c_opaque_pointer }
    }

    val cleaner = createCleaner(internalPtr) { ptr ->
        if (ptr.isInitialized() && ptr.value != null) {
            // 销毁 stdin
            free_stdin(ptr.value)
        }
    }

    override fun read(buf: ByteArray, len: UInt): Result<UInt> = memScoped {
        if (buf.isEmpty()) {
            return Result.failure(Exception("buf is empty"))
        }

        if (buf.size < len.toInt()) {
            return Result.failure(Exception("buf size is less than len"))
        }

        // 获取 ByteArray 指针
        val arrayBuffer = cValue<FFIByteArray> {
            buffer = buf.asUByteArray().refTo(0).getPointer(this@memScoped)
            this.len = len.toULong()
            this.capacity = buf.size.toULong()
        }

        // 调用 Rust FFI 方法
        val result: CValue<FFIResult> = stdin_read(internalPtr.value, arrayBuffer)
        result.useContents {
            when (tag) {
                Ok -> {
                    Result.success(ok.u_long.toUInt())
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

    override fun read_to_end(buf: MutableList<UByte>): Result<UInt> {
        val result = stdin_read_to_end(internalPtr.value)
        return result.useContents {
            when (tag) {
                Ok -> {
                    val size = ok.array.len.toUInt()
                    val data = ok.array.buffer
                    for (i in 0 until size.toInt()) {
                        data?.get(i)?.toUByte()?.let {
                            buf.add(it)
                        }
                    }
                    val arrayBuffer = cValue<FFIByteArray> {
                        buffer = ok.array.buffer
                        len = ok.array.len
                        capacity = ok.array.capacity
                    }
                    free_array_buffer(arrayBuffer)

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
