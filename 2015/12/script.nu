def main [] {}

def "main first" [] { from json | sum }

def "main second" [] { from json | sum --ignore-red }

def sum [--ignore-red] {
  match ($in | describe -d | get type) {
    int => $in
    list => ($in | each { sum --ignore-red=$ignore_red } | append 0 | math sum)
    record => (if $ignore_red and ($in | values | any { $in == red }) { 0 } else { ($in | values | each { sum --ignore-red=$ignore_red } | append 0 | math sum) })
    _ => 0
  }
}
