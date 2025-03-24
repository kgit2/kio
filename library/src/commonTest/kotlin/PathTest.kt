import path.Path
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertTrue

class PathTest {
    @Test
    fun testInit() {
        val path = Path("a/b/c")
        assertEquals("a/b/c", path.toString())
    }

    @Test
    fun testCwd() {
        for (i in 0..10000) {
            val path = Path.cwd()
            path.readDir().use { readDir ->
                readDir.forEach {
                    it.use {  }
                }
            }
        }
    }

    @Test
    fun testPush() {
        val path = Path("a")
        path.push("b")
        assertEquals("a/b", path.toString())
    }

    @Test
    fun testPop() {
        val path = Path("a/b")
        assertTrue(path.pop())
        assertEquals("a", path.toString())
    }

    @Test
    fun testFileName() {
        val path = Path("a/b/c")
        assertEquals("c", path.fileName())
    }

    @Test
    fun testExtension() {
        val path = Path("a/b/c.txt")
        assertEquals("txt", path.extension())
    }

    @Test
    fun testParent() {
        val path = Path("a/b/c")
        assertEquals("a/b", path.parent().toString())
    }

    @Test
    fun testSetFileName() {
        val path = Path("a/b/c")
        path.setFileName("d")
        assertEquals("a/b/d", path.toString())
    }

    @Test
    fun testSetExtension() {
        val path = Path("a/b/c.txt")
        path.setExtension("html")
        assertEquals("a/b/c.html", path.toString())
    }

    @Test
    fun testNormalize() {
        var path = Path("a/b/../c")
        assertEquals("a/c", path.normalize().toString())

        path = Path("a/b")
        assertEquals("a/b", path.normalize().toString())

        path = Path("a/b/../../c")
        assertEquals("c", path.normalize().toString())

        path = Path("a/./b/./c")
        assertEquals("a/b/c", path.normalize().toString())

        path = Path("a/./b/../c/./d")
        assertEquals("a/c/d", path.normalize().toString())

        path = Path("/a/b/../c")
        assertEquals("/a/c", path.normalize().toString())

        path = Path("/")
        assertEquals("/", path.normalize().toString())
    }

    @Test
    fun testIsAbsolute() {
        var path = Path("a/b/c")
        assertFalse(path.isAbsolute())

        path = Path("/a/b/c")
        assertTrue(path.isAbsolute())
    }

    @Test
    fun testIsRelative() {
        var path = Path("a/b/c")
        assertTrue(path.isRelative())

        path = Path("/a/b/c")
        assertTrue(!path.isRelative())
    }

    @Test
    fun testExists() {
        var path = Path.cwd()
        assertTrue(path.exists())

        path = Path("/a/b/c")
        assertFalse(path.exists())
    }

    @Test
    fun testIsFile() {
        var path = Path.cwd()
        assertFalse(path.isFile())

        path = Path("/a/b/c")
        assertFalse(path.isFile())
    }

    @Test
    fun testIsDirectory() {
        var path = Path.cwd()
        assertTrue(path.isDirectory())

        path = Path("/a/b/c")
        assertFalse(path.isDirectory())
    }

    @Test
    fun testComponents() {
        val path = Path("a/b/c")
        for (i in 0..10000) {
            assertEquals(listOf("a", "b", "c"), path.components())
        }
    }
}
