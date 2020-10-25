#!/bin/bash

CSV_FILE=data.csv
BINARY=generate

generateexcell() {

  schedules=(static dynamic guided auto)
  threads=(1 2 4 6 8)
  # numbers=(100000 300000 600000)
  numbers=(100 1000 10000)

  echo "SECONDS;LIMIT;THREADS;SCHEDULE" >> $CSV_FILE
  for schedule in "${schedules[@]}"
  do

    for thread in "${threads[@]}"
    do

      for number in "${numbers[@]}"
      do

        echo "Running ${schedule} with ${thread} threads with ${number} number"
        export OMP_NUM_THREADS="$thread"
        export OMP_SCHEDULE="$schedule"

        ./$BINARY $number bf >> $CSV_FILE
        echo >> $CSV_FILE
      done

    done

  done
}

rm $CSV_FILE
generateexcell