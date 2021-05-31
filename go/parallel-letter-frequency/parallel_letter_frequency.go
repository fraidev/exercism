package letter

// FreqMap records the frequency of each rune in a given text.
type FreqMap map[rune]int

// Frequency counts the frequency of each rune in a given text and returns this
// data as a FreqMap.
func Frequency(s string) FreqMap {
	m := FreqMap{}
	for _, r := range s {
		m[r]++
	}
	return m
}

func ConcurrentFrequency(strings []string) FreqMap {
	freq := FreqMap{}

	channel := make(chan FreqMap)

	for _, s := range strings {
		go func(s string) {
			channel <- Frequency(s)
		}(s)
	}

	for range strings {
		chanMap := <-channel
		for k, v := range chanMap {
			freq[k] += v
		}
	}

	return freq
}
