package io

import io.read.Read
import io.read.TypedRead
import io.write.TypedWrite
import io.write.Write

object Stdout : Write by TypedWrite(rio.typed_write_ref_new_stdout())

object Stderr : Write by TypedWrite(rio.typed_write_ref_new_stderr())

object Stdin : Read by TypedRead(rio.typed_read_ref_new_stdin())
