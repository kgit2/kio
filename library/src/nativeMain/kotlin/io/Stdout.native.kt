package io

import handleError
import kotlinx.cinterop.*
import rio.*
import toCValue
import toFFIBytes
import kotlin.native.ref.createCleaner

actual object Stdout : Write {
    val internal: Lazy<CValue<FFIHandle>> = lazy {
        stdout_init().useContents {
            when (tag) {
                rio.FFIResult_Tag.Ok -> ok.handle.toCValue()
                rio.FFIResult_Tag.Err -> throw handleError(err.string)
                else -> throw Exception("Unknown error")
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

    override fun write(buf: ByteArray, len: Int): Result<Int> = memScoped {
        if (buf.isEmpty()) {
            return Result.success(0)
        }

        if (buf.size < len) {
            return Result.failure(Exception("buf size is less than len"))
        }

        stdout_write(internal.value.ptr, buf.toFFIBytes(this)).useContents {
            when (tag) {
                rio.FFIResult_Tag.Ok -> Result.success(ok.u_size.convert())
                rio.FFIResult_Tag.Err -> Result.failure(handleError(err.string))
                else -> Result.failure(Exception("Unknown error"))
            }
        }
    }

    override fun writeAll(buf: ByteArray): Result<Unit> = memScoped {
        if (buf.isEmpty()) {
            return Result.success(Unit)
        }

        return stdout_write_all(internal.value.ptr, buf.toFFIBytes(this)).useContents {
            when (tag) {
                rio.FFIResult_Tag.Ok -> Result.success(Unit)
                rio.FFIResult_Tag.Err -> Result.failure(handleError(err.string))
                else -> Result.failure(Exception("Unknown error"))
            }
        }
    }

    override fun flush(): Result<Unit> = memScoped {
        return stdout_flush(internal.value.ptr).useContents {
            when (tag) {
                rio.FFIResult_Tag.Ok -> Result.success(Unit)
                rio.FFIResult_Tag.Err -> Result.failure(handleError(err.string))
                else -> Result.failure(Exception("Unknown error"))
            }
        }
    }
}
