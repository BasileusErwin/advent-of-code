BEGIN { total = 0; disabled = 0; }

{
  while (match($0, /mul\([0-9]+,[0-9]+\)|do\(\)|don't\(\)/)) {
    instr = substr($0, RSTART, RLENGTH)

    $0 = substr($0, RSTART + RLENGTH)

    if (instr == "do()") {
      disabled = 0
    } else if (instr == "don't()") {
      disabled = 1
    } else if (instr ~ /mul\([0-9]+,[0-9]+\)/ && !disabled) {
      match(instr, /mul\(([0-9]+),([0-9]+)\)/, expr)
      total += expr[1] * expr[2]
    }
  }
}

END { print "Part 2: " total }
