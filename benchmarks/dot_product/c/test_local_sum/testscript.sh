SNIPER_DIR=~/Desktop/Comp140/sniper

for i in 8
do
  make NT=$i
  $SNIPER_DIR/run-sniper -d thread$i\_output -n 8 -c gainestown -c nehalem.cfg --roi -- ./dot_product
done
