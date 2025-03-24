package fs

import path.Path

expect class DirEntry private constructor() : AutoCloseable {
    fun path(): Path
    fun fileType(): FileType
    fun metadata(): Metadata
}
