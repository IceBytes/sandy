# sandy

This library provides a simple HTTP server and a template rendering engine in Rust.

## Server Usage

You can create a new HTTP server using the `Server::new()` function.

```rust
use sandy::Server;

let mut server = Server::new();
```

## Server run

You can run the HTTP server using the `Server.run()` function.

```rust
let ip = "127.0.0.1"
let port = "8080"
server.run(ip, port)
```

## Routing

To handle different routes, use the `route()` method. You can define routes and their corresponding handler functions.

```rust
server.route("/", |path, params, method, data| {
    Ok("HTTP/1.1 200 OK\n\nHello, sandy".to_string())
});
```

## Handling Different HTTP Request Types

### GET Request

To handle a GET request, use the following:

```rust
server.route("/get", |_, params, method, _| {
    if method == "GET" {
        let v = params.get("param").map_or_else(|| "".to_string(), |x| x.to_string());
        Ok(format!("HTTP/1.1 200 OK\n\n{}", v))
        } else {
        Err("HTTP/1.1 405 METHOD NOT ALLOWED\n\nMethod Not Allowed".to_string())
    }
});
```

### POST Request

To handle a POST request, use the following:

```rust
server.route("/post", |_, _, method, data| {
    if method == "POST" {
        let v = data.get("data").map_or_else(|| "".to_string(), |x| x.to_string());
        Ok(format!("HTTP/1.1 200 OK\n\n{}", v))
    } else {
        Err("HTTP/1.1 405 METHOD NOT ALLOWED\n\nMethod Not Allowed".to_string())
    }
});
```

## Template Engine Usage

You can render templates using the `TemplateEngine::render_template()` function.

```rust
use sandy::TemplateEngine;
use std::collections::HashMap;

let mut context = HashMap::new();
context.insert("name", "Just Ice");

let rendered = TemplateEngine::render_template("template.html", &context);
```

The `TemplateEngine::render()` function renders templates from strings using a context.

```rust
let template = "<p>Hello, {{ name }}!</p>";
let rendered = TemplateEngine::render(template, &context);
```

## Response

All route handler functions should return a `Result<String, String>` where `Ok` represents a successful response and `Err` represents an error message.

For example:

```rust
Ok("HTTP/1.1 200 OK\n\nHello, sandy".to_string())
```

```rust
Err("HTTP/1.1 404 OK\n\nPath Not Found".to_string())
```

```rust
Err("HTTP/1.1 405 METHOD NOT ALLOWED\n\nMethod Not Allowed".to_string())
```

## Full Example

example of use all features in the library

```rust
use sandy::Server;
use sandy::TemplateEngine;
use std::collections::HashMap;

fn main() {
    let mut server = Server::new();

    server.route("/", |_, _, _, _| {
        Ok("HTTP/1.1 200 OK\n\nHello, sandy
".to_string())
    });

    server.route("/get", |_, params, method, _| {
        if method == "GET" {
            let v = params.get("param").map_or_else(|| "".to_string(), |x| x.to_string());
            Ok(format!("HTTP/1.1 200 OK\n\n{}", v))
        } else {
            Err("HTTP/1.1 405 METHOD NOT ALLOWED\n\nMethod Not Allowed".to_string())
        }
    });

    server.route("/post", |_, _, method, data| {
        if method == "POST" {
            let v = data.get("data").map_or_else(|| "".to_string(), |x| x.to_string());
            Ok(format!("HTTP/1.1 200 OK\n\n{}", v))
        } else {
            Err("HTTP/1.1 405 METHOD NOT ALLOWED\n\nMethod Not Allowed".to_string())
        }
    });
    
    server.route("/render_template", |_, _, _, _| {
        let context: HashMap<_, _> = [("var", "value")].iter().cloned().collect();
        match TemplateEngine::render_template("template.html", &context) {
            Ok(rendered) => Ok(format!("HTTP/1.1 200 OK\n\n{}", rendered)),
            Err(err) => Err(format!("HTTP/1.1 500 Internal Server Error\n\n{}", err)),
        }
    });

    server.route("/render", |_, _, _, _| {
        let context: HashMap<_, _> = [("var", "value")].iter().cloned().collect();
        let template = "This is a {{ var }} template.";
        let rendered = TemplateEngine::render(template, &context);
        Ok(format!("HTTP/1.1 200 OK\n\n{}", rendered))
    });

    // if you wanna add CSS files to your project, just include those files in your code like you would any other route
    
    server.run("0.0.0.0", "8080");
}
```
"# sandy" 
"# sandy" 

