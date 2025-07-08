use may_minihttp::{HttpService, HttpServiceFactory, Request, Response};
use std::io;
use std::thread;
use std::time::Duration;
use clap::{Arg, Command};

#[derive(Clone)]
struct TestDummyService;

impl HttpService for TestDummyService {
    fn call(&mut self, req: Request, resp: &mut Response) -> io::Result<()> {
        let path = req.path();
        let method = req.method();

        // Fastest possible root route
        if method == "GET" && path == "/" {
            resp.status_code(200, "OK");
            resp.header("Content-Type: text/plain");
            resp.body("OK"); // static, no allocations
            return Ok(());
        }

        // /get — echo request info as JSON
        if method == "GET" && path == "/get" {
            resp.status_code(200, "OK");
            resp.header("Content-Type: application/json");
            let json = format!(
                r#"{{"method":"{}","path":"{}"}}"#,
                method, path
            );
            resp.body(Box::leak(json.into_boxed_str()));
            return Ok(());
        }

        // /status/:code — return given status code
        if let Some(code) = path.strip_prefix("/status/") {
            if let Ok(code) = code.parse::<u16>() {
                let reason = match code {
                    200 => "OK",
                    201 => "Created",
                    204 => "No Content",
                    400 => "Bad Request",
                    401 => "Unauthorized",
                    404 => "Not Found",
                    500 => "Internal Server Error",
                    _ => "Custom",
                };
                resp.status_code(code as usize, reason);
                resp.header("Content-Type: text/plain");
                resp.body("");
                return Ok(());
            }
        }

        // /delay/:seconds — wait N seconds, then return
        if let Some(secs) = path.strip_prefix("/delay/") {
            if let Ok(secs) = secs.parse::<u64>() {
                thread::sleep(Duration::from_secs(secs.min(10))); // max 10s
                resp.status_code(200, "OK");
                resp.header("Content-Type: text/plain");
                resp.body("delayed");
                return Ok(());
            }
        }

        // /headers — return request headers as JSON
        if method == "GET" && path == "/headers" {
            resp.status_code(200, "OK");
            resp.header("Content-Type: application/json");
            let mut json = String::from("{\"headers\":{");
            let mut first = true;
            for header in req.headers().iter() {
                if !first { json.push(','); } first = false;
                let k = header.name;
                let v = std::str::from_utf8(header.value).unwrap_or("?");
                json.push_str(&format!("\"{}\":\"{}\"", k, v));
            }
            json.push_str("}}\n");
            resp.body(Box::leak(json.into_boxed_str()));
            return Ok(());
        }

        // /ip — return a fake client IP
        if method == "GET" && path == "/ip" {
            resp.status_code(200, "OK");
            resp.header("Content-Type: application/json");
            resp.body("{\"origin\":\"127.0.0.1\"}");
            return Ok(());
        }

        // /uuid — return a random UUID
        if method == "GET" && path == "/uuid" {
            use uuid::Uuid;
            let uuid = Uuid::new_v4().to_string();
            let json = format!("{{\"uuid\":\"{}\"}}", uuid);
            resp.status_code(200, "OK");
            resp.header("Content-Type: application/json");
            resp.body(Box::leak(json.into_boxed_str()));
            return Ok(());
        }

        // /anything — echo method, path, headers, and body
        if path == "/anything" {
            let mut json = format!(
                "{{\"method\":\"{}\",\"path\":\"{}\",\"headers\":{{",
                method, path
            );
            let mut first = true;
            for header in req.headers().iter() {
                if !first { json.push(','); } first = false;
                let k = header.name;
                let v = std::str::from_utf8(header.value).unwrap_or("?");
                json.push_str(&format!("\"{}\":\"{}\"", k, v));
            }
            json.push_str("},\"body\":");
            let mut body = Vec::new();
            let mut reader = req.body();
            use std::io::Read;
            let _ = reader.read_to_end(&mut body);
            if !body.is_empty() {
                let body_str = String::from_utf8_lossy(&body);
                json.push('"');
                json.push_str(&body_str);
                json.push('"');
            } else {
                json.push_str("null");
            }
            json.push_str("}\n");
            resp.status_code(200, "OK");
            resp.header("Content-Type: application/json");
            resp.body(Box::leak(json.into_boxed_str()));
            return Ok(());
        }

        // Default: 404
        resp.status_code(404, "Not Found");
        resp.header("Content-Type: text/plain");
        resp.body("Not found");
        Ok(())
    }
}

struct TestDummyFactory;

impl HttpServiceFactory for TestDummyFactory {
    type Service = TestDummyService;
    fn new_service(&self, _: usize) -> Self::Service {
        TestDummyService
    }
}

fn build_cli() -> Command {
    Command::new("test-dummy")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Ultra-fast httpbin-style test server for load testing")
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .help("Port to listen on")
                .default_value("8080")
                .value_parser(clap::value_parser!(u16)),
        )
        .arg(
            Arg::new("host")
                .short('i')
                .long("host")
                .help("Host/IP to bind to")
                .default_value("127.0.0.1")
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("https")
                .long("https")
                .help("Enable HTTPS (requires certificate files)")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("cert")
                .long("cert")
                .help("Path to SSL certificate file (PEM format)")
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("key")
                .long("key")
                .help("Path to SSL private key file (PEM format)")
                .value_parser(clap::value_parser!(String)),
        )
}

fn main() {
    let args = build_cli().get_matches();
    
    let port = *args.get_one::<u16>("port").unwrap_or(&8080);
    let host = args.get_one::<String>("host").unwrap_or(&"127.0.0.1".to_string()).clone();
    let https = args.get_flag("https");
    let cert_path = args.get_one::<String>("cert").map(|s| s.clone());
    let key_path = args.get_one::<String>("key").map(|s| s.clone());
    
    let addr = format!("{}:{}", host, port);
    let protocol = if https { "https" } else { "http" };
    
    println!("🐝 Test Dummy (httpbin-style) Server Starting...");
    println!("Fastest / route, and classic httpbin endpoints available!");
    
    if https {
        if let (Some(cert), Some(_key)) = (cert_path, key_path) {
            println!("🔒 Starting HTTPS server with certificate: {}", cert);
            println!("✅ Listening on {}://{}", protocol, addr);
            // Note: may_minihttp doesn't support HTTPS directly
            // This would require a different HTTP server implementation
            eprintln!("⚠️  HTTPS support not implemented in may_minihttp");
            eprintln!("   Use --cert and --key with a different HTTP server");
            std::process::exit(1);
        } else {
            eprintln!("❌ HTTPS requires both --cert and --key arguments");
            std::process::exit(1);
        }
    } else {
        println!("✅ Listening on {}://{}", protocol, addr);
        let server = TestDummyFactory.start(&addr).unwrap();
        server.wait();
    }
} 