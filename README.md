
# djed_rsx

A compiler plugin for using RSX (JSX-like syntax) as advanced templating and metaprogramming in Rust.

Made possible by the [Self Tokenize](https://github.com/victorporof/rsx) library, a trait derive for transferring data structures outside of procedural macros from compile-time to run-time.


## Purpose

This compiler plugin allows you to freely intertwine JSX-like syntax anywhere into your Rust code.

djed_rsx implements all of the [JSX](http://facebook.github.io/jsx) grammar. The [purpose and benefits](https://reactjs.co/2015/08/04/advantages-of-jsx/) of JSX and RSX are equivalent.

## How to use
To get access to the `rsx!`, `css!` macros, add this to your `Cargo.toml` file:

```toml
[dependencies]
djed_rsx = { git = "https://github.com/djedou/djed_rsx.git" }
```

Then, simply import the library into your code and use the `rsx!`, `css!` macros to parse RSX and CSS into `rsx_dom::DOMNode`, or `rsx_dom::Stylesheet` data structures respectively.

For example:

```rust
extern crate djed_rsx;

use djed_rsx::{rsx, css};


let stylesheet: Stylesheet = css! { .foo { padding: 1px; } };
let node: DOMNode = rsx! { <div>Hello world!</div> };
```

Here's some code rendering the first example from [Facebook's YOGA](https://facebook.github.io/yoga/) library:

```rust
let stylesheet: Stylesheet = css! {
  .root {
    width: 500px;
    height: 120px;
    flex-direction: row;
    padding: 20px;
  }
  .image {
    width: 80px;
    margin-right: 20px;
  }
  .text {
    height: 25px;
    align-self: center;
    flex-grow: 1;
  }
};

let node: DOMNode = rsx! {
  <view style={stylesheet.take(".root")}>
    <image style={stylesheet.take(".image")} src="..." />
    <text style={stylesheet.take(".text")}>
      Hello world!
    </text>
  </view>
};
```

### Composability

- Mixing Rust and RSX is possible
- Stylesheets can be included as separate CSS files.
- Composing components is achieved through simple function calls (for now).

#### example.css
```css
.root {
  width: 500px;
  height: 120px;
  flex-direction: row;
  padding: 20px;
}
.image {
  width: 80px;
  margin-right: 20px;
}
.text {
  height: 25px;
  align-self: center;
  flex-grow: 1;
}
```

#### example.rs
```rust
fn greeting_str(name: &str) -> String {
  format!("Hello {}!", name)
}

fn render_greeting(name: &str) -> DOMNode {
  let stylesheet = css!("example.css");

  rsx! {
    <text style={stylesheet.take(".text")}>
      { greeting_str(name) }
    </text>
  }
}

fn render_children(name: Option<&str>, image: DOMNode) -> DOMNode {
  rsx! {
    <view>
      { image }
      {
        match name {
          Some(ref n) => render_greeting(n),
          None => <text>No greetings!</text>
        }
      }
    </view>
  }
}

fn render_root() -> DOMNode {
  let stylesheet = css!("example.css");

  rsx! {
    <view style={stylesheet.take(".root")}>
      {
        let name = Some("world");
        let image = <image style={stylesheet.take(".image")} src="..." />;
        render_children(name, image)
      }
    </view>
  }
}

let node = render_root();
```

```rust
let styles: rsx_stylesheet::Stylesheet = css! { ... }
```

The `rsx!` macro returns a `rsx_dom::DOMNode` instance (coming from the [RSX DOM library](https://github.com/victorporof/rsx-dom)). The convertion is automatic between `rsx_parser::RSXElement` abstract syntax trees to the more convenient `rsx_dom::DOMNode` elements, because the AST is directly tokenized into a DOM tree to avoid any runtime work! Templating is thus a zero cost abstraction.

```rust
let node: rsx_dom::DOMNode = rsx! { ... }
```
