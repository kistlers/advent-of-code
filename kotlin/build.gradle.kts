plugins {
    kotlin("jvm") version "2.2.21"
}

repositories {
    mavenCentral()
}

dependencies {
    implementation("org.locationtech.jts:jts-core:latest.release")
}

sourceSets {
    main {
        kotlin.srcDir("src")
    }
}

tasks {
    wrapper {
        gradleVersion = "9.2.1"
    }
}
