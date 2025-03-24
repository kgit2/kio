package fs

expect class ReadDir private constructor() : Iterator<DirEntry>, AutoCloseable {
}

expect fun readDirSize(): ULong

expect fun dirEntrySize(): ULong
