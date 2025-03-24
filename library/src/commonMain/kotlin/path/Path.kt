package path

import fs.Metadata
import fs.ReadDir

expect class Path(value: String): Comparable<Path> {
    companion object {
        fun cwd(): Path
    }

    fun push(value: String)
    fun pop(): Boolean
    fun fileName(): String
    fun extension(): String
    fun parent(): Path
    fun setFileName(name: String)
    fun setExtension(extension: String)

    fun normalize(): Path
    fun canonicalize(): Path
    fun isAbsolute(): Boolean
    fun isRelative(): Boolean

    fun exists(): Boolean
    fun isFile(): Boolean
    fun isDirectory(): Boolean

    fun metadata(): Metadata
    fun readDir(): ReadDir
    fun components(): List<String>

    fun clear()
    fun clone(): Path
    fun toStringLossy(): String
}
