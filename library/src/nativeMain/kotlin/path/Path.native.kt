package path

import handleError
import kotlinx.cinterop.*
import rio.*
import toKString
import toStringList

actual class Path(
    private val internal: CValue<FFIHandle>,
) : Comparable<Path> {

    actual constructor(value: String) : this(memScoped {
        val cValue = cValue<FFIString> {
            buffer = value.cstr.ptr
            len = value.length.convert()
        }
        path_init(cValue).useContents {
            cValue {
                this.index = ok.handle.index
                this.handle_type = ok.handle.handle_type
            }
        }
    })

    actual companion object {
        actual fun cwd(): Path {
            return path_cwd().useContents {
                when (tag) {
                    FFIResult_Tag.Ok -> {
                        val cwdInternal = cValue<FFIHandle> {
                            this.index = ok.handle.index
                            this.handle_type = ok.handle.handle_type
                        }
                        Path(cwdInternal)
                    }

                    FFIResult_Tag.Err -> throw handleError(err.string)
                    else -> throw Exception("Unknown error")
                }
            }
        }
    }

    actual fun push(value: String) = memScoped {
        val cValue = cValue<FFIString> {
            buffer = value.cstr.ptr
            len = value.length.convert()
        }
        path_push(internal, cValue).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> Unit
                FFIResult_Tag.Err -> throw handleError(err.string)
                else -> throw Exception("Unknown error")
            }
        }
    }

    actual fun pop(): Boolean {
        return path_pop(internal).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> ok.boolean
                FFIResult_Tag.None -> throw NullPointerException()
                FFIResult_Tag.Err -> throw handleError(err.string)
                else -> throw Exception("Unknown error")
            }
        }
    }

    actual fun fileName(): String {
        return path_file_name(internal).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> ok.string.toKString()
                FFIResult_Tag.None -> throw NullPointerException()
                FFIResult_Tag.Err -> throw handleError(err.string)
                else -> throw Exception("Unknown error")
            }
        }
    }

    actual fun extension(): String {
        return path_extension(internal).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> ok.string.toKString()
                FFIResult_Tag.None -> throw NullPointerException()
                FFIResult_Tag.Err -> throw handleError(err.string)
                else -> throw Exception("Unknown error")
            }
        }
    }

    actual fun parent(): Path = memScoped {
        return path_parent(internal).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> {
                    val parentInternal = cValue<FFIHandle> {
                        this.index = ok.handle.index
                        this.handle_type = ok.handle.handle_type
                    }
                    Path(parentInternal)
                }

                FFIResult_Tag.None -> throw NullPointerException()
                FFIResult_Tag.Err -> throw handleError(err.string)
                else -> throw Exception("Unknown error")
            }
        }
    }

    actual fun setFileName(name: String) = memScoped {
        val cValue = cValue<FFIString> {
            buffer = name.cstr.ptr
            len = name.length.convert()
        }
        path_set_file_name(internal, cValue).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> Unit
                FFIResult_Tag.Err -> throw handleError(err.string)
                else -> throw Exception("Unknown error")
            }
        }
    }

    actual fun setExtension(extension: String) = memScoped {
        val cValue = cValue<FFIString> {
            buffer = extension.cstr.ptr
            len = extension.length.convert()
        }
        path_set_extension(internal, cValue).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> Unit
                FFIResult_Tag.Err -> throw handleError(err.string)
                else -> throw Exception("Unknown error")
            }
        }
    }

    actual fun normalize(): Path {
        return path_normalize(internal).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> {
                    val normalizedInternal = cValue<FFIHandle> {
                        this.index = ok.handle.index
                        this.handle_type = ok.handle.handle_type
                    }
                    Path(normalizedInternal)
                }

                FFIResult_Tag.Err -> throw handleError(err.string)
                else -> throw Exception("Unknown error")
            }
        }
    }

    actual fun canonicalize(): Path {
        return path_canonicalize(internal).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> {
                    val canonicalizedInternal = cValue<FFIHandle> {
                        this.index = ok.handle.index
                        this.handle_type = ok.handle.handle_type
                    }
                    Path(canonicalizedInternal)
                }

                FFIResult_Tag.Err -> throw handleError(err.string)
                else -> throw Exception("Unknown error")
            }
        }
    }

    actual fun isAbsolute(): Boolean {
        return path_is_absolute(internal).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> ok.boolean
                FFIResult_Tag.Err -> throw handleError(err.string)
                else -> throw Exception("Unknown error")
            }
        }
    }

    actual fun isRelative(): Boolean {
        return path_is_relative(internal).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> ok.boolean
                FFIResult_Tag.Err -> throw handleError(err.string)
                else -> throw Exception("Unknown error")
            }
        }
    }

    actual fun exists(): Boolean {
        return path_exists(internal).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> ok.boolean
                FFIResult_Tag.Err -> throw handleError(err.string)
                else -> throw Exception("Unknown error")
            }
        }
    }

    actual fun isFile(): Boolean {
        return path_is_file(internal).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> ok.boolean
                FFIResult_Tag.Err -> throw handleError(err.string)
                else -> throw Exception("Unknown error")
            }
        }
    }

    actual fun isDirectory(): Boolean {
        return path_is_dir(internal).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> ok.boolean
                FFIResult_Tag.Err -> throw handleError(err.string)
                else -> throw Exception("Unknown error")
            }
        }
    }

    actual fun components(): List<String> {
        return path_components(internal).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> ok.vec.toStringList()
                FFIResult_Tag.None -> throw NullPointerException()
                FFIResult_Tag.Err -> throw handleError(err.string)
                else -> throw Exception("Unknown error")
            }
        }
    }

    actual fun clear() {
        path_clear(internal).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> Unit
                FFIResult_Tag.Err -> throw handleError(err.string)
                else -> throw Exception("Unknown error")
            }
        }
    }

    actual fun clone(): Path = memScoped {
        return path_clone(internal).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> {
                    val clonedInternal = cValue<FFIHandle> {
                        this.index = ok.handle.index
                        this.handle_type = ok.handle.handle_type
                    }
                    Path(clonedInternal)
                }

                FFIResult_Tag.None -> throw NullPointerException()
                FFIResult_Tag.Err -> throw handleError(err.string)
                else -> throw Exception("Unknown error")
            }
        }
    }

    actual fun toStringLossy(): String {
        return path_to_string_lossy(internal).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> ok.string.toKString()
                FFIResult_Tag.None -> throw NullPointerException()
                FFIResult_Tag.Err -> throw handleError(err.string)
                else -> throw Exception("Unknown error")
            }
        }
    }

    override fun toString(): String {
        return path_to_string(internal).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> ok.string.toKString()
                FFIResult_Tag.None -> throw NullPointerException()
                FFIResult_Tag.Err -> throw handleError(err.string)
                else -> throw Exception("Unknown error")
            }
        }
    }

    override fun compareTo(other: Path): Int {
        return path_compare(internal, other.internal).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> ok.int32.convert()
                FFIResult_Tag.Err -> throw handleError(err.string)
                else -> throw Exception("Unknown error")
            }
        }
    }

    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (other !is Path) return false

        return path_eq(internal, other.internal).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> ok.boolean
                FFIResult_Tag.Err -> throw handleError(err.string)
                else -> throw Exception("Unknown error")
            }
        }
    }

    override fun hashCode(): Int {
        return path_hash(internal).useContents {
            when (tag) {
                FFIResult_Tag.Ok -> ok.u_size.convert()
                FFIResult_Tag.Err -> throw handleError(err.string)
                else -> throw Exception("Unknown error")
            }
        }
    }
}
