import org.jetbrains.kotlin.gradle.ExperimentalKotlinGradlePluginApi

plugins {
    alias(libs.plugins.kotlinMultiplatform)
    id("module.publication")
}

group = "com.kgit2"
version = "0.0.1"

kotlin {
    val os = System.getProperty("os.name").lowercase()
    val arch = System.getProperty("os.arch").lowercase()
    var platform = Platform.MACOS_X64
    val nativeTarget = when {
        os.contains("mac") -> if (arch.contains("arm")) {
            platform = Platform.MACOS_ARM64
            macosArm64("native")
        } else {
            platform = Platform.MACOS_X64
            macosX64("native")
        }
        os.contains("linux") -> if (arch.contains("arm")) {
            platform = Platform.LINUX_ARM64
            linuxArm64("native")
        } else {
            platform = Platform.LINUX_X64
            linuxX64("native")
        }
        os.contains("windows") -> {
            platform = Platform.MINGW_X64
            mingwX64("native")
        }
        else -> error("Unsupported OS: $os")
    }

    nativeTarget.apply {
        compilations.getByName("main") {
            cinterops {
                create("rio") {
                    defFile(project.file("../nativeInterop/cinterop/${platform.archName}.def"))
                    packageName("rio")
                }
            }
        }

        binaries {
            executable()
        }
    }

    sourceSets {
        // add opt-in
        all {
            // languageSettings.optIn("kotlinx.cinterop.UnsafeNumber")
            languageSettings.optIn("kotlinx.cinterop.ExperimentalForeignApi")
            languageSettings.optIn("kotlin.experimental.ExperimentalNativeApi")
            // languageSettings.optIn("kotlin.native.runtime.NativeRuntimeApi")
            languageSettings.optIn("kotlin.ExperimentalStdlibApi")

            languageSettings {
                @OptIn(ExperimentalKotlinGradlePluginApi::class)
                compilerOptions {
                    freeCompilerArgs.add("-Xexpect-actual-classes")
                }
            }
        }

        val commonMain by getting {
            dependencies {
                //put your multiplatform dependencies here
                implementation(project(":library"))
            }
        }
        val commonTest by getting {
            dependencies {
                implementation(libs.kotlin.test)
            }
        }

        val nativeMain by getting {
            dependencies {  }
        }
    }
}

enum class Platform(
    val archName: String
) {
    MACOS_X64("x86_64-apple-darwin"),
    MACOS_ARM64("aarch64-apple-darwin"),
    LINUX_X64("x86_64-unknown-linux-gnu"),
    LINUX_ARM64("aarch64-unknown-linux-gnu"),
    MINGW_X64("x86_64-pc-windows-gnu"),
    ;
}

val platforms: List<Platform> = listOf(
    Platform.MACOS_X64,
    Platform.MACOS_ARM64,
    Platform.LINUX_X64,
    Platform.LINUX_ARM64,
    Platform.MINGW_X64,
)
