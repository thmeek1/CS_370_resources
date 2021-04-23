/**
 * Interface for shared memory.
 *
 * Figure 3.15
 * Operating System Concepts with Java - Eighth Edition
 * Copyright John Wiley &amp; Sons - 2010.
 *
 * @author Gagne, Galvin, Silberschatz
 */
public interface Buffer <E> {
	// producers call this method
	public void insert(E item);

	// consumers call this method
	public E remove();
}
