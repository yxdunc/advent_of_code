for x in $@; do
  result=$(echo $x / 3 - 2 | bc)
  total=0
  while ((result > 0)); do
    total="$total + $result"
    result=$(echo $result / 3 - 2 | bc)
  done
  echo $total | bc
done | paste -s -d+ - | bc
