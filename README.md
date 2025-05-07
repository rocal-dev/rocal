# Welcome to Rocal

## What's Rocal?

Rocal is Full-Stack WASM framework that can be used to build fast and robust web apps thanks to high performance of WebAssembly and Rust's typing system and smart memory management.

Rocal adopts MVC(Model-View-Controller) architecture, so if you are not familiarized with the architecture, we highly recommend learning the architecture first before using Rocal. That's the essential part to make your application with Rocal effectively.

## Getting Started

```rust
fn run() {
  migrate!("db/migrations");
  
  route! {
    get "/hello-world" => { controller: HelloWorldController, action: index, view: HelloWorldView }
  }
}

// ... in HelloWorldController
impl Controller for HelloWorldController {
  type View = UserView;
}

#[rocal::action]
pub fn index(&self) {
  self.view.index("Hello, World!");
}

// ... in HelloWorldView
pub fn index(&self, message: &str) {
  let template = HelloWorldTemplate::new(self.router.clone());
  template.render(message);
}

// ... in HelloWorldTemplate
fn body(&self, data: Self::Data) -> String {
  view! {
    <h1>{"Welcome to Rocal World!"}</h1>
	
    if data.is_empty() {
      <h2>{"There is no message."}</h2>
    } else {	
      <h2>{{ data }}</h2>
    }
   	
    <form action="/posts">
      <input type="text" />
      {{ &button("submit", "btn btn-primary", "Submit") }}
    </form>
  }
}

fn button(ty: &str, class: &str, label: &str) -> String {
  view! {
    <button type={{ ty }} class={{ class }}>
      {{ label }}
    </button>
  }
}
```
As you can see the quick example, to render HTML with MVC architecture, in this case, the router and each controller, view, and template can be written like that.

### Requirements
1. Install Rocal by the command below if you haven't yet:

On MacOS or Linux

```bash
$ curl -fsSL https://www.rocal.dev/install.sh | sh
```

On Windows
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) which is used to build an Rocal application
- [brotli](https://github.com/google/brotli) to be used compressing releasing files to publish.

```bash
$ cargo install rocal --features="cli"
```

2. Create a new Rocal application:

```bash
$ rocal new -n myapp
```

where `myapp` is the application name

3. Run to access the application:

```bash
$ cd myapp
$ rocal run # you can change a port where the app runs with `-p <Port>`. An app runs on 3000 by default
```

Go to `http://127.0.0.1:3000` and you'll see the welcome message!

4. Build the application without running it:

```bash
$ rocal build
```

5. See the generated directories and files:

Probably, you could find some directories and files in the application directory after executing the leading commands.

Especially, if you want to learn how the application works, you should take a look at lib.rs, controllers, views, and models. 

Some Rocal macros are used to build the application such as `config!` and `#[rocal::main]` which are in `src/lib.rs` and required to run. On top of that, you could see `route!` macro that provides you with an easy way to set up application routing.

Other than the macros, there is an essential struct to communicate with an embedded database which is now we utilize [SQLite WASM](https://sqlite.org/wasm/doc/trunk/index.md).

You could write like below to execute queries to the database.

```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct User {
  id: u32,
  first_name: String,
  last_name: String,
}

let database = crate::CONFIG.get_database().clone();

let result: Result<Vec<User>, JsValue> = database.query("select id, first_name, last_name from users;").fetch().await;

let first_name = "John";
let last_name = "Smith";

database
  .query("insert users (first_name, last_name) into ($1, $2);")
  .bind(first_name)
  .bind(last_name)
  .execute()
  .await;
```

And, to create tables, you are able to put SQL files in `db/migrations` directory.

e.g. db/migrations/202502090330_create_user_table.sql

```sql
create table if not exists users (
  id integer primary key,
  first_name text not null,
  last_name text not null,
  created_at datetime default current_timestamp
);
```

6. (Optional) Publish a Rocal application:
```bash
$ cd myapp
$ rocal publish
```

where `myapp` is the application name

Then you can find `release/` and `release.tar.gz` to publish to your hosting server.


## License

Rocal is released under the [MIT License](https://opensource.org/licenses/MIT).
