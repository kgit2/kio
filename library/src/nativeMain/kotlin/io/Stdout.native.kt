package io

import handleError
import kotlinx.cinterop.*
import rio.*
import kotlin.native.ref.createCleaner

actual object Stdout : Write {
    val handle: Lazy<CValue<FFIHandle>> = lazy {
        stdout_init().useContents {
            val handle = ok.handle
            return@lazy cValue<FFIHandle> {
                index = handle.index
                handle_type = handle.handle_type
            }
        }
    }

    val cleaner = createCleaner(handle) {
        if (handle.isInitialized()) {
            free_stdin(handle.value)
        }
    }

    override fun write(buf: ByteArray, len: Int): Result<Int> = memScoped {
        if (buf.isEmpty()) {
            return Result.success(0)
        }

        if (buf.size < len) {
            return Result.failure(Exception("buf size is less than len"))
        }

        val buffer = cValue<FFIBytes> {
            buffer = buf.asUByteArray().refTo(0).getPointer(this@memScoped)
            this.len = len.toULong()
            capacity = buf.size.toULong()
        }

        stdout_write(handle.value, buffer).useContents {
            when (tag) {
                rio.FFIResult_Tag.Ok -> Result.success(ok.u_long.toInt())
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
            len = buf.size.toULong()
            capacity = buf.size.toULong()
        }

        return stdout_write_all(handle.value, buffer).useContents {
            when (tag) {
                rio.FFIResult_Tag.Ok -> Result.success(Unit)
                rio.FFIResult_Tag.Err -> Result.failure(handleError(err.string))
                else -> Result.failure(Exception("Unknown error"))
            }
        }
    }

    override fun flush(): Result<Unit> {
        return stdout_flush(handle.value).useContents {
            when (tag) {
                rio.FFIResult_Tag.Ok -> Result.success(Unit)
                rio.FFIResult_Tag.Err -> Result.failure(handleError(err.string))
                else -> Result.failure(Exception("Unknown error"))
            }
        }
    }
}
