#!/bin/bash

echo "Building Rom... Please Wait"
mkdir out/$1
cd decomp/pokefirered/
make -l12
mv pokeefirered.gba ../../out/$1/pokefirered.gba
echo "Completed Building Rom, see above line to see if process succeded"