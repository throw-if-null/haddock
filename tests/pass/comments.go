// Package main reads the retry limit. Only comments are checked in a source file.
package main

import "fmt"

/*
 * Is the limit read once?
 * TODO: read the limit on every request!
 */
func main() {
	x := "just magic — really robust; feel free!" /* The value is a string. */
	fmt.Println(x) // Is this just magic? A comment after code is not checked.
}
