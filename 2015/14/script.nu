def main [] {}

def "main first" [] {
  prepare-input | each {(
    2503 // ($in.flight_time + $in.rest_time) * $in.velocity * $in.flight_time
    + ([(2503 mod ($in.flight_time + $in.rest_time)) $in.flight_time] | math min) * $in.velocity
  )} | math max
}

def "main second" [] {
  let start = (prepare-input | insert distance 0)
  0..<2503 | generate {|time x|
    let x = $x | update distance {|x| if $time mod ($x.flight_time + $x.rest_time) < $x.flight_time { $in + $x.velocity } else { $in } }
    let max_distance = $x.distance | math max
    {out: ($x | where distance == $max_distance | get name) next: $x}
  } $start | flatten | uniq -c | get count | math max
}

def prepare-input [] {
  lines
  | parse '{name} can fly {velocity} km/s for {flight_time} seconds, but then must rest for {rest_time} seconds.'
  | into int velocity flight_time rest_time
}
