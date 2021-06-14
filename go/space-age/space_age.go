package space

import (
	"fmt"
)

type Planet string

func Age(seconds float64, planet Planet) float64 {
	fmt.Println(planet)
	minutes := (seconds / 60)
	hours := (minutes / 60)
	days := hours / 24
	years := days / 365.25

	multiplier := 0.0
	switch planet {
	case "Mercury":
		multiplier = 0.2408467
	case "Venus":
		multiplier = 0.61519726
	case "Earth":
		multiplier = 1.0
	case "Mars":
		multiplier = 1.8808158
	case "Jupiter":
		multiplier = 11.862615
	case "Saturn":
		multiplier = 29.447498
	case "Uranus":
		multiplier = 84.016846
	case "Neptune":
		multiplier = 164.79132
	}

	return years / multiplier
}
