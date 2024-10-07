package io.read

import kotlinx.cinterop.CValue
import kotlinx.cinterop.convert
import kotlinx.cinterop.refTo

open class TypedRead(
    private val ref: CValue<rio.TypedReadRef>
) : Read {
    override fun read(buf: ByteArray): ULong {
        return rio.typed_read_ref_read(ref, buf.refTo(0), buf.size.convert())
    }
}
