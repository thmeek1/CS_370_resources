import java.util.Date;

// Step 1
public class Example2 implements Runnable {
    private String message;
    private static final int DELAY = 10000;

    public Example2(String message) {
        this.message = message;
    }

    // Step 2
    public void run() {
        try {
            if (Thread.currentThread().getId() == 13) {
                System.out.println("BOOM " + Thread.currentThread().getId());
              //  System.exit(1);
            } else {
                System.out.println("FOOM " + Thread.currentThread().getId());
            }
            Thread.sleep(100);
            Date current = new Date();
            System.out.println(current + " : " + message + ": " + Thread.currentThread().getId());
            /*
             * Putting a thread to sleep can be risky. It may go to sleep for so long that it is not useful anymore
             * and should be terminated. When a sleeping thread is interrupted it generates an InterruptedException.
             * You should catch this Exception and then terminate the thread.
             */
        } catch(InterruptedException ie) {
            // Cleanup code goes here. In this case I don't need to do anything
        }
    }
}
