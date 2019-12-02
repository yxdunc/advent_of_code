for x in $(cat input.txt); do
  echo $x / 3 - 2 | bc
done | paste -s -d+ - | bc
