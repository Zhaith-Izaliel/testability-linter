public class Example {

    private int value;

    public Example() {
        value = 0;
    }

    public void test3(int ah, int bh, int ch) {
        System.out.println(new Example().value);
    }

    public static int main(String[] args) {
        System.out.println(new Example().value);
        return 1;
    }

    public void test() {
        System.out.println("Hello World!");
    }

    public void test2() {
    }
}