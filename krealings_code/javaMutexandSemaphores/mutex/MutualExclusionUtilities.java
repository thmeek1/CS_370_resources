/**
 * MutualExclusionUtilities.java
 *
 * Utilities for simulating critical and non-critical sections.
 */

public class MutualExclusionUtilities
{
   /**
    * critical and non-critical sections are simulated by sleeping
    * for a random amount of time between 0 and 3 seconds.
    */
   public static void criticalSection(String name) {
      SleepUtilities.nap(3);
   }

   public static void nonCriticalSection(String name) {
      SleepUtilities.nap(3);
   }
}
