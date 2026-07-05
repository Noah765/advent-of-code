def main [] {}

def "main first" [] {
  let chars = split chars | group-by --to-table | update items { length } | transpose -rd
  ($chars.'('? | default 0) - ($chars.')'? | default 0)
}

def "main second" [] {
  mut floor = 0
  for x in ($in | split chars | enumerate) {
    if $x.item == '(' { $floor += 1 } else { $floor -= 1 }
    if $floor < 0 { return ($x.index + 1) }
  }
}
