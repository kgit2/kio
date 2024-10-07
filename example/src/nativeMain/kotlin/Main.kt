import io.write.Stdout
import kotlinx.cinterop.ULongVar
import kotlinx.cinterop.alloc
import kotlinx.cinterop.convert
import kotlinx.cinterop.get
import kotlinx.cinterop.memScoped
import kotlinx.cinterop.ptr
import kotlinx.cinterop.refTo
import kotlinx.cinterop.value
import rio.typed_read_ref_new_byte_array
import rio.typed_read_ref_read_to_string
import rio.typed_write_ref_create_byte_array
import rio.typed_write_ref_write

fun main() {
    Stdout.write("Hello, world!")
    // val byteArray = UByteArray(1024)
    //
    // val cursor = typed_write_ref_create_byte_array(byteArray.refTo(0), byteArray.size.convert<ULong>())
    //
    // val buf = "Hello, world!"
    // typed_write_ref_write(cursor, buf, buf.length.convert<ULong>())
    //
    // val readCursor = typed_read_ref_new_byte_array(byteArray.refTo(0), byteArray.size.convert<ULong>())
    //
    // memScoped {
    //     val size = alloc<ULongVar>()
    //     val byteVar = typed_read_ref_read_to_string(readCursor, size.ptr)
    //     byteVar?.let {
    //         val string = CharArray(size.value.toInt()) { byteVar[it].toInt().toChar() }.concatToString()
    //         println(size.value)
    //         println(string)
    //     }
    //     // drop_c_char(byteVar)
    // }
}
