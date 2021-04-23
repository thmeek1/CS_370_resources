/**
 * Worker.java
 * 
 * This thread is used to demonstrate the operation of a binary semaphore (AKA a mutex).
 * 
 * Figure 6.8
 * Operating System Concepts with Java - Eighth Edition
 * Copyright John Wiley &amp; Sons - 2010.
 *
 * @author Gagne, Galvin, Silberschatz
 * @author Modifed by William Kreahling
 */
import java.util.concurrent.Semaphore;
public class Worker implements Runnable {

	private Semaphore mutex;

	private String name;

	public Worker(Semaphore mutex, String name) {
		this.name = name;
		this.mutex = mutex;
	}

	public void run() {
		while (true) {
			try {
                mutex.acquire();
      		    System.out.println(name + " is in critical section");
		    	MutualExclusionUtilities.criticalSection(name);
      		    System.out.println(name + " is out of critical section");
			    mutex.release();
		    	MutualExclusionUtilities.nonCriticalSection(name);
            } catch(InterruptedException ie) {
                // We should really handle this :(
            }
		}
	}

}
