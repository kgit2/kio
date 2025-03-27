import path.Path
import kotlin.test.Test

class LeaksTest {
    @Test
    fun testLeaks() {
        for (i in 0..10000) {
            val path = Path.cwd()
            path.readDir().use { readDir ->
                readDir.forEach {
                    it.use {  }
                }
            }
        }
    }
}
