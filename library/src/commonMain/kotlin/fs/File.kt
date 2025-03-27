package fs

import io.Read
import io.Write
import path.Path

expect class File(path: Path): Read, Write {
}
