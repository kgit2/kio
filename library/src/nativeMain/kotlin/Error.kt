import exception.IOException
import rio.FFIValue

fun handleError(error: FFIValue): Exception {
    return IOException(error.string.toKString())
}
