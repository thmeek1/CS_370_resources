/**
 * A example program illustrating message passing
 *
 */

import java.util.concurrent.*;
import java.util.ArrayList;

public class Message
{
    public static void main(String[] args) {
        if (args.length < 1){
            System.out.println("Usage: java TPExample <num tasks>");
            System.exit(1);
        }
        int numTasks = Integer.parseInt(args[0].trim());
        
        // create the thread pool (create as needed, but re-use)
        ExecutorService pool = java.util.concurrent.Executors.newFixedThreadPool(2);

        // run each task using a thread in the pool
        for (int i = 0; i < numTasks; i++) {
            System.out.println("Submitting " + i);
            Future item = pool.submit(new Task(i));
            try {
                System.out.println("I got " + item.get());
            } catch(Exception e) {};
        }
        
        // sleep for 5 seconds
        try { Thread.sleep(5000); } catch (InterruptedException ie) { }

        pool.shutdown();
    }
}


class Task implements Callable {
    private int num;

    public Task(int value) {
            num = value;
    }
    public Long call() {
        if (num == 2)
            try { Thread.sleep(5000); } catch (InterruptedException ie) { }
        System.out.println("I am working on a task: " + Thread.currentThread().getId());
        return num + Thread.currentThread().getId();
    }
}
