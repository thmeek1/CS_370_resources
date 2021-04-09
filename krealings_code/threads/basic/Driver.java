import java.util.ArrayList;

public class Driver {

    public static void main(String[] args){
        System.out.println("I am the main thread: " + Thread.currentThread().getId());

        // Step 3
        //Runnable threadOne = new Example("Easter Candy");
        // Step 4
        //Thread thread1 = new Thread(threadOne);

        ArrayList<Thread> children = new ArrayList<Thread>();

        for (int i = 0; i < 5; i++) {
                         //Step 4       // Step 3
            children.add(new Thread(new Example("Easter Candy " + i)));
        }
        for (int i = 0; i < 5; i++) {
            children.get(i).start(); // Step 5
        }
//        try {
//            for (int i = 0; i < 5; i++) {
//                children.get(i).join(); // Step 5
//            }
//        }catch(Exception e) {}; // for example purposes, bad style
        System.out.println("MAIN thread is terminating");
    }
}

