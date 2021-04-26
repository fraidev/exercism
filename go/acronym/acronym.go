// This is a "stub" file.  It's a little start on your solution.
// It's not a complete solution though; you have to write some code.

// Package acronym should have a package comment that summarizes what it's about.
// https://golang.org/doc/effective_go.html#commentary
package acronym

import (
	"strings"
)

// Abbreviate should have a comment documenting it.
func Abbreviate(s string) string {
	// Write some code here to pass the test suite.
	// Then remove all the stock comments.
	// They're here to help you get started but they only clutter a finished solution.
	// If you leave them in, reviewers may protest!

	w := strip(s)

	words := strings.Split(w, " ")

	var sb strings.Builder

	for _, word := range words {
		if len(word) > 0 {
			sb.WriteString(string(word[0]))
		}
	}
	return strings.ToUpper(sb.String())
}

func strip(s string) string {
	var result strings.Builder
	for i := 0; i < len(s); i++ {
		b := s[i]
		if ('a' <= b && b <= 'z') ||
			('A' <= b && b <= 'Z') ||
			('0' <= b && b <= '9') ||
			b == ' ' || b == '\'' {
			result.WriteByte(b)
		} else {
			result.WriteByte(' ')
		}
	}
	return result.String()
}
