# Welcome to Rocal

## What's Rocal?

Rocal is a **local-first-web-development** a.k.a LWD driven framework that includes everything you need to make an web application by LWD such as an embedded database, router, backup server etc.

Local-first web application is intended to be fully run on a web browser by itself without any server which means it can work independently even when it's offline once an application is served.

That being said, you would like to back up your local data on a remote server just in case, right? 
Rocal has sync server mechanism to synchronize your local data that is stored in your browser with a remote server whatever you want to use (but that needs to satisfy some requirements) in an easy and quick way.

Rocal adopts MVC(Model-View-Controller) architecture, so if you are not familiarized with the architecture, we highly recommend learning the architecture first before using Rocal. That's the essential part to make your application with Rocal effectively.

## Getting Started

### Requirements
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) which is used to build an Rocal application
- (Optional) [miniserve](https://github.com/svenstaro/miniserve) which is used to serve an Rocal application
- (Optional) [brotli](https://github.com/google/brotli) to be used compressing releasing files to publish. See Section 6.

1. Install Rocal by the command below if you haven't yet:

```bash
$ cargo install rocal --features="cli"
```

2. Create a new Rocal application:

```bash
$ rocal new -n myapp
```

where `myapp` is the application name

3. Change directory to `myapp` and build the application:

```bash
$ cd myapp
$ rocal build
```

4. See the generated directories and files:

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

let result: Result<Vec<User>, JsValue> = database.query("select id, first_name, last_name from users;").await;

database.exec("insert users (first_name, last_name) into ('John', 'Smith');").await;
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


5. (Optional) Run `miniserver` to access the application:

```bash
$ miniserve . --header "Cross-Origin-Opener-Policy: same-origin" --header "Cross-Origin-Embedder-Policy: require-corp"
```

Go to `http://127.0.0.1:8080/index.html` and you'll see the welcome message!

6. (Optional) Publish a Rocal application:

```bash
$ cd myapp
$ rocal publish
```

where `myapp` is the application name

Then you can find `release/` and `release.tar.gz` to publish to your hosting server.


## License

Rocal is released under the [MIT License](https://opensource.org/licenses/MIT).
