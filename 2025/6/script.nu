def main [] {}

def "main first" [] { detect columns -n | roll down | values  | each { if $in.0 == + { skip | into int | math sum } else { skip | into int | math product } } | math sum }

def "main second" [] {
  let table = lines | split chars | each { wrap a | transpose -id }
  let operators = $table | last | values | where $it != ' '
  let values = $table | drop | values | each { where $it != ' ' } | split list [] | each { each { str join } | into int }
  $operators | zip $values | each { if $in.0 == + { $in.1 | math sum } else { $in.1 | math product } } | math sum
}
