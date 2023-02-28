public class Test {

    private int value;

    public Test() {
        value = 0;
    }

    public static int test3(int ah, int bh, int ch) {
        System.out.println(new Test().value);
        return 1;
    }

    public int test2() {
        return 1;
    }
}