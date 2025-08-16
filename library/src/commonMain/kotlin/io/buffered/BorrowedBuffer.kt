package io.buffered

class BorrowedBuffer(
    private val buffer: SlicedByteArray
) {
    // Rust: `init_len`
    private var initialized: Int = 0

    // Rust: `len()` == `init_len()`
    val len: Int get() = initialized

    // Rust: `capacity()`
    val capacity: Int get() = buffer.size

    // Rust: `unfilled().remaining()`
    val remaining: Int get() = buffer.size - initialized

    /**
     * Rust: `set_init(n)` — 标记前 n 个字节为已初始化（unsafe 外部写入）
     */
    fun setInit(n: Int) {
        require(n in 0..buffer.size) { "Init length out of bounds: $n" }
        initialized = n
    }

    /**
     * Rust: `init_len()` — 返回已初始化的字节数
     */
    fun initLen(): Int = initialized

    /**
     * Rust: `unfilled()` — 返回一个可供写入的 buffer 视图
     */
    fun unfilledSlice(): SlicedByteArray =
        buffer.slice(initialized until buffer.size)

    /**
     * Rust: `filled()` — 返回前面初始化过的 buffer 视图
     */
    fun filledSlice(): SlicedByteArray =
        buffer.slice(0 until initialized)

    /**
     * Rust: 更新初始化长度（写入之后）
     */
    fun advanceInit(count: Int) {
        require(initialized + count <= buffer.size) {
            "Advance init out of bounds: $initialized + $count > ${buffer.size}"
        }
        initialized += count
    }

    /**
     * Rust: `as_mut()` — 返回整个 buffer 的可写视图
     */
    fun fullSlice(): SlicedByteArray = buffer

    override fun toString(): String =
        "BorrowedBuffer(len=$len, capacity=$capacity, remaining=$remaining)"
}
