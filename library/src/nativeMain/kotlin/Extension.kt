import kotlinx.cinterop.cValue
import kotlinx.cinterop.convert
import kotlinx.cinterop.get
import kotlinx.cinterop.toKStringFromUtf8
import rio.FFIString
import rio.FFIVec
import rio.free_ffi_string
import rio.free_ffi_vec

fun FFIString.toKString(): String {
    val cValue = cValue<FFIString> {
        buffer = this@toKString.buffer
        len = this@toKString.len
    }
    val kString = this.buffer!!.toKStringFromUtf8()
    free_ffi_string(cValue)
    return kString
}

fun FFIVec.toStringList(): List<String> {
    val list = List(len.convert()) {
        items!![it].string.toKString()
    }
    val cValue = cValue<FFIVec> {
        this.items = this@toStringList.items
        this.len = this@toStringList.len
        this.capacity = this@toStringList.capacity
    }
    free_ffi_vec(cValue)
   return list
}

fun FFIVec.toStringMutableList(): MutableList<String> {
    val list = MutableList(len.convert()) {
        items!![it].string.toKString()
    }
    val cValue = cValue<FFIVec> {
        this.items = this@toStringMutableList.items
        this.len = this@toStringMutableList.len
        this.capacity = this@toStringMutableList.capacity
    }
    free_ffi_vec(cValue)
    return list
}
