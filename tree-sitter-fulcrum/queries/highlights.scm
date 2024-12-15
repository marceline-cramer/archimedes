(comment) @comment.line
(integer) @constant.numeric
(variable) @variable
(pattern (symbol) @constant)
(import (symbol) @module)
(uniform) @keyword

(rule head: (atom relation: (symbol) @function))
(rule (_) (atom relation: (symbol) @constant))
(constraint (_) (atom relation: (symbol) @constant))

[ "," "." ] @punctuation.delimiter
[ "(" ")" ] @punctuation.bracket

[
  "constrain"
  "decide"
  "if"
  "cardinality"
  "soft"
  "import"
] @keyword

(constraint_kind) @keyword.control
