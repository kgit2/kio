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
    val path = Path("/Users/bppleman/a")
    path.setFileName("stats_user_wes_kit_keep")
    path.setExtension("html")
//    println(path)
//    val clone = path.clone()
////    clone.setFileName("CLionProjects")
//    println(clone.toString())
//    println(path == clone)
//    println(path === clone)
//     println(path.fileName())
//     println(path.extension())
//     println(path.parent())
//     println()
}
