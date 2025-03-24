import kotlinx.cinterop.*
import rio.*

fun FFIString.toKString(): String {
    val kString = this.buffer!!.toKStringFromUtf8()
    free_ffi_string(cValue {
        this.buffer = this@toKString.buffer
        this.len = this@toKString.len
    })
    return kString
}

fun FFIVec.toStringList(): List<String> {
    val list = List(len.convert()) {
        items!![it].string.buffer!!.toKStringFromUtf8()
    }
    free_ffi_vec(cValue {
        this.items = this@toStringList.items
        this.len = this@toStringList.len
        this.capacity = this@toStringList.capacity
    })
    return list
}

fun FFIVec.toStringMutableList(): MutableList<String> {
    val list = MutableList(len.convert()) {
        items!![it].string.buffer!!.toKStringFromUtf8()
    }
    free_ffi_vec(cValue {
        this.items = this@toStringMutableList.items
        this.len = this@toStringMutableList.len
        this.capacity = this@toStringMutableList.capacity
    })
    return list
}

fun FFIHandle.toCValue(): CValue<FFIHandle> {
    return cValue {
        this.index = this@toCValue.index
        this.handle_type = this@toCValue.handle_type
    }
}

fun String.toFFIString(scope: MemScope): CValue<FFIString> {
    return cValue<FFIString> {
        buffer = this@toFFIString.cstr.getPointer(scope)
        len = this@toFFIString.length.convert()
    }
}

fun ByteArray.toFFIBytes(scope: MemScope): CValue<FFIBytes> {
    return cValue<FFIBytes> {
        buffer = asUByteArray().refTo(0).getPointer(scope)
        len = size.convert()
        capacity = size.convert()
    }
}
