def main [] {}

def "main first" [] { sum-invalid-ids '^(.+)\1$' }

def "main second" [] { sum-invalid-ids '^(.+)\1+$' }

def sum-invalid-ids [pattern] { split row , | parse '{start}-{end}' | into int start end | each { $in.start..$in.end | where ($it | into string) =~ $pattern | append 0 | math sum } | math sum }
