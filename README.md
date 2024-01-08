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

## Routing with Slugs

To handle different routes including slugs, use the `route()` method. You can define routes with slugs and their corresponding handler functions.

For instance, to handle a route with a slug:

```rust
server.route("/articles/:slug", |path, params, method, data| {
    if method == "GET" {
        let slug = params.get("slug").map_or_else(|| "".to_string(), |x| x.to_string());
        Ok(format!("HTTP/1.1 200 OK\n\nArticle content for slug: {}", slug))
    } else {
        Err("HTTP/1.1 405 METHOD NOT ALLOWED\n\nMethod Not Allowed".to_string())
    }
});
```

```rust
server.route("/articles", |path, params, method, data| {
    if method == "GET" {
        Ok(format!("HTTP/1.1 200 OK\n\nArticle content : Article1, ...."))
    } else {
        Err("HTTP/1.1 405 METHOD NOT ALLOWED\n\nMethod Not Allowed".to_string())
    }
});
```

## Handling Different HTTP Request Types

### GET Request

To handle a GET request, you can use slugs within the route:

```rust
server.route("/get/:param", |_, params, method, _| {
    if method == "GET" {
        let v = params.get("param").map_or_else(|| "".to_string(), |x| x.to_string());
        Ok(format!("HTTP/1.1 200 OK\n\n{}", v))
    } else {
        Err("HTTP/1.1 405 METHOD NOT ALLOWED\n\nMethod Not Allowed".to_string())
    }
});
```

### POST Request

To handle a POST request, slugs can be used similarly:

```rust
server.route("/post/:data", |_, _, method, data| {
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

An example showcasing the use of slugs in routing:

```rust
use sandy::Server;
use sandy::TemplateEngine;
use std::collections::HashMap;

fn main() {
    let mut server = Server::new();

    server.route("/", |_, _, _, _| {
        Ok("HTTP/1.1 200 OK\n\nHello, sandy".to_string())
    });

    server.route("/get/:param", |_, params, method, _| {
        if method == "GET" {
            let v = params.get("param").map_or_else(|| "".to_string(), |x| x.to_string());
            Ok(format!("HTTP/1.1 200 OK\n\n{}", v))
        } else {
            Err("HTTP/1.1 405 METHOD NOT ALLOWED\n\nMethod Not Allowed".to_string())
        }
    });

    server.route("/post/:data", |_, _, method, data| {
        if method == "POST" {
            let v = data.get("data").map_or_else(|| "".to_string(), |x| x.to_string());
            Ok(format!("HTTP/1.1 200 OK\n\n{}", v))
        } else {
            Err("HTTP/1.1 405 METHOD NOT ALLOWED\n\nMethod Not Allowed".to_string())
        }
    });
    
    server.route("/render_template/:var", |_, _, _, _| {
        let context: HashMap<_, _> = [("var", "value")].iter().cloned().collect();
        match TemplateEngine::render_template("template.html", &context) {
            Ok(rendered) => Ok(format!("HTTP/1.1 200 OK\n\n{}", rendered)),
            Err(err) => Err(format!("HTTP/1.1 500 Internal Server Error\n\n{}", err)),
        }
    });

    server.route("/render/:var", |_, _, _, _| {
        let context: HashMap<_, _> = [("var", "value")].iter().cloned().collect();
        let template = "This is a {{ var }} template.";
        let rendered = TemplateEngine::render(template, &context);
        Ok(format!("HTTP/1.1 200 OK\n\n{}", rendered))
    });

    server.run("0.0.0.0", "8080");
}
```
