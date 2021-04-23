/**
 * This is the consumer thread for the bounded buffer problem.
 *
 * Figure 6.13
 * Operating System Concepts with Java - Eighth Edition
 * Copyright John Wiley &amp; Sons - 2010.
 *
 * @author Gagne, Galvin, Silberschatz
 */
import java.util.Date;

public class Consumer implements Runnable {
    private  Buffer<Date> buffer;

    public Consumer(Buffer<Date> buffer) { 
        this.buffer = buffer;
    }
   
    public void run()
    {
        Date message;
   
        while (true)
        {
            System.out.println("Consumer napping");
	        SleepUtilities.nap(); 
         
            // consume an item from the buffer
            System.out.println("Consumer wants to consume.");
           
            message = buffer.remove();
        }
   }
   
}


