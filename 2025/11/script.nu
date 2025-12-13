def main [] {}

def "main first" [] { prepare-input | generate-device-path-counts you  out }

def "main second" [] {
  let devices = prepare-input
  (($devices | generate-device-path-counts svr fft) * ($devices | generate-device-path-counts fft dac) * ($devices | generate-device-path-counts dac out) +
  ($devices | generate-device-path-counts svr dac) * ($devices | generate-device-path-counts dac fft) * ($devices | generate-device-path-counts fft out))
}

def prepare-input [] { lines | parse '{name}: {outputs}' | update outputs { split row ' ' } }

def generate-device-path-counts [start finish] {
  let devices = $in | insert path_count { if $in.name == $finish or $finish == out and $in.outputs == [out] { 1 } else { null } }
  generate {|devices|
    let next = $devices | update path_count {|x|
      if $in != null { return $in }
      $x.outputs | generate {|x sum=0| $devices | where name == $x | default -e [{path_count: 0}] | first | get path_count | if $in == null { {out: null} } else { {out: ($sum + $in) next: ($sum + $in)} } } | last
    }
    $next | where name == $start | first | get path_count | if $in == null { {next: $next} } else { {out: $in} }
  } $devices | first
}
