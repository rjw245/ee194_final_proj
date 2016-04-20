#!/bin/bash
for i in 1 2 4 8 16 ; do (~/ee194/sniper/run-sniper -d thread$i\_output -c gainestown -c nehalem.cfg -n 8 -- ./dotproduct $i &) ; done
