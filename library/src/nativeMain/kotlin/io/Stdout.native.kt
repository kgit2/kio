package io

import exception.BufferSizeException
import exception.UnknownFFIError
import handleError
import io.buffered.SlicedByteArray
import kotlinx.cinterop.*
import rio.*
import toCValue
import toFFIBytes
import kotlin.native.ref.createCleaner

actual object Stdout : Write {
    val internal: Lazy<CValue<FFIHandler>> = lazy {
        stdout_init().useContents {
            when (tag) {
                rio.FFIResult_Tag.Ok -> ok.handle.toCValue()
                rio.FFIResult_Tag.Err -> throw handleError(err)
                else -> throw UnknownFFIError()
            }
        }
    }

    val cleaner = createCleaner(internal) {
        if (internal.isInitialized()) {
            memScoped {
                free_stdin(internal.value.ptr)
            }
        }
    }

    override fun write(buf: SlicedByteArray, offset: Int, len: Int): Int = memScoped {
        if (buf.isEmpty()) {
            return 0
        }

        if (buf.size < len) {
            throw BufferSizeException()
        }

        stdout_write(
            internal.value.ptr,
            buf.slice(offset, len).toFFIBytes(this)
        ).useContents {
            when (tag) {
                rio.FFIResult_Tag.Ok -> ok.u_size.convert()
                rio.FFIResult_Tag.Err -> throw handleError(err)
                else -> throw UnknownFFIError()
            }
        }
    }

    override fun writeAll(buf: SlicedByteArray, offset: Int) = memScoped {
        if (buf.isEmpty()) return@memScoped

        stdout_write_all(
            internal.value.ptr,
            buf.slice(offset, buf.size).toFFIBytes(this)
        ).useContents {
            when (tag) {
                rio.FFIResult_Tag.Ok -> return@memScoped
                rio.FFIResult_Tag.Err -> throw handleError(err)
                else -> throw UnknownFFIError()
            }
        }
    }

    override fun flush() = memScoped {
        stdout_flush(internal.value.ptr).useContents {
            when (tag) {
                rio.FFIResult_Tag.Ok -> return@memScoped
                rio.FFIResult_Tag.Err -> throw handleError(err)
                else -> throw UnknownFFIError()
            }
        }
    }
}
