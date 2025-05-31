def main [] {}

def "main first" [] {
  let secret = $in
  1.. | where ($'($secret)($it)' | hash md5) =~ '^0{5}' | first
}

def "main second" [] {
  let secret = $in
  1.. | where ($'($secret)($it)' | hash md5) =~ '^0{6}' | first
}
