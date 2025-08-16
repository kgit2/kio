package fs

import exception.UnknownFFIError
import handleError
import kotlinx.cinterop.CValue
import kotlinx.cinterop.memScoped
import kotlinx.cinterop.useContents
import rio.*
import kotlin.native.ref.createCleaner

actual class Metadata private actual constructor() {
    private var internal: CValue<FFIHandler>? = null

    private val cleaner = createCleaner(internal) { handle ->
        handle?.apply {
            memScoped {
                free_metadata(ptr)
            }
        }
    }

    constructor(handle: CValue<FFIHandler>) : this() {
        internal = handle
    }

    actual fun fileType(): FileType = memScoped {
        return metadata_file_type(internal!!.ptr).useContents {
            when (tag) {
                rio.FFIResult_Tag.Ok -> ok.file_type.toFileType()
                rio.FFIResult_Tag.Err -> throw handleError(err)
                else -> throw UnknownFFIError()
            }
        }
    }

    actual fun len(): ULong = memScoped {
        return metadata_len(internal!!.ptr).useContents {
            when (tag) {
                rio.FFIResult_Tag.Ok -> ok.u_int64
                rio.FFIResult_Tag.Err -> throw handleError(err)
                else -> throw UnknownFFIError()
            }
        }
    }

    actual fun readonly(): Boolean = memScoped {
        return metadata_readonly(internal!!.ptr).useContents {
            when (tag) {
                rio.FFIResult_Tag.Ok -> ok.boolean
                rio.FFIResult_Tag.Err -> throw handleError(err)
                else -> throw UnknownFFIError()
            }
        }
    }

    actual fun setReadonly(value: Boolean) = memScoped {
        metadata_set_readonly(internal!!.ptr, value).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> Unit
                FFIResult_Tag.Err -> throw handleError(err)
                else -> throw UnknownFFIError()
            }
        }
    }
}
