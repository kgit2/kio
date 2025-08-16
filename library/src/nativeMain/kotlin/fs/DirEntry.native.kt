package fs

import exception.UnknownFFIError
import handleError
import kotlinx.cinterop.*
import path.Path
import rio.*
import toCValue
import kotlin.native.ref.createCleaner

actual class DirEntry private actual constructor() : AutoCloseable {
    private var internal: CValue<FFIHandler>? = null

    private val cleaner = createCleaner(internal) { handle ->
        handle?.apply {
            memScoped {
                free_dir_entry(ptr)
            }
        }
    }

    constructor(handle: CValue<FFIHandler>) : this() {
        internal = handle
    }

    actual fun fileType(): FileType = memScoped {
        dir_entry_file_type(internal!!.ptr).useContents {
            when (tag) {
                rio.FFIResult_Tag.Ok -> ok.file_type.toFileType()
                rio.FFIResult_Tag.Err -> throw handleError(err)
                else -> throw UnknownFFIError()
            }
        }
    }

    actual fun path(): Path = memScoped {
        return dir_entry_path(internal!!.ptr).useContents {
            when (tag) {
                rio.FFIResult_Tag.Ok -> Path(cValue<FFIHandler> {
                    this.index = ok.handle.index
                    this.handle_type = ok.handle.handle_type
                })

                rio.FFIResult_Tag.Err -> throw handleError(err)
                else -> throw UnknownFFIError()
            }
        }
    }

    actual fun metadata(): Metadata = memScoped {
        return dir_entry_metadata(internal!!.ptr).useContents {
            when (tag) {
                rio.FFIResult_Tag.Ok -> Metadata(ok.handle.toCValue())
                rio.FFIResult_Tag.Err -> throw handleError(err)
                else -> throw UnknownFFIError()
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

    override fun toString(): String {
        return "${fileType()} ${path()}"
    }
}
