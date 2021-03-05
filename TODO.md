# TODO

# To reach Alpha ...

  * Components

  * Using code as a child. i.e. the `{nodes}` in `<div>{nodes}</div>`

  * Attributes
    - Values, i.e. the `value` in `<div key="value" />`
    - Add support for non-strings. i.e. `<input type="text" min={5} max={10} />`
    - Code for attributes, i.e. `<div key={some_variable} />`
    - Attribute keys with hyphens and underscores in them. i.e. `<div data-name="div">`
    * Using a literal for an attribute key. i.e. `<div "data-name"="blah" />`. This is for full DOCTYPE support.

  * Nodes
    * Namespaces; this is names with the form `a:b:c`. i.e. the `blah:foo` in `<blah:foo />`
    * Being able to use code for a node name. i.e. `let el = &"div"; <{el} />`

  * Structure
    * Replacate the grammar tests for render integration tests.
    * Some complex tests.
    * Move render into it's own crate.
    * Should content be escaped???
      * Children of a node???
      * Attribute values???

  * Output - Optimise what is laid out.
    * Attributes use a static list of key=value where possible.
    * Nodes use a static list of nodes where possible.
    * The whole _'Node & Child'_ thing needs to be rethought.
