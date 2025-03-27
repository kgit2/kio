package fs

import path.Path

actual class File actual constructor(path: Path) : Read, Write {
}
