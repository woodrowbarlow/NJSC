package tests;

import java.io.File;

/**
 * Created by wbarlow on 6/15/15.
 */
public class Test {

    public static void main(String args[]) {
        NJSC jniObject = new NJSC();
        jniObject.info();
        jniObject.hasArgument(5);
        int x = jniObject.sum(2,3);
        System.out.println(x);

        String s = jniObject.makeString();
        System.out.println(s);

        String sb = jniObject.concat("hello", "world");
        System.out.println(sb);

        jniObject.concat(null, "world");
        jniObject.concat("hello", null);
    }

}
