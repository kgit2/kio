package io.buffered

import io.Read
import kotlin.math.min

class Buffer(
    val buf: SlicedByteArray,
    pos: Int,
    filled: Int,
    private var initialized: Int,
) {
    var pos = pos
        private set

    var filled = filled
        private set

    constructor(capacity: Int) : this(SlicedByteArray.allocate(capacity), 0, 0, 0)

    inline fun buffer(): SlicedByteArray = buf

    inline fun capacity(): Int = buf.size

    fun discardBuffer() {
        pos = 0
        filled = 0
    }

    fun consume(amt: Int) {
        pos = min(pos + amt, filled)
    }

    fun consumeWith(amt: Int, visitor: (SlicedByteArray) -> Unit): Boolean {
        if (filled - pos >= amt) {
            visitor(buf.slice(0, amt))
            return true
        } else {
            return false
        }
    }

    fun unconsume(amt: Int) {
        pos = pos.minus(amt)
        pos = if (pos > amt) pos - amt else 0
    }

    fun <T : Read> readMore(reader: T): Int {
        val buf = this.buf.slice(pos..< this.buf.size)
        val oldInit = this.initialized - this.pos
        val readSize = reader.read(buf, oldInit, buf.size - oldInit)
        this.filled += readSize
        this.initialized += readSize
        return readSize
    }

    // Remove bytes that have already been read from the buffer.
    fun backshift() {
        buf.copyInto(buf, 0, pos, filled)
        initialized -= pos
        filled -= pos
        pos = 0
    }

    fun fillBuf(reader: Read): SlicedByteArray {
        // If we've reached the end of our internal buffer then we need to fetch
        // some more data from the reader.
        // Branch using `>=` instead of the more correct `==`
        // to tell the compiler that the pos..cap slice is always valid.
        if (pos >= filled) {
            val buf = this.buf.slice(pos..< this.buf.size)
            val oldInit = this.initialized - this.pos
            reader.read(buf, oldInit, buf.size - oldInit)
            filled += buf.size
            initialized += buf.size - oldInit
        }
        return this.buf.slice(pos..< filled)
    }

//    #[inline ]
//    pub fn fill_buf(&mut self, mut reader: impl Read) -> io::Result<&[u8]>
//    {
//        // If we've reached the end of our internal buffer then we need to fetch
//        // some more data from the reader.
//        // Branch using `>=` instead of the more correct `==`
//        // to tell the compiler that the pos..cap slice is always valid.
//        if self.pos >= self.filled {
//            debug_assert!(self.pos == self.filled);
//
//            let mut buf = BorrowedBuf::from(& mut * self . buf);
//            // SAFETY: `self.filled` bytes will always have been initialized.
//            unsafe {
//                buf.set_init(self.initialized);
//            }
//
//            let result = reader . read_buf (buf.unfilled());
//
//            self.pos = 0;
//            self.filled = buf.len();
//            self.initialized = buf.init_len();
//
//            result?;
//        }
//        Ok(self.buffer())
//    }
}
