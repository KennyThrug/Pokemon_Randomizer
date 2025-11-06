#!/bin/bash

echo "Building Rom... Please Wait"
mkdir out/$1
cd decomp/pokeemerald-expansion/
# git restore data/maps/.
make -j12
mv pokeemerald.gba ../../out/$1/pokeemerald.gba
echo "Completed Building Rom, see above line to see if process succeded"