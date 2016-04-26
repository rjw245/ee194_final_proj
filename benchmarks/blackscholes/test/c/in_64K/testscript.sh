#!/bin/bash

../../../parsec-3.0/bin/parsecmgmt -a build -p blackscholes -c gcc-serial 

~/ee194/sniper/run-sniper -d ./thread1_output -n 8 -c gainestown -c nehalem.cfg -- ../../../parsec-3.0/pkgs/apps/blackscholes/inst/amd64-linux.gcc-serial/bin/blackscholes 1 ../../../inputs/in_64K.txt thread1_output/output.txt

../../../parsec-3.0/bin/parsecmgmt -a build -p blackscholes -c gcc-pthreads
     
for i in 2 4 8 16 ; do (~/ee194/sniper/run-sniper -d ./thread$i\_output -n 8 -c gainestown -c nehalem.cfg -- ../../../parsec-3.0/pkgs/apps/blackscholes/inst/amd64-linux.gcc-pthreads/bin/blackscholes $i ../../../inputs/in_64K.txt thread$i\_output/output.txt&) ; done

