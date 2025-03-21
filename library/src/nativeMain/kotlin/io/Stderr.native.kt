package io

import handleError
import kotlinx.cinterop.*
import rio.*
import kotlin.native.ref.createCleaner

actual object Stderr : Write {
    val internal: Lazy<CValue<FFIHandle>> = lazy {
        stderr_init().useContents {
            val handle = ok.handle
            return@lazy cValue<FFIHandle> {
                index = handle.index
                handle_type = handle.handle_type
            }
        }
    }

    val cleaner = createCleaner(internal) {
        if (internal.isInitialized()) {
            free_stdin(internal.value)
        }
    }

    override fun write(buf: ByteArray, len: Int): Result<Int> = memScoped {
        if (buf.isEmpty()) {
            Result.success(0)
        }

        if (buf.size < len) {
            return Result.failure(Exception("buf size is less than len"))
        }

        val buffer = cValue<FFIBytes> {
            this.buffer = buf.asUByteArray().refTo(0).getPointer(this@memScoped)
            this.len = len.convert()
            this.capacity = buf.size.convert()
        }

        stderr_write(Stdout.internal.value, buffer).useContents {
            when (tag) {
                rio.FFIResult_Tag.Ok -> Result.success(ok.u_int64.convert())
                rio.FFIResult_Tag.Err -> Result.failure(handleError(err.string))
                else -> Result.failure(Exception("Unknown error"))
            }
        }
    }

    override fun writeAll(buf: ByteArray): Result<Unit> = memScoped {
        if (buf.isEmpty()) {
            return Result.success(Unit)
        }

        val buffer = cValue<FFIBytes> {
            buffer = buf.asUByteArray().refTo(0).getPointer(this@memScoped)
            len = buf.size.convert()
            capacity = buf.size.convert()
        }

        stderr_write_all(Stdout.internal.value, buffer).useContents {
            when (tag) {
                rio.FFIResult_Tag.Ok -> Result.success(Unit)
                rio.FFIResult_Tag.Err -> Result.failure(handleError(err.string))
                else -> Result.failure(Exception("Unknown error"))
            }
        }
    }

    override fun flush(): Result<Unit> {
        return stderr_flush(internal.value).useContents {
            when (tag) {
                rio.FFIResult_Tag.Ok -> Result.success(Unit)
                rio.FFIResult_Tag.Err -> Result.failure(handleError(err.string))
                else -> Result.failure(Exception("Unknown error"))
            }
        }
    }
}
