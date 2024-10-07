package io.write

import kotlinx.cinterop.CValue
import kotlinx.cinterop.convert

class TypedWrite(
    private val ref: CValue<rio.TypedWriteRef>
) : Write {
    override fun write(buf: String): ULong {
        return rio.typed_write_ref_write(ref, buf, buf.length.convert<ULong>())
    }

    override fun flush() {
        rio.typed_write_ref_flush(ref)
    }
}
