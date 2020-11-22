#!/bin/bash

CSV_FILE=datampiopenmp_3_8threads.csv
BINARY=generate_mpi_openmp
CLUSTER_DIR=cluster

generateexcelmpiopenmp() {

  schedules=(static dynamic guided)
  threads=(2222)
  numbers=(100000 300000 600000)
  hosts_ens=(ens1 ens2 ens4 ens5)
  # numbers=(100 1000 10000)

  echo "SECONDS;LIMIT;THREADSMPI;SCHEDULE;THREADSOPENMP" >> $CSV_FILE
  for schedule in "${schedules[@]}"
  do

    for thread in "${threads[@]}"
    do

      for number in "${numbers[@]}"
      do

        _cluster_file=$CLUSTER_DIR/$thread
        echo "Running ${schedule} with ${thread} threads with ${number} number for cluster file ${_cluster_file}"
        # export OMP_NUM_THREADS="$thread"
        export OMP_SCHEDULE="$schedule"

        mpirun --machinefile $_cluster_file ./$BINARY $number bf >> $CSV_FILE
        echo >> $CSV_FILE # break line
      done

    done

  done
}

rm $CSV_FILE
generateexcelmpiopenmp