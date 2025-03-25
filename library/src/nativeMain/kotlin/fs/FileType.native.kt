package fs

import rio.FFIFileType

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

fun FFIFileType.toFileType(): FileType {
    return when (this) {
        FFIFileType.IsDirectory -> FileType.Directory
        FFIFileType.IsFile -> FileType.File
        FFIFileType.IsSymlink -> FileType.Symlink
        FFIFileType.Other -> FileType.Other
        else -> throw Exception("Unknown file type")
    }
}
