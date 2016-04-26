#!/bin/bash
for i in 1 2 4 8 ; do (~/Desktop/Comp140/sniper/run-sniper -d thread$i\_output -c gainestown -c nehalem.cfg -n 8 -- ./dotproduct $i &) ; done
