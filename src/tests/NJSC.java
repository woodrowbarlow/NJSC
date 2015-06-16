package tests;

import java.io.File;

public class NJSC {

    public static native void test();

    public static void main(String args[]) {
        File f = new File("njsc/target/release/libnjsc.so");
        System.load(f.getAbsolutePath());
        NJSC.test();
    }

}
