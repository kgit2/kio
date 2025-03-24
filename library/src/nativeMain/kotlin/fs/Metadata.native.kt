package fs

import handleError
import kotlinx.cinterop.CValue
import kotlinx.cinterop.memScoped
import kotlinx.cinterop.useContents
import rio.*
import kotlin.native.ref.createCleaner

actual class Metadata private actual constructor() {
    private var internal: CValue<FFIHandle>? = null

    private val cleaner = createCleaner(internal) { handle ->
        handle?.apply {
            memScoped {
                free_metadata(ptr)
            }
        }
    }

    constructor(handle: CValue<FFIHandle>) : this() {
        internal = handle
    }

    actual fun fileType(): FileType = memScoped {
        return metadata_file_type(internal!!.ptr).useContents {
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

    actual fun len(): ULong = memScoped {
        return metadata_len(internal!!.ptr).useContents {
            when (tag) {
                rio.FFIResult_Tag.Ok -> ok.u_int64
                rio.FFIResult_Tag.Err -> throw handleError(err.string)
                else -> throw Exception("Unknown error")
            }
        }
    }

    actual fun readonly(): Boolean = memScoped {
        return metadata_readonly(internal!!.ptr).useContents {
            when (tag) {
                rio.FFIResult_Tag.Ok -> ok.boolean
                rio.FFIResult_Tag.Err -> throw handleError(err.string)
                else -> throw Exception("Unknown error")
            }
        }
    }

    actual fun setReadonly(value: Boolean) = memScoped {
        metadata_set_readonly(internal!!.ptr, value).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> Unit
                FFIResult_Tag.Err -> throw handleError(err.string)
                else -> throw Exception("Unknown error")
            }
        }
    }
}
