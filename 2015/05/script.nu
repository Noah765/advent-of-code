def main [] {}

def "main first" [] { lines | where $it =~ '([aeiou].*){3}' and $it =~ '(.)\1' and $it !~ 'ab|cd|pq|xy' | length }

def "main second" [] { lines | where $it =~ '(..).*\1' and $it =~ '(.).\1' | length }
