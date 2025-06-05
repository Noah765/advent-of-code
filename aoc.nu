const nu_script_template = 'def main [] {}

def "main first" [] {
  prepare-input
}

def "main second" [] {
  null
}

def prepare-input [] {
  
}
'

const rust_script_template = 'use serde::Serialize;

fn main() {
  advent_of_code::run(first, second);
}

fn first(input: String) -> impl Serialize {
  
}

#[allow(unused_variables)]
fn second(input: String) -> impl Serialize {}
'

const tests_template = "{
  inputs: [
    ''
  ]
  outputs: [[first second]; [ null]]
}
"

# Generate boilerplate for an Advent of Code day.
def "main generate" [
  --year (-y): int
  --day (-d): int
  --rust (-r)
]: nothing -> nothing {
  let date = prepare-input $year $day --year-span (metadata $year).span --day-span (metadata $day).span --generate
  let path = $'($date.year)/($date.day)'

  let previous_fs_entries = glob $'($path)/**'

  let input_exists = $'($path)/input.txt' | path exists
  let script_file = glob $'($path)/{script.nu,src/main.rs}' | path relative-to (pwd) | get 0?
  let tests_exist = $'($path)/tests.nuon' | path exists

  if not ('SESSION' | path exists) {
    error make --unspanned {
      msg: $"The file 'SESSION' does not exist"
      help: $"Create the file and write the session cookie to it"
    }
  }
  if ('SESSION' | path type) != 'file' {
    error make --unspanned {
      msg: $"'SESSION' is not a file"
      help: $"Replace 'SESSION' with a file and write the session cookie to it"
    }
  }

  try {
    mkdir $path

    try {
      http get --headers {cookie: $'session=(open SESSION | str trim)'} $'https://adventofcode.com/($date.year)/day/($date.day)/input'
    } catch {
      if $in.debug =~ 'status code' {
        let code = $in.debug | parse -r 'status code (\d+)' | get capture0.0
        error make --unspanned {
          msg: $"Accessing 'https://adventofcode.com/($date.year)/day/($date.day)/input' failed with a status code of ($code)"
          help: $"Using a browser, verify that you can access 'https://adventofcode.com' and that the session cookie in 'SESSION' is still valid"
        }
      } else if $in.debug =~ 'Dns Failed' {
        error make --unspanned {
          msg: $"Unable to access 'https://adventofcode.com/($date.year)/day/($date.day)/input'"
          help: $"Verify that you can access 'https://adventofcode.com/($date.year)/day/($date.day)/input' using a browser"
        }
      }
      $in.raw
    } | str trim | save --force $'($path)/input.txt'
    if $input_exists { print --stderr $"(ansi yellow_bold)Warning:(ansi reset) The file '($path)/input.txt' has been overwritten" }

    match $script_file {
      null if $rust => {
        cargo init $path --name $'advent-of-code-($date.year)-($date.day)'
        cd $path
        cargo add --path '../../rust'
        cargo add 'serde'
        cd '../..'
        $rust_script_template | save --force $'($path)/src/main.rs'
      }
      null => ($nu_script_template | save $'($path)/script.nu')
      _ => (print --stderr $"(ansi yellow_bold)Warning:(ansi reset) Script generation was skipped because the file '($script_file)' already exists")
    }

    if not $tests_exist {
      $tests_template | save $'($path)/tests.nuon'
    } else {
      print --stderr $"(ansi yellow_bold)Warning:(ansi reset) Test data generation was skipped because the file '($path)/tests.nuon' already exists"
    }
  } catch {
    glob $'($path)/**' | reverse | where $it not-in $previous_fs_entries | each { rm $in }
    $in.raw
  }

  let generated_files = [(if $script_file == null { $"'($path)/(if $rust { 'src/main.rs' } else { 'script.nu' })'" }) (if not $tests_exist { $"'($path)/tests.nuon'" }) $"'($path)/input.txt'"] | compact
  print $"(ansi green)Successfully generated ($generated_files | drop | str join ', ')(if ($generated_files | length) > 1 { ' and ' } )($generated_files | last)(ansi reset)"
}

# Run the script for an Advent of Code puzzle, along with its associated tests.
def main [
  --year (-y): int
  --day (-d): int
  puzzle?: string # Either 'first' or 'second'
  --rounds (-r): int = 1 # The number of times to run the script, to reduce the variance in the timing output
  --test (-t): int # Only run that test
  --only-tests (-o) # Only run tests
  --no-tests (-n) # Don't run tests
]: nothing -> nothing {
  let date = (
    prepare-input $year $day $puzzle $rounds $test
    --year-span (metadata $year).span --day-span (metadata $day).span --puzzle-span (metadata $puzzle).span --rounds-span (metadata $rounds).span --test-span (metadata $test).span
    --only-tests=$only_tests --no-tests=$no_tests
  )
  let path = $'($date.year)/($date.day)'

  let rust_binary = if ($'($path)/src/main.rs' | path exists) {
    cd $path
    cargo build --release
    cd '../..'
    print ''
    $'($path)/target/release/(open $'($path)/Cargo.toml' | get package.name)'
  }

  if not $no_tests { run-tests $path $rust_binary $puzzle --test $test --test-span (metadata $test).span }
  if $only_tests { return }
  if not $no_tests { print '' }

  if not ($'($path)/input.txt' | path exists) {
    error make --unspanned {
      msg: $"The file '($path)/input.txt' does not exist"
      help: $"Enable the '--only-tests \(-o)' flag or create the file"
    }
  }

  $puzzle | default ['first' 'second'] | each {|x|
    print $'Running (ansi attr_bold)($path):($x)(ansi reset)'
    let result = run-script $path $rust_binary $x (open $'($path)/input.txt') --rounds $rounds
    print $'Finished after (ansi attr_bold)($result.time)(ansi reset) with(format-result $result)'
    if $result.type == 'error' { exit 1 }
    if $puzzle == null and $x == 'first' { print '' }
  }

  null
}

def prepare-input [year? day? puzzle? rounds? test? --year-span: any --day-span: any --puzzle-span: any --rounds-span: any --test-span: any --generate --only-tests --no-tests] {
  def check-parameter [name value span possible_values] {
    if $value == null or $value in $possible_values { return }
    let is_continuous_range = ($possible_values | describe) == 'range' or ($possible_values | describe) == 'list<int>' and ($possible_values | length) == ($possible_values | last) - $possible_values.0 + 1
    def highlight [x] { $x | to nuon | nu-highlight }
    error make {
      msg: $'Invalid ($name)'
      label: { text: $'($value | to nuon) is invalid' span: $span }
      help: (match $possible_values {
        [] => $'Do not provide a ($name)'
        [$x] => $'The provided ($name) must be (highlight $x)'
        [$x $y] => $'The provided ($name) must be either (highlight $x) or (highlight $y)'
        _ if $is_continuous_range => $'The provided ($name) must be between (highlight $possible_values.0) and (highlight ($possible_values | last))'
        _ => $'The provided ($name) must be one of (highlight $possible_values)'
      })
    }
  }
  def make-wrong-filetype-error [file] {
    error make --unspanned {
      msg: $"'($file)' is not a file"
      help: $"Replace '($file)' with a file"
    }
  }

  let latest_date = (
    date now
    | into record
    | if $in.month == 12 { {year: $in.year day: ([$in.day 25] | math min)} } else { {year: ($in.year - 1) day: 25} }
  )

  let existing_puzzles = (
    glob --no-dir --no-symlink '<[0-9]>/<[0-9]>/{script.nu,src/main.rs}'
    | path relative-to (pwd) | path split | each { {year: $in.0 day: $in.1} } | into int year day
    | where year in 2015..($latest_date.year) and day in (if $it.year == $latest_date.year { 1..$latest_date.day } else { 1..25 })
    | uniq | sort-by year day
  )

  let possible_years = if $generate { 2015..($latest_date.year) } else { $existing_puzzles | get year | uniq }
  if $possible_years == [] {
    error make --unspanned {
      msg: 'This directory does not contain any Advent of Code puzzles'
      help: $"Run 'aoc generate' to generate boilerplate for an Advent of Code day"
    }
  }
  check-parameter 'year' $year $year_span $possible_years
  let year = match [$generate $year $day] {
    [true null null] => ($latest_date.year..2015 | filter {|x| ($existing_puzzles | where year == $x | length) < (if $x == $latest_date.year { $latest_date.day } else { 25 }) } | get 0? | default $latest_date.year)
    [false null null] => ($possible_years | last)
    [true null _] => ($latest_date.year..2015 | where ($it != $latest_date.year or $day <= $latest_date.day) and ({year: $it day: $day} not-in $existing_puzzles) | get 0? | default $latest_date.year)
    [false null _] => ($existing_puzzles | reverse | where day == $day | get 0?.year | default $latest_date.year)
    _ => $year
  }

  let possible_days = if $generate and $year == $latest_date.year { 1..$latest_date.day } else if $generate { 1..25 } else { $existing_puzzles | where year == $year | get day }
  check-parameter 'day' $day $day_span $possible_days
  let day = match [$generate $day] {
    [true null] => ($possible_days | where {year: $year day: $it} not-in $existing_puzzles | get 0? | default ($possible_days | last))
    [false null] => ($possible_days | last)
    _ => $day
  }

  let path = $'($year)/($day)'

  if ($'($path)/script.nu' | path type) == 'file' and ($'($path)/src/main.rs' | path type) == 'file' {
    error make --unspanned {
      msg: $"The '($path)' directory contains both a 'script.nu' and a 'src/main.rs' file"
      help: 'Delete one of the two files'
    }
  }
  if not $only_tests and ($'($path)/inputs.txt' | path type) not-in [null 'file'] { make-wrong-filetype-error $'($path)/inputs.txt' }
  if not $no_tests and ($'($path)/tests.nuon' | path type) not-in [null 'file'] { make-wrong-filetype-error $'($path)/tests.nuon' }

  check-parameter 'puzzle' $puzzle $puzzle_span ['first' 'second']
  if $rounds != null and $rounds < 1 { error make {msg: $'Invalid rounds' label: { text: $'($rounds) is invalid' span: $rounds_span } help: $'The provided rounds must be positive'} }

  if $generate or $no_tests { return {year: $year day: $day} }

  let raw_tests = if not ($'($path)/tests.nuon' | path exists) {
    {inputs: [] outputs: []}
  } else {
    let result = try-open-nuon $'($path)/tests.nuon'
    if $result.type == 'error' {
      $result.value | print --stderr
      exit 1
    }
    $result.value
  }

  let is_wrong_type = ($raw_tests | describe) !~ '^record<inputs: list<(string|any)>, outputs: (list<any>|table<first: [^,>]+, second: [^,>]+>)>$'
  let is_wrong_inputs_type = $is_wrong_type or ($raw_tests.inputs | describe) == 'list<any>' and $raw_tests.inputs != []
  let is_wrong_outputs_type = $is_wrong_type or ($raw_tests.outputs | describe) == 'list<any>' and $raw_tests.outputs != [] and ($raw_tests.outputs | columns) != ['first' 'second']
  if $is_wrong_inputs_type or $is_wrong_outputs_type {
    error make --unspanned {
      msg: $"The type of '($path)/tests.nuon' is incorrect"
      help: $"Expected 'record<inputs: list<string>, outputs: table<first: any, second: any>>' but got: '($raw_tests | describe)'"
    }
  }

  if ($raw_tests.inputs | length) != ($raw_tests.outputs | length) {
    let more = if ($raw_tests.inputs | length) > ($raw_tests.outputs | length) { 'inputs' } else { 'outputs' }
    let less = if $more == 'inputs' { 'outputs' } else { 'inputs' }
    error make --unspanned {
      msg: $"'($path)/tests.nuon' has more ($more) than ($less)"
      help: $'Either specify more ($less) or remove ($more)'
    }
  }

  let possible_tests = (
    $raw_tests.inputs | zip $raw_tests.outputs
    | each {|x| $x.1 | transpose puzzle output | insert input $x.0 }
    | enumerate | flatten --all | compact output
    | where $puzzle == null or puzzle == $puzzle | get index | uniq
  )
  check-parameter 'test' $test $test_span $possible_tests

  {year: $year day: $day}
}

def run-tests [path rust_binary puzzle --test: any --test-span: any] {
  let raw_tests = if ($'($path)/tests.nuon' | path exists) { open $'($path)/tests.nuon' } else { {inputs: [] outputs: []} }
  let tests = (
    $raw_tests.inputs | zip $raw_tests.outputs
    | each {|x| $x.1 | transpose puzzle output | insert input $x.0 }
    | enumerate | flatten --all | compact output
    | where ($puzzle == null or $it.puzzle == $puzzle) and ($test == null or $it.index == $test)
  )
  if $tests == [] {
    print $'(ansi green)There are no tests to run(ansi reset)'
    return
  }

  print $'Running ($tests | length) test(if ($tests | length) != 1 { 's' }) for (ansi attr_bold)($path)(ansi reset)'

  let test_results = $tests | par-each --keep-order {|x|
    let result = run-script $path $rust_binary $x.puzzle $x.input --collect-log

    let name = $'(ansi attr_bold)($in.puzzle):($in.index)(ansi reset)'
    if $result.type == 'success' and $result.value == $x.output {
      $'(ansi green_bold)SUCCESS(ansi reset)'
    } else {
      $'(ansi red_bold)FAILURE(ansi reset)'
    } | print $"test ($name) ... ($in) \(after ($result.time))"

    {
      name: $name
      expected: $x.output
      got: $result
    }
  }

  let failed = $test_results | where got.type == 'error' or got.value != $it.expected | get 0?
  if $failed != null {
    print $"\n(ansi red_bold)FAILURE(ansi reset) Test ($failed.name)"
    print $'(if $failed.got.log != '' { $"Log:\n($failed.got.log)\n\n" })Expected ($failed.expected | to nuon | nu-highlight) but got(format-result $failed.got)'
    exit 1
  }

  print $'(ansi green)All tests passed(ansi reset)'
}

def run-script [path rust_binary puzzle input --rounds: int = 1 --collect-log] {
  let result_file = mktemp --tmpdir 'aoc-run-result.nuon.XXXXX'
  rm $result_file
  let result = match $rust_binary {
    null => (run-nu-script $path $puzzle $input $rounds $result_file --collect-log=$collect_log)
    _ => (run-rust-script $path $rust_binary $puzzle $input $rounds $result_file --collect-log=$collect_log)
  }
  rm --force $result_file
  $result
}

def run-nu-script [path puzzle input rounds result_file --collect-log] {
  let code = $"
    source '($path)/script.nu'
    let input = $in
    def exit-with [output] {
      $output | to nuon --serialize | save --force ($result_file)
      exit
    }
    let results = 1..($rounds) | each {
      let start = date now
      let result = try { $input | main ($puzzle) | {type: 'success' value: $in} } catch { {type: 'error' value: $in.rendered} }
      let end = date now
      if $result.type == 'error' { exit-with {time: \($end - $start) ...$result} }
      {time: \($end - $start) value: $result.value}
    }
    let time = if ($rounds) == 1 { $results.0.time } else { $'\($results.time | math avg) +/- \($results.time | into int | into float | math stddev | into duration)' }
    let unique_values = $results.value | uniq
    if \($unique_values | length) > 1 { exit-with {time: $time type: 'inconsistent' value: $unique_values} }
    exit-with {time: $time type: 'success' value: $unique_values.0}
  "
  $env.FORCE_COLOR = true
  let log = if $collect_log { $input | nu --no-config-file --stdin --commands $code o+e>| collect } else { $input | try { nu --no-config-file --stdin --commands $code }; '' }
  let was_parse_error = not ($result_file | path exists)
  if $was_parse_error { {time: 0ns type: 'error' value: $log log: ''} } else { open $result_file | from nuon | insert log $log }
}

def run-rust-script [path rust_binary puzzle input rounds result_file --collect-log] {
  mut results = []
  for _ in 1..$rounds {
    let log = if $collect_log { $input | ^$rust_binary $puzzle $result_file o+e>| collect } else { $input | try { ^$rust_binary $puzzle $result_file }; '' }
    if not ($result_file | path exists) {
      $results ++= [{time: 0ns type: 'error' value: $log log: ''}]
      break
    }
    let result = try-open-nuon $result_file
    if $result.type == 'error' {
      $results ++= [{time: 0ns ...$result log: $log}]
      break
    }
    let time = {second: ($result.value.0.secs? | default 0) nanosecond: ($result.value.0.nanos? | default 0)} | into duration
    $results ++= [{time: $time type: 'success' value: $result.value.1 log: $log}]
  }
  if ($results | last | get type) == 'error' { return ($results | last) }

  let time = if $rounds == 1 { $results.0.time } else { $'($results.time | math avg) +/- ($results.time | into int | into float | math stddev | into duration)' }
  let unique_values = $results.value | uniq
  if ($unique_values | length) > 1 { return {time: $time type: 'inconsistent' value: $unique_values log: ($results.log | str join "\n")} }
  {time: $time type: 'success' value: $unique_values.0 log: $results.0.log}
}

def try-open-nuon [$file] {
  try {
    open --raw $file | from nuon | {type: 'success' value: $in}
  } catch {
    if $in.msg != 'error when loading nuon text' {
      get rendered
    } else {
      get rendered | split row "\n\n" | last | str replace --regex '×.*' $"×(ansi reset) '($file)' is not a valid nuon file"
    } | {type: 'error' value: $in}
  }
}

def format-result [result] {
  if $result.type == 'error' { return $' an error(if $result.value != '' { $":\n($result.value | str trim)" })' }
  let pretty = if ($result.value | describe --detailed | get type) in ['table' 'record' 'list'] { $result.value | table --expand --expand-deep 5 | str trim } else { $result.value | to nuon | nu-highlight }
  $'(if $result.type == "inconsistent" { " inconsistent output" }):(if $pretty =~ "\n" { "\n" } else { " " })($pretty)'
}
