def main [] {}

def "main first" [] { lines | each { ($in | str length) - ($in | from yml | str length -g) } | math sum }

def "main second" [] { lines | each { ($in | to json | str length) - ($in | str length) } | math sum }
