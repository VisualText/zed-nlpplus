(separator) @punctuation.special

(pass_header) @title

(tree_header) @title

((name) @type
  (#match? @type "^_"))

((name) @variable
  (#not-match? @variable "^_"))

(type) @type.builtin

(flag) @attribute

(variable
  name: (string) @property)

(variable
  value: (string) @string)

(number) @number

[
  "["
  "]"
  "("
  ")"
] @punctuation.bracket

"," @punctuation.delimiter
