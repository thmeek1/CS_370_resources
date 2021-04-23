/**
 * SemaphoreFactory.java
 *
 * Figure 6.8.
 * Operating System Concepts with Java - Eighth Edition
 * Copyright John Wiley &amp; Sons - 2010.
 *
 * This program uses a semaphore as a means of handling synchronization.  It creates 5 threads and a thread can
 * perform its critical section only if it is able to complete a acquire() operation on the sempahore.
 *
 * @author Gagne, Galvin, Silberschatz
 * @author Modified by William Kreahling
 */
import java.util.concurrent.Semaphore;
public class SemaphoreFactory
{
   public static void main(String args[]) {
      Semaphore mutex = new Semaphore(1); // binary semaphore (aka a mutex)

      Thread[] bees = new Thread[5];

      for (int i = 0; i < 5; i++)
         bees[i] = new Thread(new Worker(mutex, "Worker " + i));

      for (int i = 0; i < 5; i++)
         bees[i].start();
   }
}

