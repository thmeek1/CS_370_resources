/**
 * A example program illustrating a cached thread pool
 *
 * Figure 4.17
 *
 * @author Gagne, Galvin, Silberschatz
 * Operating System Concepts with Java - Eighth Edition
 * Copyright John Wiley &amp; Sons - 2010.
 */

import java.util.concurrent.*;
import java.util.ArrayList;

public class Order
{
    public static void main(String[] args) {
        if (args.length < 1){
            System.out.println("Usage: java TPExample <num tasks>");
            System.exit(1);
        }
        int numTasks = Integer.parseInt(args[0].trim());
        
        // create the thread pool (create as needed, but re-use)
        ExecutorService pool = java.util.concurrent.Executors.newFixedThreadPool(2);

        ArrayList<Future> all = new ArrayList<Future>();

        // run each task using a thread in the pool
        for (int i = 0; i < numTasks; i++) {
            System.out.println("Submitting " + i);
            Future item = pool.submit(new Task(i));
            all.add(item);
        }

        // This is not an efficient way of getting values from the number of threads
        int index = 0;
        for (int i = 0; i < numTasks; ) {
            Future<Integer> item = all.get(index);
            if (item != null && item.isDone()) {
                try {
                    all.set(index, null);
                    Integer val = item.get();
                    System.out.println("I got " + item.get());
                    i++;
                } catch(Exception e) {};
            }
            index = (index + 1) % numTasks;
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
    public Integer call() {
        if (num == 2)
            try { Thread.sleep(5000); } catch (InterruptedException ie) { }

        System.out.println("I (" + num + ") am working on a task: " + Thread.currentThread().getId());
        return num;
    }
}
