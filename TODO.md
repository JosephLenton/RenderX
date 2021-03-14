# TODO

# To reach Alpha ...

  * Components
    * Component functions
    * Props

  * Client Side Updates
    * Spike; create a crate that replicates the SPA update idea, and does a performance check.
      * https://github.com/krausest/js-framework-benchmark
      * https://krausest.github.io/js-framework-benchmark/2020/table_chrome_87.0.4280.66.html

  * State Management
  * Be able to use `classnames`. i.e. `class={base_class.el("form")}`
  - Using code as a child. i.e. the `{nodes}` in `<div>{nodes}</div>`

  * Attributes
    * Remove being able to use code for keys, as it doesn't work with Props to components.
    * Add support for any identifier key names. i.e. Props { r"my-key": 35 }
    - Values, i.e. the `value` in `<div key="value" />`
    - Add support for non-strings. i.e. `<input type="text" min={5} max={10} />`
    - Code for attributes, i.e. `<div key={some_variable} />`
    - Attribute keys with hyphens and underscores in them. i.e. `<div data-name="div">`
    - Using a literal for an attribute key. i.e. `<div "data-name"="blah" />`. This is for full DOCTYPE support.
    - Using code as the name for an attribute key. i.e. `<div {"disabled"} />`

  * Nodes
    - Namespaces; this is names with the form `a:b:c`. i.e. the `blah:foo` in `<blah:foo />`
    - Being able to use code for a node name. i.e. `let el = &"div"; <{el} />`

  * Structure
    * Replacate the grammar tests for render integration tests.
    * Some complex tests.
    * Move render into it's own crate.
    * Should content be escaped???
      * Children of a node???
      * Attribute values???

  * Output - Optimise what is laid out.
    * Allow overriding the path to the node (the `::renderx::dom::Node` stuff), so another library can change it.
      * This can be done by allowing the path to be set via an environmental setting in Cargo, and optionally picked up in `output.rs`.
    * Attributes use a static list of key=value where possible.
    * Nodes use a static list of nodes where possible.
    * The whole _'Node & Child'_ thing needs to be rethought.
