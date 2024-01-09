use std::collections::{HashMap};
use std::net::TcpStream;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use tera::{Context, Tera};
use std::fs;
use chrono::{Utc, Datelike};

pub struct Server {
    routes: HashMap<String, Arc<Mutex<dyn Fn(&str, HashMap<String, String>, &str, HashMap<String, String>) -> Result<String, String> + Send + Sync>>>,
    static_routes: HashMap<String, String>,
}

impl Clone for Server {
    fn clone(&self) -> Self {
        Server {
            routes: self.routes.clone(),
            static_routes: self.static_routes.clone(),
        }
    }
}

impl Server {
    pub fn new() -> Self {
        Server {
            routes: HashMap::new(),
            static_routes: HashMap::new(),
        }
    }

    pub fn route<F>(&mut self, path: &str, func: F)
    where
        F: Fn(&str, HashMap<String, String>, &str, HashMap<String, String>) -> Result<String, String> + 'static + Send + Sync,
    {
        self.routes.insert(path.to_string(), Arc::new(Mutex::new(func)));
    }

    pub fn static_route(&mut self, path: &str, content: &str) {
        self.static_routes.insert(path.to_string(), content.to_string());
    }

    pub fn add_route_to_sitemap(&self, path: &str, lastmod: bool, changefreq: &str, priority: f32, base_url: &str) {
        let current_date = Utc::now().format("%Y-%m-%d").to_string();
        let full_url = format!("{}{}", base_url, path);

        let mut sitemap_content = if let Ok(content) = fs::read_to_string("static/sitemap.xml") {
            content
        } else {
            String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n</urlset>")
        };

        let url_to_check = format!("<loc>{}</loc>", full_url);
        if !sitemap_content.contains(&url_to_check) {
            let mut route_entry = String::new();
            route_entry.push_str("    <url>\n");
            route_entry.push_str(&format!("        <loc>{}</loc>\n", full_url));
            if lastmod {
                route_entry.push_str(&format!("        <lastmod>{}</lastmod>\n", current_date));
            }
            route_entry.push_str(&format!("        <changefreq>{}</changefreq>\n", changefreq));
            route_entry.push_str(&format!("        <priority>{}</priority>\n", priority));
            route_entry.push_str("    </url>\n");

            sitemap_content.insert_str(sitemap_content.rfind("</urlset>").unwrap(), &route_entry);
            if let Err(err) = fs::write("static/sitemap.xml", sitemap_content) {
                eprintln!("Failed to write sitemap: {}", err);
            }
        } else {
            println!("The route {} already exists in the sitemap.", url_to_check);
        }
    }

    pub fn generate_sitemap(&self, sitemap: bool, lastmod: bool, changefreq: &str, priority: f32, base_url: &str) {
        if sitemap {
            let mut sitemap_content = if let Ok(content) = fs::read_to_string("static/sitemap.xml") {
                content
            } else {
                String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n</urlset>")
            };

            let current_date = Utc::now().format("%Y-%m-%d").to_string();

            for (path, _) in &self.static_routes {
                let full_url = format!("{}{}", base_url, path);

                let url_to_check = format!("<loc>{}</loc>", full_url);
                if !sitemap_content.contains(&url_to_check) {
                    let mut route_entry = String::new();
                    route_entry.push_str("    <url>\n");
                    route_entry.push_str(&format!("        <loc>{}</loc>\n", full_url));
                    if lastmod {
                        route_entry.push_str(&format!("        <lastmod>{}</lastmod>\n", current_date));
                    }
                    route_entry.push_str(&format!("        <changefreq>{}</changefreq>\n", changefreq));
                    route_entry.push_str(&format!("        <priority>{}</priority>\n", priority));
                    route_entry.push_str("    </url>\n");

                    sitemap_content.insert_str(sitemap_content.rfind("</urlset>").unwrap(), &route_entry);
                } else {
                    println!("The route {} already exists in the sitemap.", url_to_check);
                }
            }

            if let Err(err) = fs::write("static/sitemap.xml", sitemap_content) {
                eprintln!("Failed to write sitemap: {}", err);
            }
        }
    }

    pub fn load_static_files(&mut self, static_folder: &str) {
        if let Ok(entries) = fs::read_dir(static_folder) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(file_name) = path.file_name() {
                            if let Some(file_str) = file_name.to_str() {
                                let route = format!("/{}", file_str);
                                self.static_route(&route, &path.to_string_lossy());
                            }
                        }
                    }
                }
            }
        }
    }    

    fn serve_static(&self, path: &str) -> Option<String> {
        if let Some(file_path) = self.static_routes.get(path) {
            if let Ok(content) = fs::read_to_string(&file_path) {
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                    content.len(),
                    content
                );
                return Some(response);
            }
        }
        None
    }    

    fn handle_static(&self, path: &str) -> Option<String> {
        if let Some(content) = self.static_routes.get(path) {
            Some(content.clone())
        } else {
            None
        }
    }

    fn path_matches_route(&self, path: &str, route: &str) -> Option<HashMap<String, String>> {
        let path_parts: Vec<&str> = path.split('/').collect();
        let route_parts: Vec<&str> = route.split('/').collect();
    
        if path_parts.len() != route_parts.len() {
            return None;
        }
    
        let mut params = HashMap::new();
    
        for (path_part, route_part) in path_parts.iter().zip(route_parts.iter()) {
            if route_part.starts_with(':') {
                let slug_name = &route_part[1..];
                params.insert(slug_name.to_string(), path_part.to_string());
            } else if path_part != route_part {
                return None;
            }
        }
    
        Some(params)
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
    
        let request = String::from_utf8_lossy(&buffer[..]);
        let request_parts: Vec<&str> = request.split("\r\n\r\n").collect();
    
        let path_params: Vec<&str> = request_parts[0].split_whitespace().collect();
        let path = path_params[1].split('?').next().unwrap_or("");
    
        if let Some(content) = self.serve_static(path) {
            stream.write_all(content.as_bytes()).unwrap();
            stream.flush().unwrap();
            return;
        }
    
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
    
        let response = match self.static_routes.get(path) {
            Some(content) => Ok(format!("{}", content)),
            None => match self.routes.iter().find_map(|(route, handler)| {
                if let Some(params) = self.path_matches_route(path, route) {
                    let handler = handler.lock().unwrap();
                    let cloned_params = params.clone();
                    let cloned_data = data.clone();
                    return Some(handler(path, cloned_params, method, cloned_data).map(|res| {
                        res
                    }));
                }
                None
            }) {
                Some(result) => result,
                None => Err("HTTP/1.1 404 NOT FOUND\n\nPath Not Found".to_string()),
            },
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
    

    pub fn run(mut self, ip: &str, port: &str) {
        self.load_static_files("static");
        
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
