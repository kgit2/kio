package fs

import exception.UnknownFFIError
import handleError
import kotlinx.cinterop.CValue
import kotlinx.cinterop.memScoped
import kotlinx.cinterop.useContents
import rio.*
import toCValue
import kotlin.native.ref.createCleaner

actual class ReadDir private actual constructor() : Iterator<DirEntry>, AutoCloseable {
    private var internal: CValue<FFIHandler>? = null
    private var current: DirEntry? = null

    private val cleaner = createCleaner(internal) { handle ->
        handle?.apply {
            memScoped {
                free_read_dir(ptr)
            }
        }
    }

    constructor(handle: CValue<FFIHandler>) : this() {
        internal = handle
    }

    override fun hasNext(): Boolean = memScoped {
        return internal?.run {
            read_dir_next(ptr)
        }?.run {
            useContents {
                when (tag) {
                    FFIResult_Tag.None -> false
                    FFIResult_Tag.Ok -> {
                        current = DirEntry(ok.handle.toCValue())
                        true
                    }
                    FFIResult_Tag.Err -> throw handleError(err)
                    else -> throw UnknownFFIError()
                }
            }
        } ?: false
    }

    override fun next(): DirEntry {
        return current!!
    }

    override fun close() {
        internal?.apply {
            memScoped {
                free_read_dir(ptr)
            }
        }
        internal = null
    }

    companion object {
        val count = kotlin.concurrent.AtomicInt(0)
    }
}

actual fun readDirSize(): ULong {
    return read_dir_size()
}

actual fun dirEntrySize(): ULong {
    return dir_entry_size()
}
