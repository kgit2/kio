@OptIn(ExperimentalUnsignedTypes::class)
interface Read {
    // Required method
    fun read(buf: MutableList<UByte>): Result<UInt, Throwable>;

    // Provided methods
    // fun read_vectored(bufs: &mut [IoSliceMut<'_>]) -> Result<usize> { ... }
    fun is_read_vectored(): Boolean = false
    fun read_to_end(buf: MutableList<UByte>): Result<UInt, Throwable>
    // fn read_to_string(&mut self, buf: &mut String) -> Result<usize> { ... }
    // fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> { ... }
    // fn read_buf(&mut self, buf: BorrowedCursor<'_>) -> Result<()> { ... }
    // fn read_buf_exact(&mut self, cursor: BorrowedCursor<'_>) -> Result<()> { ... }
    // fn by_ref(&mut self) -> &mut Self
    // where Self: Sized { ... }
    // fn bytes(self) -> Bytes<Self> ⓘ
    // where Self: Sized { ... }
    // fn chain<R: Read>(self, next: R) -> Chain<Self, R> ⓘ
    // where Self: Sized { ... }
    // fn take(self, limit: u64) -> Take<Self> ⓘ
    // where Self: Sized { ... }
}

class TypedRead {

}

enum class ReadType {

}
