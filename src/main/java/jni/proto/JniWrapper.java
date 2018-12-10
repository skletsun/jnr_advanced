package jni.proto;

public class JniWrapper {
    public JniWrapper() {
        System.loadLibrary("wrapper");
    }

    public native int div(int a, int b);
}
