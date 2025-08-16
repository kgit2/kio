package io

import exception.BufferSizeException
import exception.EmptyBufferException
import exception.UnknownFFIError
import handleError
import io.buffered.SlicedByteArray
import kotlinx.cinterop.*
import rio.*
import rio.FFIResult_Tag.Err
import rio.FFIResult_Tag.Ok
import kotlin.native.ref.createCleaner

actual object Stdin : Read {
    private var internal: Lazy<CValue<FFIHandler>> = lazy {
        stdin_init().useContents {
            when (tag) {
                Ok -> cValue<FFIHandler> {
                    this.index = ok.handle.index
                    this.handle_type = ok.handle.handle_type
                }
                Err -> throw handleError(err)
                else -> throw UnknownFFIError()
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

    override fun read(buf: SlicedByteArray, offset: Int, len: Int): Int = memScoped {
        if (buf.isEmpty()) {
            throw EmptyBufferException()
        }

        if (buf.size < len) {
            throw BufferSizeException()
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
                Ok -> ok.u_size.convert()
                Err -> throw handleError(err)
                else -> throw UnknownFFIError()
            }
        }
    }

    override fun readToEnd(buf: MutableList<UByte>, offset: Int): Int = memScoped {
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

                    size
                }

                Err -> throw handleError(err)

                else -> throw UnknownFFIError()
            }
        }
    }
}
