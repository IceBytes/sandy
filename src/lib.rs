use std::collections::{HashMap};
use std::net::TcpStream;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use regex::Regex;
use std::fs;
use tera::{Context, Tera};

pub struct Server {
    routes: HashMap<String, Arc<Mutex<dyn Fn(&str, HashMap<String, String>, &str, HashMap<String, String>) -> Result<String, String> + Send + Sync>>>,
}

impl Clone for Server {
    fn clone(&self) -> Self {
        Server {
            routes: self.routes.clone(),
        }
    }
}

impl Server {
    pub fn new() -> Self {
        Server {
            routes: HashMap::new(),
        }
    }

    pub fn route<F>(&mut self, path: &str, func: F)
    where
        F: Fn(&str, HashMap<String, String>, &str, HashMap<String, String>) -> Result<String, String> + 'static + Send + Sync,
    {
        self.routes.insert(path.to_string(), Arc::new(Mutex::new(func)));
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        let request = String::from_utf8_lossy(&buffer[..]);
        let request_parts: Vec<&str> = request.split("\r\n\r\n").collect();

        let path_params: Vec<&str> = request_parts[0].split_whitespace().collect();
        let path = path_params[1].split('?').next().unwrap_or("");

        let method = path_params[0];

        let mut params = HashMap::new();
        let mut data = HashMap::new();
        if let Some(query_params) = path_params[1].splitn(2, '?').nth(1) {
            let payload_parts: Vec<&str> = query_params.split("&").collect();
            for part in payload_parts {
                let kv: Vec<&str> = part.split("=").collect();
                if kv.len() == 2 {
                    params.insert(kv[0].to_string(), kv[1].to_string());
                }
            }
        }

        let body_params: Vec<&str> = request_parts[1].split('&').collect();
        for part in body_params {
            let kv: Vec<&str> = part.split("=").collect();
            if kv.len() == 2 {
                data.insert(kv[0].to_string(), kv[1].to_string());
            }
        }

        let response = match self.routes.get(path) {
            Some(handler) => {
                let handler = handler.lock().unwrap();
                handler(path, params, method, data)
            }
            None => Err("HTTP/1.1 404 NOT FOUND\n\nPath Not Found".to_string()),
        };

        match response {
            Ok(result) => {
                stream.write_all(result.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
            Err(err) => {
                stream.write_all(err.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
        }
    }

    pub fn run(self, ip: &str, port: &str) {
        let listener = std::net::TcpListener::bind(format!("{}:{}", ip, port)).unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let server = self.clone();

            thread::spawn(move || {
                server.handle_connection(stream);
            });
        }
    }
}

pub struct TemplateEngine;

impl TemplateEngine {
    pub fn render(template: &str, context: &HashMap<&str, &str>) -> String {
        let mut tera = Tera::default();
        tera.add_raw_template("template", template).unwrap();

        let mut ctx = Context::new();
        for (key, val) in context {
            ctx.insert(*key, val);
        }

        tera.render("template", &ctx).unwrap()
    }

    pub fn render_template(template_name: &str, context: &HashMap<&str, &str>) -> Result<String, String> {
        let file_content = match std::fs::read_to_string(format!("templates/{}", template_name)) {
            Ok(content) => content,
            Err(_) => return Err("Template file not found".to_string()),
        };

        let mut tera = Tera::default();
        tera.add_raw_template("template", &file_content).unwrap();

        let mut ctx = Context::new();
        for (key, val) in context {
            ctx.insert(*key, val);
        }

        tera.render("template", &ctx).map_err(|e| e.to_string())
    }
}
