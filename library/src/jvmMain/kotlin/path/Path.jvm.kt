package path

import fs.Metadata
import fs.ReadDir
import java.io.BufferedReader
import java.io.BufferedWriter

actual class Path actual constructor(value: String) : Comparable<Path> {

    actual fun push(value: String) {
    }

    actual fun pop(): Boolean {
        TODO("Not yet implemented")
    }

    actual fun setFileName(name: String) {
    }

    actual fun setExtension(extension: String) {
    }

    actual fun clear() {
    }

    override fun compareTo(other: Path): Int {
        TODO("Not yet implemented")
    }

    actual fun fileName(): String {
        TODO("Not yet implemented")
    }

    actual fun extension(): String {
        TODO("Not yet implemented")
    }

    actual fun parent(): Path? {
        TODO("Not yet implemented")
    }

    actual fun normalize(): Path {
        TODO("Not yet implemented")
    }

    actual fun canonicalize(): Path {
        TODO("Not yet implemented")
    }

    actual fun isAbsolute(): Boolean {
        TODO("Not yet implemented")
    }

    actual fun isRelative(): Boolean {
        TODO("Not yet implemented")
    }

    actual fun exists(): Boolean {
        TODO("Not yet implemented")
    }

    actual fun isFile(): Boolean {
        TODO("Not yet implemented")
    }

    actual fun isDirectory(): Boolean {
        TODO("Not yet implemented")
    }

    actual fun metadata(): Metadata {
        TODO("Not yet implemented")
    }

    actual fun readDir(): ReadDir {
        TODO("Not yet implemented")
    }

    actual fun components(): List<String> {
        TODO("Not yet implemented")
    }

    actual fun clone(): Path {
        TODO("Not yet implemented")
    }

    actual fun toStringLossy(): String {
        TODO("Not yet implemented")
    }

    actual companion object {
        actual fun cwd(): Path {
            TODO("Not yet implemented")
        }
    }
}
