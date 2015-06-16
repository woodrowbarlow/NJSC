package tests;

import java.io.File;

public class NJSC {

    static {
        File f = new File("njsc/target/release/libnjsc.so");
        System.load(f.getAbsolutePath());
    }

    public native void info();

    public native void hasArgument(int i);

    public native int sum(int a, int b);

}
