import rio.create_stdout_ptr
import kotlin.test.Test

class Test {
    @Test
    fun test() {
        val tw = create_stdout_ptr()
        println(tw)
    }
}
