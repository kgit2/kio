import path.Path
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertNull
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
        assertEquals(listOf("a", "b", "c"), path.components())
    }
    @Test
    fun testNormalizeMultipleSlashesAndDots() {
        val path = Path("a//./b///c/.")
        assertEquals("a/b/c", path.normalize().toString())
    }

    @Test
    fun testNormalizeBeyondRootAbsolute() {
        val path = Path("/../../a")
        assertEquals("/a", path.normalize().toString())
    }

    @Test
    fun testNormalizeLeadingDotDotRelative() {
        val path = Path("../a/b/..")
        // Leading ".." in a relative path should be preserved
        assertEquals("../a", path.normalize().toString())
    }

    @Test
    fun testNormalizeIdempotent() {
        val p1 = Path("a/./b/../c/./d").normalize()
        val p2 = p1.normalize()
        assertEquals(p1.toString(), p2.toString())
    }

    @Test
    fun testParentOfRootStable() {
        val root = Path("/")
        assertNull(root.parent())
    }

    @Test
    fun testTrailingSlashHandling() {
        val p = Path("a/b/")
        assertEquals("a/b", p.normalize().toString())
        // fileName after normalization should be the last component
        assertEquals("b", p.normalize().fileName())
    }

    @Test
    fun testExtensionMultipleDots() {
        val p = Path("archive.tar.gz")
        assertEquals("gz", p.extension())
    }

    @Test
    fun testPushPopWithNormalize() {
        val p = Path("/")
        p.push("a")
        p.push("..")
        assertEquals("/", p.normalize().toString())
    }

    @Test
    fun testComponentsAfterNormalizeCollapses() {
        val p = Path("a//b///c/").normalize()
        assertEquals(listOf("a", "b", "c"), p.components())
    }

    @Test
    fun testNormalizeRootDot() {
        val p = Path("/./")
        assertEquals("/", p.normalize().toString())
    }

    @Test
    fun testNormalizeExcessParentInRelative() {
        val p = Path("a/b/../../../c")
        // One more ".." than segments should survive for relative paths
        assertEquals("../c", p.normalize().toString())
    }
}
