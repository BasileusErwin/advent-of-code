{
  while (match($0, /mul\([0-9]+,[0-9]+\)/)) {
    expr = substr($0, RSTART, RLENGTH);
    split(expr, nums, /[(),]/);
    total += nums[2] * nums[3];
    $0 = substr($0, RSTART + RLENGTH);
  }
}
END { print "Part 1: " total }
