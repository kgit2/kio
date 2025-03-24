package fs

actual enum class FileType {
    Directory,
    File,
    Symlink,
    Other,
    ;

    override fun toString(): String {
        return when (this) {
            Directory -> "[D]"
            File -> "[F]"
            Symlink -> "[S]"
            Other -> "[O]"
        }
    }
}
