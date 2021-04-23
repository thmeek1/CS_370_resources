/**
 * A example program illustrating shared memory (dangerous, no precautions taken)
 * The point here is how easy it is to share memory in Java, and perhaps not realize.
 * This is not a GOOD example of how to share memory!!!!
 */

import java.util.concurrent.*;

public class Shared
{
    public static void main(String[] args) {
        if (args.length < 1) {
            System.out.println("Usage: java Shared <num tasks>");
            System.exit(1);
        }
        int numTasks = Integer.parseInt(args[0].trim());
        
        // create the thread pool (create as needed, but re-use)
        ExecutorService pool = java.util.concurrent.Executors.newFixedThreadPool(2);

        Foo danger = new Foo(42);

        System.out.println("Danger Will Robinson???? " + danger.getValue());

        // run each task using a thread in the pool
        for (int i = 0; i < numTasks; i++) {
            pool.execute(new Task(danger));
        }
        
        // sleep for 5 seconds
        try { Thread.sleep(5000); } catch (InterruptedException ie) { }
        System.out.println("Danger Will Robinson!!!! " + danger.getValue());

        pool.shutdown();
    }
}


class Task implements Runnable {
    private Foo thing;
    public Task(Foo thing) {
        this.thing = thing;
    }
    public void run() {
        System.out.println(thing.getValue() + " -- I am working on a task: " + Thread.currentThread().getId());
        thing.setValue(999);
    }
}

class Foo {
    private int value;

    public Foo(int value) {
        this.value = value;
    }
    public void setValue(int value) {
        this.value = value;
    }

    public int getValue() {
        return this.value;
    }

}
