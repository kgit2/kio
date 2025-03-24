import fs.dirEntrySize
import fs.readDirSize
import path.Path

//fun main() {
//    val buf = ByteArray(1024)
//    Stdin.read(buf, 1024).map {
//        println("Read ${it} bytes")
//        Stdout.write(buf.sliceArray(0 until it.toInt()), it)
//            .map {
//                println("Wrote $it bytes")
//            }
//    }
//}

fun main() {
    for (i in 0..< 40) {
        val path = Path.cwd()
        val readDir = path.readDir()
        readDir.use {
            while (readDir.hasNext()) {
                readDir.next()
            }
        }
    }
    println(readDirSize())
    println(dirEntrySize())
}
