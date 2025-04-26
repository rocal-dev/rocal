# Rocal UI - A simple template engine with Rust

![logo](https://dev-to-uploads.s3.amazonaws.com/uploads/articles/a2eofyw92dwrvbuorvik.png)

Although this template engine is basically intended to use with Rocal framework to craft views, it can be used anywhere with Rust.

Let's begin with syntax of Rocal UI. Here is a simple example including variables, if-else control, and for-loop control.

```rust ,ignore
view! {
  <div class="container">
    <h1 class="title">{{ title }}</h1>

    if user.id <= 10 {
      <p>{ "You are an early user!" }</p>
      <a href={{ reward_url }}>{ "Click here to get rewards!" }</a>
    } else if user.id <= 20 {
      <p>{ "You are kind of an early user." }</p>
      <a href={{ sort_of_reward_url }}>{ "Check it out for your reward." }</a>
    } else {
      <p>{ "You are a regular user." }</p>
    }

    <hr/>
    
    <ul>
      for article in articles {
        <li><a href={{ article.url }}>{{ article.title }}</a></li>
      }
    </ul>
  </div>
}
```

It's straight forward, isn't it?

- `{{ variable }}` : as you saw the code above, you can use any variable with it
- `if-else` : you can utilize `if-else` even `else-if` as below
```rust ,ignore
if user.id <= 10 {
    <p>{ "You are an early user!" }</p>
    <a href={{ reward_url }}>{ "Click here to get rewards!" }</a>
} else if user.id <= 20 {
   <p>{ "You are kind of an early user." }</p>
   <a href={{ sort_of_reward_url }}>{ "Check it out for your reward." }</a>
} else {
  <p>{ "You are a regular user." }</p>
}
```
- `for-in`: This can be used as same as Rust syntax
```rust,ignore
for article in articles {
  <li><a href={{ article.url }}>{{ article.title }}</a></li>
}
```
- `{ "string" }`: This is sort of shorthand of variable embedding for only string. You can use it with regular string, exactly, `&str` in Rust context

## Advanced use
`view! {}` produces HTML string technically, so you can embed view! in another view! like using it as a partial template.

```rust ,ignore
let button = view! {
  <button type="submit" class="btn btn-primary">
    Submit
  </button>
};

view! {
  <form action={{ &format!("/articles/{}", article.id) }}>
    <input type="text" />
    {{ &button }}
  </form>
}
```

On top of that, so `{{ variable }}` can take any expression that emits `&str` of Rust, if you want to do string interpolation, you can write like `{{ &format!("Hi, {}", name) }}`.

Links:
- [GitHub](https://github.com/rocal-dev/rocal) I'd appreciate it if you could star it.
- [Download](https://crates.io/crates/rocal-ui)
