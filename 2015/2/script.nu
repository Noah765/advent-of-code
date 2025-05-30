def main [] {}

def "main first" [] {
  prepare-input | each {
    let sides = [($in.l * $in.w) ($in.l * $in.h) ($in.w * $in.h)]
    2 * ($sides | math sum) + ($sides | math min)
  } | math sum
}

def "main second" [] { prepare-input | each { 2 * ([($in.l + $in.w) ($in.l + $in.h) ($in.w + $in.h)] | math min) + $in.l * $in.w * $in.h } | math sum }

def prepare-input [] { lines | parse '{l}x{w}x{h}' | into int l w h }
