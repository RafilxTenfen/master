package main

import (
	"fmt"

	"github.com/RafilxTenfen/master/go-gallon/gallons"
)

func main() {
	bottleGroup := gallons.CreateBottleGroup(2)

	fmt.Printf("bottle %+v", bottleGroup)
}
