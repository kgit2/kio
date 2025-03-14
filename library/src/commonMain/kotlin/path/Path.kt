package path

class Path (
    val inner: String
) {
    constructor(path: Path) : this(path.inner)

    fun join(other: String): Path {
        return Path("$inner/$other")
    }

    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (other == null || this::class != other::class) return false

        other as Path

        return inner == other.inner
    }

    override fun hashCode(): Int {
        return inner.hashCode()
    }

}
