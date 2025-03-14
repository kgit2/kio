package io

import kotlinx.cinterop.*
import rio.*
import kotlin.native.ref.createCleaner

actual object Stderr : Write {
    val handle: Lazy<CValue<FFIHandle>> = lazy {
        stderr_init().useContents {
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
            Result.success(0)
        }

        if (buf.size < len.toInt()) {
            return Result.failure(Exception("buf size is less than len"))
        }

        val buffer = cValue<FFIByteArray> {
            this.buffer = buf.asUByteArray().refTo(0).getPointer(this@memScoped)
            this.len = len.toULong()
            this.capacity = buf.size.toULong()
        }

        stderr_write(Stdout.handle.value, buffer).useContents {
            when (tag) {
                rio.FFIResult_Tag.Ok -> Result.success(ok.u_long.toInt())
                rio.FFIResult_Tag.Err -> {
                    val errorMessage = err.string?.toKStringFromUtf8()
                    free_string(err.string)
                    Result.failure(Exception(errorMessage))
                }

                else -> Result.failure(Exception("Unknown error"))
            }
        }
    }

    override fun writeAll(buf: ByteArray): Result<Unit> = memScoped {
        if (buf.isEmpty()) {
            return Result.success(Unit)
        }

        val buffer = cValue<FFIByteArray> {
            buffer = buf.asUByteArray().refTo(0).getPointer(this@memScoped)
            len = buf.size.toULong()
            capacity = buf.size.toULong()
        }

        stderr_write_all(Stdout.handle.value, buffer).useContents {
            when (tag) {
                rio.FFIResult_Tag.Ok -> Result.success(Unit)
                rio.FFIResult_Tag.Err -> {
                    val errorMessage = err.string?.toKStringFromUtf8()
                    free_string(err.string)
                    Result.failure(Exception(errorMessage))
                }

                else -> Result.failure(Exception("Unknown error"))
            }
        }
    }

    override fun flush(): Result<Unit> {
        return stderr_flush(handle.value).useContents {
            when (tag) {
                rio.FFIResult_Tag.Ok -> Result.success(Unit)
                rio.FFIResult_Tag.Err -> {
                    val errorMessage = err.string?.toKStringFromUtf8()
                    free_string(err.string)
                    Result.failure(Exception(errorMessage))
                }

                else -> Result.failure(Exception("Unknown error"))
            }
        }
    }
}
