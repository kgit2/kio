sealed class Result<T: Any, E: Any> {
    data class Ok<T: Any, E: Any>(val value: T) : Result<T, E>()
    data class Err<T: Any, E: Any>(val error: E) : Result<T, E>()

    fun isOk(): Boolean = this is Ok

    fun isErr(): Boolean = this is Err

    fun unwrap(): T = when (this) {
        is Ok -> value
        is Err -> throw RuntimeException("Called unwrap on an Err value: $error")
    }

    fun unwrapErr(): E = when (this) {
        is Ok -> throw RuntimeException("Called unwrapErr on an Ok value: $value")
        is Err -> error
    }

    fun expect(message: String): T = when (this) {
        is Ok -> value
        is Err -> throw RuntimeException(message)
    }

    fun expectErr(message: String): E = when (this) {
        is Ok -> throw RuntimeException(message)
        is Err -> error
    }

    fun <U: Any> map(transform: (T) -> U): Result<U, E> = when (this) {
        is Ok -> Ok(transform(value))
        is Err -> Err(error)
    }

    fun <U: Any> mapErr(transform: (E) -> U): Result<T, U> = when (this) {
        is Ok -> Ok(value)
        is Err -> Err(transform(error))
    }

    fun <U: Any> andThen(transform: (T) -> Result<U, E>): Result<U, E> = when (this) {
        is Ok -> transform(value)
        is Err -> Err(error)
    }
}
