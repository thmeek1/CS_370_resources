/**
 * This creates the buffer and the producer and consumer threads.
 *
 * Figure 6.14
 * Operating System Concepts with Java - Eighth Edition
 * Copyright John Wiley &amp; Sons - 2010.
 *
 * @author Gagne, Galvin, Silberschatz
 * @author Modified by William Kreahling
 */

import java.util.Date;

public class Factory
{
	public static void main(String args[]) {
		Buffer<Date> server = new BoundedBuffer<Date>();
		
		// now create the producer and consumer threads
		Thread producerThread = new Thread(new Producer(server));
		Thread consumerThread = new Thread(new Consumer(server));
		
		producerThread.start();
		consumerThread.start();               
	}
}
