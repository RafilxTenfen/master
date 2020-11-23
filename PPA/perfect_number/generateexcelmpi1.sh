#!/bin/bash

CSV_FILE=datampi_16.csv
BINARY=generate_mpi
CLUSTER_DIR=cluster

generateexcellmpi() {

  schedules=(MPI)
  # threads=(1 2 4 6 8 10 12 16 20 24)
  threads=(1)
  numbers=(100000 300000 600000)
  # numbers=(100 1000 10000)

  echo "SECONDS;LIMIT;THREADS;SCHEDULE" >> $CSV_FILE
  for schedule in "${schedules[@]}"
  do

    for thread in "${threads[@]}"
    do

      for number in "${numbers[@]}"
      do

        _cluster_file=$CLUSTER_DIR/$thread

        echo "Running ${schedule} with ${thread} threads with ${number} number for cluster file ${_cluster_file}"
        # export OMP_NUM_THREADS="$thread"
        # export OMP_SCHEDULE="$schedule"

        mpirun --machinefile $_cluster_file ./$BINARY $number bf >> $CSV_FILE
        echo >> $CSV_FILE # break line
      done

    done

  done
}

rm $CSV_FILE
generateexcellmpi