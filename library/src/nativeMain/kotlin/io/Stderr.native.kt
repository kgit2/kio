package io

import handleError
import kotlinx.cinterop.*
import rio.*
import toCValue
import toFFIBytes
import kotlin.native.ref.createCleaner

actual object Stderr : Write {
    private val internal: Lazy<CValue<FFIHandle>> = lazy {
        stderr_init().useContents {
            when (tag) {
                rio.FFIResult_Tag.Ok -> ok.handle.toCValue()
                rio.FFIResult_Tag.Err -> throw handleError(err.string)
                else -> throw Exception("Unknown error")
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

    override fun write(buf: ByteArray, offset: Int, len: Int): Result<Int> = memScoped {
        if (buf.isEmpty()) {
            Result.success(0)
        }

        if (buf.size < len) {
            return Result.failure(Exception("buf size is less than len"))
        }

        stderr_write(
            Stdout.internal.value.ptr,
            buf.sliceArray(IntRange(offset, len)).toFFIBytes(this)
        ).useContents {
            when (tag) {
                rio.FFIResult_Tag.Ok -> Result.success(ok.u_int64.convert())
                rio.FFIResult_Tag.Err -> Result.failure(handleError(err.string))
                else -> Result.failure(Exception("Unknown error"))
            }
        }
    }

    override fun writeAll(buf: ByteArray, offset: Int): Result<Unit> = memScoped {
        if (buf.isEmpty()) {
            return Result.success(Unit)
        }

        stderr_write_all(
            Stdout.internal.value.ptr,
            buf.sliceArray(offset ..< buf.size).toFFIBytes(this)
        ).useContents {
            when (tag) {
                rio.FFIResult_Tag.Ok -> Result.success(Unit)
                rio.FFIResult_Tag.Err -> Result.failure(handleError(err.string))
                else -> Result.failure(Exception("Unknown error"))
            }
        }
    }

    override fun flush(): Result<Unit> = memScoped {
        return stderr_flush(internal.value.ptr).useContents {
            when (tag) {
                rio.FFIResult_Tag.Ok -> Result.success(Unit)
                rio.FFIResult_Tag.Err -> Result.failure(handleError(err.string))
                else -> Result.failure(Exception("Unknown error"))
            }
        }
    }
}
