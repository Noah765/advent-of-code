def main [] {}

def "main first" [] {
  let input = prepare-input
  $input.available | where ($input.fresh | any { $it in $in.start..$in.end }) | length
}

def "main second" [] {
  prepare-input | get fresh | reduce {|a| each {|b|
    if $b.end < $a.start or $b.start > $a.end { return $b }
    [
      (if $b.start < $a.start { $b | update end ($a.start - 1) })
      (if $b.end > $a.end { $b | update start ($a.end + 1) })
    ]
  } | flatten | compact | append $a } | each { $in.end - $in.start + 1 } | math sum
}

def prepare-input [] { lines | split list '' | { fresh: ($in.0 | parse '{start}-{end}' | into int start end) available: ($in.1 | into int) } }
