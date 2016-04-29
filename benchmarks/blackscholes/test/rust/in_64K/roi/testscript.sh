for i in 1 2 4 8 ; do (~/ee194/sniper/run-sniper -d ./thread$i\_output_roi -n 8 -c gainestown -c nehalem.cfg --roi -- ../../../../rust/bs $i ../../../../inputs/in_64K.txt &) ; done
