package fs

actual enum class FileType {
    Directory,
    File,
    Symlink,
    Other,
    ;
}
