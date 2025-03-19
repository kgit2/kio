import kotlinx.cinterop.cValue
import kotlinx.cinterop.readBytes
import kotlinx.cinterop.toKString
import kotlinx.cinterop.useContents
import rio.FFIString
import rio.free_ffi_string

fun handleError(error: FFIString): Exception {
    val errorString = cValue<FFIString> {
        this.buffer = error.buffer
        this.len = error.len
    }
    val errorMessage = errorString.useContents {
        this.buffer?.readBytes(this.len.toInt())?.toKString()
    }
    free_ffi_string(errorString)

    return Exception(errorMessage)
}
