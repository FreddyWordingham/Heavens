mkdir output/smooth

for VARIABLE in $(seq -f "%06g" 1 1050)
do
    # echo $VARIABLE
    convert output/$VARIABLE.png -filter Gaussian -blur 0x1 output/smooth/$VARIABLE.png
done
