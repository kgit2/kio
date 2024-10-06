import org.jetbrains.kotlin.gradle.ExperimentalKotlinGradlePluginApi

plugins {
    alias(libs.plugins.kotlinMultiplatform)
    // alias(libs.plugins.androidLibrary)
    id("module.publication")
}

group = "com.kgit2"
version = "0.0.1"

kotlin {
    jvm()
    // androidTarget {
    //     publishLibraryVariants("release")
    //     @OptIn(ExperimentalKotlinGradlePluginApi::class)
    //     compilerOptions {
    //         jvmTarget.set(JvmTarget.JVM_1_8)
    //     }
    // }
    // iosX64()
    // iosArm64()
    // iosSimulatorArm64()
    // linuxX64()

    val nativeTargets = listOf(
        macosX64() to Platform.MACOS_X64,
        macosArm64() to Platform.MACOS_ARM64,
        linuxX64() to Platform.LINUX_X64,
        linuxArm64() to Platform.LINUX_ARM64,
        mingwX64() to Platform.MINGW_X64,
    )

    nativeTargets.forEach { (nativeTarget, targetPlatform) ->
        nativeTarget.apply {
            compilations.getByName("main") {
                cinterops {
                    create("rio") {
                        defFile(project.file("src/nativeInterop/cinterop/${targetPlatform.archName}.def"))
                        packageName("rio")
                    }
                }
            }
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
                implementation("org.jetbrains.kotlinx:atomicfu:0.23.1")
            }
        }
        val commonTest by getting {
            dependencies {
                implementation(libs.kotlin.test)
            }
        }
    }
}

// android {
//     namespace = "org.jetbrains.kotlinx.multiplatform.library.template"
//     compileSdk = libs.versions.android.compileSdk.get().toInt()
//     defaultConfig {
//         minSdk = libs.versions.android.minSdk.get().toInt()
//     }
// }

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
