public class Example {

    private int value;

    public Example() {
        value = 0;
    }

    public static int test3(int ah, int bh, int ch) {
        System.out.println(new Example().value);
        return 1;
    }

    public static int main(String[] args) {
        System.out.println(new Example().value);
        return 1;
    }

    public int test() {
        System.out.println("Hello World!");
        return 1;
    }

    public int test2() {
        return 1;
    }
}