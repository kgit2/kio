package util

fun checkFromIndexSize(fromIndex: Int, size: Int, length: Int): Result<Int> {
    if (fromIndex < 0 || size < 0 || fromIndex > length - size) {
        return Result.failure(
            IndexOutOfBoundsException(
                "Range [fromIndex=$fromIndex, size=$size) out of bounds for length=$length"
            )
        )
    }
    return Result.success(fromIndex)
}
