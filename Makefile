all: clean setup build run
setup:
	mkdir -p out
build: build-rust build-java
build-rust:
	cargo build --release
build-java:
	javac src/java/me/wbarlow/njsc/*.java -d out/java
run:
	java -classpath out/java me.wbarlow.njsc.Test
clean:
	rm -rf out
	rm -rf target
