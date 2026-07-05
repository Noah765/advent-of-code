def main [] {}

def "main first" [] {
  prepare-input | reduce -f {state: 50 count: 0} {|x acc|
    let state = ($acc.state + $x) mod 100
    {state: $state count: (if $state == 0 { $acc.count + 1 } else { $acc.count })}
  } | get count
}

def "main second" [] {
  prepare-input | reduce -f {state: 50 count: 0} {|x acc|
    let new_count = if $x > 0 { ($acc.state + $x) // 100 } else { ((0 - $acc.state) mod 100 - $x) // 100 }
    {state: (($acc.state + $x) mod 100) count: ($acc.count + $new_count)}
  } | get count
}

def prepare-input [] { lines | each { str replace R '' | str replace L '-' | into int } }
