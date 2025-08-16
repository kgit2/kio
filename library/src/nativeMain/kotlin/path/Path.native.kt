package path

import exception.UnknownFFIError
import fs.Metadata
import fs.ReadDir
import handleError
import kotlinx.cinterop.CValue
import kotlinx.cinterop.convert
import kotlinx.cinterop.memScoped
import kotlinx.cinterop.useContents
import rio.*
import toCValue
import toFFIString
import toKString
import toStringList

actual class Path(
    val internal: CValue<FFIHandle>,
) : Comparable<Path> {

    actual constructor(value: String) : this(memScoped {
        path_init(value.toFFIString(this))
            .useContents { ok.handle.toCValue() }
    })

    actual companion object {
        actual fun cwd(): Path {
            return path_cwd().useContents {
                when (tag) {
                    FFIResult_Tag.Ok -> Path(ok.handle.toCValue())
                    FFIResult_Tag.Err -> throw handleError(err)
                    else -> throw UnknownFFIError()
                }
            }
        }
    }

    actual fun push(value: String) = memScoped {
        path_push(internal.ptr, value.toFFIString(this)).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> Unit
                FFIResult_Tag.Err -> throw handleError(err)
                else -> throw UnknownFFIError()
            }
        }
    }

    actual fun pop(): Boolean = memScoped {
        return path_pop(internal.ptr).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> ok.boolean
                FFIResult_Tag.None -> throw NullPointerException()
                FFIResult_Tag.Err -> throw handleError(err)
                else -> throw UnknownFFIError()
            }
        }
    }

    actual fun fileName(): String = memScoped {
        return path_file_name(internal.ptr).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> ok.string.toKString()
                FFIResult_Tag.None -> throw NullPointerException()
                FFIResult_Tag.Err -> throw handleError(err)
                else -> throw UnknownFFIError()
            }
        }
    }

    actual fun extension(): String = memScoped {
        return path_extension(internal.ptr).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> ok.string.toKString()
                FFIResult_Tag.None -> throw NullPointerException()
                FFIResult_Tag.Err -> throw handleError(err)
                else -> throw UnknownFFIError()
            }
        }
    }

    actual fun parent(): Path = memScoped {
        return path_parent(internal.ptr).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> Path(ok.handle.toCValue())
                FFIResult_Tag.None -> throw NullPointerException()
                FFIResult_Tag.Err -> throw handleError(err)
                else -> throw UnknownFFIError()
            }
        }
    }

    actual fun setFileName(name: String) = memScoped {
        path_set_file_name(internal.ptr, name.toFFIString(this))
            .useContents {
                when (tag) {
                    FFIResult_Tag.Ok -> Unit
                    FFIResult_Tag.Err -> throw handleError(err)
                    else -> throw UnknownFFIError()
                }
            }
    }

    actual fun setExtension(extension: String) = memScoped {
        path_set_extension(internal.ptr, extension.toFFIString(this))
            .useContents {
                when (tag) {
                    FFIResult_Tag.Ok -> Unit
                    FFIResult_Tag.Err -> throw handleError(err)
                    else -> throw UnknownFFIError()
                }
            }
    }

    actual fun normalize(): Path = memScoped {
        return path_normalize(internal.ptr).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> Path(ok.handle.toCValue())
                FFIResult_Tag.Err -> throw handleError(err)
                else -> throw UnknownFFIError()
            }
        }
    }

    actual fun canonicalize(): Path = memScoped {
        return path_canonicalize(internal.ptr).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> Path(ok.handle.toCValue())
                FFIResult_Tag.Err -> throw handleError(err)
                else -> throw UnknownFFIError()
            }
        }
    }

    actual fun isAbsolute(): Boolean = memScoped {
        return path_is_absolute(internal.ptr).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> ok.boolean
                FFIResult_Tag.Err -> throw handleError(err)
                else -> throw UnknownFFIError()
            }
        }
    }

    actual fun isRelative(): Boolean = memScoped {
        return path_is_relative(internal.ptr).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> ok.boolean
                FFIResult_Tag.Err -> throw handleError(err)
                else -> throw UnknownFFIError()
            }
        }
    }

    actual fun exists(): Boolean = memScoped {
        return path_exists(internal.ptr).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> ok.boolean
                FFIResult_Tag.Err -> throw handleError(err)
                else -> throw UnknownFFIError()
            }
        }
    }

    actual fun isFile(): Boolean = memScoped {
        return path_is_file(internal.ptr).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> ok.boolean
                FFIResult_Tag.Err -> throw handleError(err)
                else -> throw UnknownFFIError()
            }
        }
    }

    actual fun isDirectory(): Boolean = memScoped {
        return path_is_dir(internal.ptr).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> ok.boolean
                FFIResult_Tag.Err -> throw handleError(err)
                else -> throw UnknownFFIError()
            }
        }
    }

    actual fun metadata(): Metadata = memScoped {
        return path_metadata(internal.ptr).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> Metadata(ok.handle.toCValue())
                FFIResult_Tag.None -> throw NullPointerException()
                FFIResult_Tag.Err -> throw handleError(err)
                else -> throw UnknownFFIError()
            }
        }
    }

    actual fun readDir(): ReadDir = memScoped {
        return path_read_dir(internal.ptr).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> ReadDir(ok.handle.toCValue())
                FFIResult_Tag.None -> throw NullPointerException()
                FFIResult_Tag.Err -> throw handleError(err)
                else -> throw UnknownFFIError()
            }
        }
    }

    actual fun components(): List<String> = memScoped {
        return path_components(internal.ptr).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> ok.vec.toStringList()
                FFIResult_Tag.None -> throw NullPointerException()
                FFIResult_Tag.Err -> throw handleError(err)
                else -> throw UnknownFFIError()
            }
        }
    }

    actual fun clear() = memScoped {
        path_clear(internal.ptr).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> Unit
                FFIResult_Tag.Err -> throw handleError(err)
                else -> throw UnknownFFIError()
            }
        }
    }

    actual fun clone(): Path = memScoped {
        return path_clone(internal.ptr).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> Path(ok.handle.toCValue())
                FFIResult_Tag.None -> throw NullPointerException()
                FFIResult_Tag.Err -> throw handleError(err)
                else -> throw UnknownFFIError()
            }
        }
    }

    actual fun toStringLossy(): String = memScoped {
        return path_to_string_lossy(internal.ptr).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> ok.string.toKString()
                FFIResult_Tag.None -> throw NullPointerException()
                FFIResult_Tag.Err -> throw handleError(err)
                else -> throw UnknownFFIError()
            }
        }
    }

    override fun toString(): String = memScoped {
        return path_to_string(internal.ptr).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> ok.string.toKString()
                FFIResult_Tag.None -> throw NullPointerException()
                FFIResult_Tag.Err -> throw handleError(err)
                else -> throw UnknownFFIError()
            }
        }
    }

    override fun compareTo(other: Path): Int = memScoped {
        return path_compare(internal.ptr, other.internal.ptr).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> ok.int32.convert()
                FFIResult_Tag.Err -> throw handleError(err)
                else -> throw UnknownFFIError()
            }
        }
    }

    override fun equals(other: Any?): Boolean = memScoped {
        if (this === other) return true
        if (other !is Path) return false

        return path_eq(internal.ptr, other.internal.ptr).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> ok.boolean
                FFIResult_Tag.Err -> throw handleError(err)
                else -> throw UnknownFFIError()
            }
        }
    }

    override fun hashCode(): Int = memScoped {
        return path_hash(internal.ptr).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> ok.u_size.convert()
                FFIResult_Tag.Err -> throw handleError(err)
                else -> throw UnknownFFIError()
            }
        }
    }
}
