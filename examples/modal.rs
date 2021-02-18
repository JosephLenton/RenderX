use ::awesome::render::ccomponent;
use ::awesome::render::rsx;
use ::awesome::render::ServerRender;
use ::classnames::classname;

pub fn main() {
    let render = ServerRender::new();
    modal(render, &"", None);

    println!("{}", render);
}

#[component]
fn modal<'a>(value: &'a str, maybe_error: Option<String>) {
    let base_class = classname("modal");

    rsx!(
      <div class={base_class} aria-label="button">
        <div class={base_class.el("header")}>
          Upgrade Your Account Today!
        </div>

        <select>
          {options.map(|option| {
            <option>{option}</option>
          })}
        </select>

        <input type="text" value={price} />

        (maybe_error.is_some() && <div class="error">
          {maybe_error.unwrap()}
        </div>)

        <footer>
          crate time is self for super
        </footer>
      </div>
    );
}
