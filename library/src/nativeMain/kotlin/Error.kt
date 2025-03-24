import rio.FFIString

fun handleError(error: FFIString): Exception {
    return Exception(error.toKString())
}
