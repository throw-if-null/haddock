<!-- Rule: in source code, line comments and block comments are checked. A leading * in a block comment is not a list marker. -->
package main

// The retry limit is load-bearing.
func main() {
	/* The worker is just a loop. */
	limit := 3 /* The inline block is really robust. */
	_ = limit
}

/*
 * The scheduler talks to the queue. It sends each job to a worker, waits for the
 * result, writes the result to the log, and retries the job later when the worker fails.
 */
