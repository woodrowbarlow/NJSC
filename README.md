# NJSC
New Java Serial Connector

This project will provide a Java library which provides serial port connectivity. The project will be a spiritual successor to the jSSC project. It will be compatible with Java 1.5 and will use standardized interfaces similar to other Java connectors. It will ship multiple variants: a normal standalone JAR and an OSGi bundle JAR. If appropriate, it will also ship a Java 1.8 version of each variant which utilizes newer interfaces.

Native code will be written in Rust and packaged in the JAR (so that it does not need to be pre-installed on the target system). For now, this might limit the potential compile targets, but Rust will quickly expand into the embedded devices world.
