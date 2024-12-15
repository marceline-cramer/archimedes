(comment) @comment.line
(integer) @constant.numeric
(variable) @variable
(pattern (symbol) @constant)

(rule head: (atom relation: (symbol) @function))
(rule_body (atom relation: (symbol) @constant))

[ "," "." ] @punctuation.delimiter
[ "(" ")" ] @punctuation.bracket

[
  "constrain"
  "decide"
  "if"
  "cardinality"
] @keyword

(constraint_kind) @keyword.control
