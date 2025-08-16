package io

abstract class BufferedReader<T : Read>(
    private val source: T,
    private val bufferSize: Int = 8192,
) : Read {

}
