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

actual object Stderr : Write {
    private val internal: Lazy<CValue<FFIHandler>> = lazy {
        stderr_init().useContents {
            when (tag) {
                rio.FFIResult_Tag.Ok -> ok.handle.toCValue()
                rio.FFIResult_Tag.Err -> throw handleError(err)
                else -> throw UnknownFFIError()
            }
        }
    }

    val cleaner = createCleaner(internal) { internal ->
        if (internal.isInitialized()) {
            memScoped {
                free_stdin(internal.value.ptr)
            }
        }
    }

    override fun write(buf: SlicedByteArray, offset: Int, len: Int): Int = memScoped {
        if (buf.isEmpty()) return@memScoped 0

        if (buf.size < len) throw BufferSizeException()

        stderr_write(
            Stdout.internal.value.ptr,
            buf.slice(offset, len).toFFIBytes(this)
        ).useContents {
            when (tag) {
                rio.FFIResult_Tag.Ok -> return ok.u_int64.convert()
                rio.FFIResult_Tag.Err -> throw handleError(err)
                else -> throw UnknownFFIError()
            }
        }
    }

    override fun writeAll(buf: SlicedByteArray, offset: Int) = memScoped {
        if (buf.isEmpty()) return@memScoped

        stderr_write_all(
            Stdout.internal.value.ptr,
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
        stderr_flush(internal.value.ptr).useContents {
            when (tag) {
                rio.FFIResult_Tag.Ok -> return@memScoped
                rio.FFIResult_Tag.Err -> throw handleError(err)
                else -> throw UnknownFFIError()
            }
        }
    }
}
