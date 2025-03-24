package fs

import handleError
import kotlinx.cinterop.*
import path.Path
import rio.*
import kotlin.native.ref.createCleaner

actual class DirEntry private actual constructor(): AutoCloseable {
    private var internal: CValue<FFIHandle>? = null

    private val cleaner = createCleaner(internal) { handle ->
        handle?.apply {
            memScoped {
                free_dir_entry(ptr)
            }
        }
    }

    constructor(handle: CValue<FFIHandle>) : this() {
        internal = handle
    }

    fun path(): Path = memScoped {
        return dir_entry_path(internal!!.ptr).useContents {
            when (tag) {
                rio.FFIResult_Tag.Ok -> Path(cValue<FFIHandle> {
                    this.index = ok.handle.index
                    this.handle_type = ok.handle.handle_type
                })
                rio.FFIResult_Tag.Err -> throw handleError(err.string)
                else -> throw Exception("Unknown error")
            }
        }
    }

    fun fileType(): FileType = memScoped {
        dir_entry_file_type(internal!!.ptr).useContents {
            when (tag) {
                rio.FFIResult_Tag.Ok -> when (ok.file_type) {
                    FFIFileType.IsDirectory -> FileType.Directory
                    FFIFileType.IsFile -> FileType.File
                    FFIFileType.IsSymlink -> FileType.Symlink
                    FFIFileType.Other -> FileType.Other
                    else -> throw Exception("Unknown file type")
                }
                rio.FFIResult_Tag.Err -> throw handleError(err.string)
                else -> throw Exception("Unknown error")
            }
        }
    }

    override fun close() {
        internal?.apply {
            memScoped {
                free_dir_entry(ptr)
            }
        }
        internal = null
    }

    // override fun toString(): String {
    //     return "${fileType()} ${path()}"
    // }

    companion object {
        enum class FileType {
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
    }
}
