for i in 1 2 8 32 64 ; do (~/ee194/sniper/run-sniper -d ./thread$i\_output -n $i -c gainestown -c nehalem.cfg -- ../../../rust/bs $i ../../../inputs/in_64K.txt &) ; done
