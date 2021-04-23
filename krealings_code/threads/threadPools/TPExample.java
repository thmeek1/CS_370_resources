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

public class TPExample
{
    public static void main(String[] args) {
        if (args.length < 1) {
            System.out.println("Usage: java TPExample <num tasks>");
            System.exit(1);
        }
        int numTasks = Integer.parseInt(args[0].trim());
        
        // create the thread pool (create as needed, but re-use)
        ExecutorService pool = java.util.concurrent.Executors.newFixedThreadPool(2);

        // run each task using a thread in the pool
        for (int i = 0; i < numTasks; i++)
            pool.execute(new Task());
        
        // sleep for 5 seconds
        try { Thread.sleep(5000); } catch (InterruptedException ie) { }

        pool.shutdown();
    }
}


class Task implements Runnable {
    public void run() {
        System.out.println("I am working on a task: " + Thread.currentThread().getId());
    }
}
