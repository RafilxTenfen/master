package gallons

// Bottle struct to store volume
type Bottle struct {
	VolumeML uint32
}

// NewBottle returns a new Bottle struct
func NewBottle(volume uint32) Bottle {
	return Bottle{
		VolumeML: volume,
	}
}

// CreateBottleGroup based on parameters
func CreateBottleGroup(volumes ...uint32) []Bottle {
	group := make([]Bottle, len(volumes))

	for i, v := range volumes {
		group[i] = NewBottle(v)
	}

	return group
}
