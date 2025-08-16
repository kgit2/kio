package exception

open class IOException(
    message: String? = null,
    cause: Throwable? = null
) : Exception(message, cause)

class EOFException(
    message: String = "Unexpected end of input"
) : IOException(message)

class FileNotFoundException(
    message: String = "File not found"
) : IOException(message)

class StreamCorruptedException(
    message: String = "Stream corrupted"
) : IOException(message)

class EmptyBufferException(
    message: String = "Buffer is empty"
) : IOException(message)

class BufferSizeException(
    message: String = "Buffer size is less than expected length"
) : IOException(message)

class UnknownFFIError(
    message: String = "Unknown FFI error"
) : IOException(message)

class UnknownFileType(
    message: String = "Unknown file type"
) : IOException(message)
