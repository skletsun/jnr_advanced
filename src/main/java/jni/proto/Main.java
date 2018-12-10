package jni.proto;

import jnr.ffi.Runtime;
import jnr.ffi.*;

public class Main {
    public interface First {
        int add(int a, int b);
        // this one dynamically depends on second::mult(a, b)
        int mult(int a, int b);
    }

    public interface Second {
        int sub(int a, int b);
    }

    public static void main(String[] args) {
        testSecondFirst();
        testFirstSecond();
        System.out.println("=== DONE ===");
    }

    private static void testSecondFirst() {
        System.out.println("=== Test1: Load \"second\" library first, then \"first\" ===");
        loadAndTestSecond();
        loadAndTestFirst();
    }

    private static void testFirstSecond() {
        System.out.println("=== Test2: Load \"first\" library first - this should implicitly load \"second\" ===");
        loadAndTestFirst();
        loadAndTestSecond();
    }

    private static void loadAndTestSecond() {
        System.out.println("[Loading library \"second\"]");
        Second secondLib = LibraryLoader.create(Second.class).load("second");
        Runtime runtime = Runtime.getRuntime(secondLib);
        System.out.print("Java: Second.sub(4, 2) =>");
        System.out.println(secondLib.sub(4, 2));
        System.out.print("Java: JniWrapper.div(42, 21) =>");
        System.out.println(new JniWrapper().div(42, 21));
    }

    private static void loadAndTestFirst() {
        System.out.println("[Loading library \"first\"]");
        First firstLib = LibraryLoader.create(First.class).load("first");
        Runtime runtime = Runtime.getRuntime(firstLib);

        System.out.print("Java: First.add(2, 1) =>");
        System.out.println(firstLib.add(2, 1));
        // This method calls
        System.out.print("Java: First.mult(3, 4) => ");
        System.out.println(firstLib.mult(3, 4));
    }
}
