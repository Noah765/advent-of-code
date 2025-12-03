def main [] {}

def "main first" [] { joltage-sum 2 }

def "main second" [] { joltage-sum 12 }

def joltage-sum [number_of_batteries] { lines | each { split chars | into int | largest-joltage $number_of_batteries } | math sum }

def largest-joltage [number_of_batteries] {
  if $number_of_batteries == 1 { return ($in | math max) }
  let first = $in | drop ($number_of_batteries - 1) | math max
  $first * 10 ** ($number_of_batteries - 1) + ($in | skip until { $in == $first } | skip | largest-joltage ($number_of_batteries - 1))
}
