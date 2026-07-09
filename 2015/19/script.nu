def main [] {}

def "main first" [] {
  let input = prepare-input

  ..<($input.molecule | str length) | each {|i| $input.replacements | each {|x|
    (($input.molecule | str substring ..<$i)
    + ($input.molecule | str substring $i.. | str replace $x.find $x.replace))
  } } | flatten | uniq | where $it != $input.molecule | length
}

def "main second" [] {
  prepare-input | get molecule | split chars | skip | where $it == ($it | str upcase)
  | each { if $in in [R Y] { -1 } else { 1 } } | math sum
}

def prepare-input [] { parse "{replacements}\n\n{molecule}" | get 0 | update replacements { lines | parse '{find} => {replace}' } }
