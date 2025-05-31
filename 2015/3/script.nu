def main [] {}

def "main first" [] {
  mut locations = [[0 0]]
  mut position = [0 0]
  for x in ($in | split chars) {
    match $x {
      ^ => ($position.1 += 1)
      v => ($position.1 -= 1)
      > => ($position.0 += 1)
      < => ($position.0 -= 1)
    }
    $locations = [$position ...$locations]
  }
  $locations | uniq | length
}

def "main second" [] {
  mut locations = [[0 0]]
  mut santa_position = [0 0]
  mut robo_santa_position = [0 0]
  for x in ($in | split chars | enumerate) {
    if $x.index mod 2 == 0 {
      match $x.item {
        ^ => ($santa_position.1 += 1)
        v => ($santa_position.1 -= 1)
        > => ($santa_position.0 += 1)
        < => ($santa_position.0 -= 1)
      }
      $locations = [$santa_position ...$locations]
    } else {
      match $x.item {
        ^ => ($robo_santa_position.1 += 1)
        v => ($robo_santa_position.1 -= 1)
        > => ($robo_santa_position.0 += 1)
        < => ($robo_santa_position.0 -= 1)
      }
      $locations = [$robo_santa_position ...$locations]
    }
  }
  $locations | uniq | length
}
