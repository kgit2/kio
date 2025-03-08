package io

import kotlinx.cinterop.*
import rio.*
import kotlin.native.ref.createCleaner

actual object Stdout : Write {
    val internalPtr: Lazy<COpaquePointer?> = lazy {
        stdout_init().useContents { ok.c_opaque_pointer }
    }

    val cleaner = createCleaner(internalPtr) {
        if (it.isInitialized() && it.value != null) {
            free_stdout(it.value)
        }
    }

    override fun write(buf: ByteArray, len: UInt): Result<UInt> = memScoped {
        if (buf.isEmpty()) {
            return Result.success(0u)
        }

        if (buf.size < len.toInt()) {
            return Result.failure(Exception("buf size is less than len"))
        }

        val arrayBuffer = cValue<FFIByteArray> {
            buffer = buf.asUByteArray().refTo(0).getPointer(this@memScoped)
            this.len = len.toULong()
            capacity = buf.size.toULong()
        }

        stdout_write(internalPtr.value, arrayBuffer).useContents {
            when (tag) {
                rio.FFIResult_Tag.Ok -> Result.success(ok.u_long.toUInt())
                rio.FFIResult_Tag.Err -> {
                    val errorMessage = err.string?.toKStringFromUtf8()
                    free_string(err.string)
                    Result.failure(Exception(errorMessage))
                }
                else -> Result.failure(Exception("Unknown error"))
            }
        }
    }

    override fun write_all(buf: ByteArray): Result<Unit> = memScoped {
        if (buf.isEmpty()) {
            return Result.success(Unit)
        }

        val arrayBuffer = cValue<FFIByteArray> {
            buffer = buf.asUByteArray().refTo(0).getPointer(this@memScoped)
            len = buf.size.toULong()
            capacity = buf.size.toULong()
        }

        return stdout_write_all(internalPtr.value, arrayBuffer).useContents {
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
        return stdout_flush(internalPtr.value).useContents {
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
