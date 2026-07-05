def main [] {}

def "main first" [] {
  mut grid = lines | split chars | each { each { match $in { . => 0 S => 1 ^ => 2 } } }
  mut split_count = 0

  for y in 0..<(($grid | length) - 1) {
    for x in ($grid | get $y | enumerate | where item == 1 | get index) {
      match ($grid | get ($y + 1) | get $x) {
        0 => ($grid = $grid | update ($y + 1) { update $x 1 })
        2 => {
          $split_count += 1
          $grid = $grid | update ($y + 1) { update ($x - 1) 1 | update ($x + 1) 1 }
        }
      }
    }
  }

  $split_count
}

def "main second" [] {
  mut grid = lines | split chars | each { each { match $in { . => 0 S => 1 ^ => -1 } } }

  for y in 0..<(($grid | length) - 1) {
    for x in ($grid | get $y | enumerate | where item > 0) {
      match ($grid | get ($y + 1) | get $x.index) {
        -1 => ($grid = $grid | update ($y + 1) { update ($x.index - 1) { $in + $x.item } | update ($x.index + 1) { $in + $x.item } })
        _ => ($grid = $grid | update ($y + 1) { update $x.index { $in + $x.item } })
      }
    }
  }

  $grid | last | math sum
}
