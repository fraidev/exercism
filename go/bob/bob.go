// This is a "stub" file.  It's a little start on your solution.
// It's not a complete solution though; you have to write some code.

// Package bob should have a package comment that summarizes what it's about.
// https://golang.org/doc/effective_go.html#commentary
package bob

import (
	"strings"
	"unicode"
)

// Hey should have a comment documenting it.
func Hey(remark string) string {
	// Write some code here to pass the test suite.
	// Then remove all the stock comments.
	// They're here to help you get started but they only clutter a finished solution.
	// If you leave them in, reviewers may protest!

	sentence := strings.TrimSpace(remark);

	if len(sentence) == 0 {
		return "Fine. Be that way!"
	}

	if IsUpper(sentence) && IsAsk(sentence) {
		return "Calm down, I know what I'm doing!"
	}

	if IsUpper(sentence) {
		return "Whoa, chill out!"
	}

	if IsAsk(sentence) {
		return "Sure."
	}
ex
	return "Whatever."
}

func IsUpper(s string) bool {
	hasLatter := false
	for _, r := range s {
		if !unicode.IsUpper(r) && unicode.IsLetter(r) {
			return false
		}
		if unicode.IsLetter(r) {
			hasLatter = true
		}
	}

	return hasLatter
}

func IsAsk(s string) bool {
	if len(s) == 0 {
		return false
	}
	if s[len(s)-1] == '?' {
		return true
	}
	return false
}
