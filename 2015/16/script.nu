def main [] {}

const measured = {
  children: 3
  cats: 7
  samoyeds: 2
  pomeranians: 3
  akitas: 0
  vizslas: 0
  goldfish: 5
  trees: 3
  cars: 2
  perfumes: 1
}

def "main first" [] { prepare-input | where { get compounds | all {|x| $x.value == ($measured | get $x.name) } } | get 0.number }

def "main second" [] {
  prepare-input | where { get compounds | all {|x| (
    $x.name in [cats trees] and $x.value > ($measured | get $x.name)
    or $x.name in [pomeranians goldfish] and $x.value < ($measured | get $x.name)
    or $x.name not-in [cats trees pomeranians goldfish] and $x.value == ($measured | get $x.name)
  ) } } | get 0.number
}

def prepare-input [] {
  lines | parse 'Sue {number}: {compounds}' | into int number | update compounds {
    split row ', ' | split column ': ' name value | into int value
  }
}
