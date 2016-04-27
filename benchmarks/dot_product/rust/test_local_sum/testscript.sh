for i in 1 2 4 8 ; do (~/ee194/sniper/run-sniper -d ./thread$i\_output -n 8 -c gainestown -c nehalem.cfg -- ./dotproduct $i &) ; done
