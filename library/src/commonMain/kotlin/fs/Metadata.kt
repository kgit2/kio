package fs

expect class Metadata private constructor() {
    fun fileType(): FileType
    fun len(): ULong
    fun readonly(): Boolean
    fun setReadonly(value: Boolean)
}
