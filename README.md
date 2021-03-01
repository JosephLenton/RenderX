# RenderX

## Motivations

There are a long list of implementations of rsx for Rust. Most are fantastic work, but are quite limiting. Before writing RenderX, the best of them, by far, was [Render](https://crates.io/crates/render). It is an awesome library. However even that has limitations.

In practice on any real code base there are a number of patterns that take up about 10% of your code. Things that you can do in React with JSX, but you can't do in Render.

This projects aims to do everything Render can do, plus more.

Here are a list of things you can do in RenderX, which you cannot do in Render-rs:

# ^ CHECK THE ABOVE IS 100% TRUE!
#
#

### Support for `<!--` tags

```
#[Component]
pub fn Home() {
  rsx! {
    <!DOCTYPE html>
    <!--
      This is a comment within my page.
    -->
  }
}
```

### Write text inline

Both of the below work ...

```
#[Component]
pub fn Button() {
  rsx! {
    <button>
      Click Me
    </button>
  }
}
```

```
#[Component]
pub fn Button() {
  rsx! {
    <button>
      "Click Me"
    </button>
  }
}
```

Bear in mind this only works for simple text. You have no control over the spacing between items. However for my needs, that's fine.

### Using a variable name for a tag

Often you want to use a tag based on some condition. In this example the component is rendered with `a` if a href is provided, and `button` if it is not.

```
#[Component]
pub fn Button(maybe_href:Option<&'static str>) {
  let el = if maybe_href.is_some() {
    "a"
  } else {
    "button"
  };

  rsx! {
    <{el} href={maybe_href}>
      Click Me
    </{el}>
  }
}
```

### Using an `if` statement to return *different* nodes from a Component

```
#[Component]
pub fn Button(maybe_href:Option<&'static str>) {
  if let Some(href) = maybe_href {
    rsx! {
      <button>
        Click Me
      </button>
    }
  } else {
    rsx! {
      <a href={href}>
        Click Me
      </a>
    }
  }
}
```

### Using `Option` values on attribute values to decide if to print them or not

Here the `href` is only set if `maybe_href` is not `None`. If it is `None`, then nothing happens.

```
#[Component]
pub fn Button(maybe_href:Option<&'static str>) {
  rsx! {
    <a href={maybe_href}>
      Click Me
    </a>
  }
}
```

### Comments are supported
