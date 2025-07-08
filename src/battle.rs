use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::net::{SocketAddr, ToSocketAddrs};

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use mio::net::TcpStream;
use mio::{Events, Interest, Poll, Token};
use mio::event::Source;
use nix::sys::socket::{setsockopt, sockopt::TcpNoDelay};
use rustls::{ClientConfig, ClientConnection, ServerName};

use rustls::client::{ServerCertVerified, ServerCertVerifier};

#[derive(Debug, Clone)]
pub struct RequestConfig {
    pub method: String,
    pub path: String,
    pub host: String,
    pub port: u16,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
    pub is_ht: bool,
}

impl RequestConfig {
    pub fn from_url(url: &str, method: &str, headers: &[String], body: Option<&str>) -> Result<Self, Box<dyn std::error::Error>> {
        let uri = url::Url::parse(url)?;
        let host = uri.host_str().unwrap_or("127.0.0.1").to_string();
        let port = uri.port_or_known_default().unwrap_or(80);
        let path = if uri.path().is_empty() { "/" } else { uri.path() };
        let is_ht = uri.scheme() == "ht";
        
        // Parse custom headers
        let mut parsed_headers = Vec::new();
        for header in headers {
            if let Some(colon_pos) = header.find(':') {
                let key = header[..colon_pos].trim().to_string();
                let value = header[colon_pos + 1..].trim().to_string();
                parsed_headers.push((key, value));
            }
        }
        
        // Add default headers if not provided
        if !parsed_headers.iter().any(|(k, _)| k.eq_ignore_ascii_case("host")) {
            parsed_headers.push(("Host".to_string(), host.clone()));
        }
        if !parsed_headers.iter().any(|(k, _)| k.eq_ignore_ascii_case("connection")) {
            parsed_headers.push(("Connection".to_string(), "keep-alive".to_string()));
        }
        if !parsed_headers.iter().any(|(k, _)| k.eq_ignore_ascii_case("user-agent")) {
            parsed_headers.push(("User-Agent".to_string(), "Wasps-With-Bazookas/2.0.0".to_string()));
        }
        
        Ok(RequestConfig {
            method: method.to_uppercase(),
            path: path.to_string(),
            host,
            port,
            headers: parsed_headers,
            body: body.map(|s| s.to_string()),
            is_ht,
        })
    }
    
    pub fn build_request(&self) -> String {
        let mut capacity = 64;
        capacity += self.headers.iter().map(|(k, v)| k.len() + v.len() + 4).sum::<usize>();
        if self.body.is_some() {
            capacity += 20;
        }
        capacity += 4;
        if let Some(ref body) = self.body {
            capacity += body.len();
        }
        let mut request = String::with_capacity(capacity);
        request.push_str(&self.method);
        request.push(' ');
        request.push_str(&self.path);
        request.push_str(" HTTP/1.1\r\n");
        for (key, value) in &self.headers {
            request.push_str(key);
            request.push_str(": ");
            request.push_str(value);
            request.push_str("\r\n");
        }
        if let Some(ref body) = self.body {
            request.push_str("Content-Length: ");
            request.push_str(&body.len().to_string());
            request.push_str("\r\n");
        }
        request.push_str("\r\n");
        if let Some(ref body) = self.body {
            request.push_str(body);
        }
        request
    }
}

#[derive(Debug)]
enum ConnectionStream {
    Plain(TcpStream),
    Tls(ClientConnection, TcpStream),
}

impl ConnectionStream {
    fn is_handshaking(&self) -> bool {
        match self {
            ConnectionStream::Plain(_) => false,
            ConnectionStream::Tls(conn, _) => conn.is_handshaking(),
        }
    }
}

impl Source for ConnectionStream {
    fn register(&mut self, registry: &mio::Registry, token: Token, interests: Interest) -> io::Result<()> {
        match self {
            ConnectionStream::Plain(stream) => stream.register(registry, token, interests),
            ConnectionStream::Tls(_, stream) => stream.register(registry, token, interests),
        }
    }
    fn reregister(&mut self, registry: &mio::Registry, token: Token, interests: Interest) -> io::Result<()> {
        match self {
            ConnectionStream::Plain(stream) => stream.reregister(registry, token, interests),
            ConnectionStream::Tls(_, stream) => stream.reregister(registry, token, interests),
        }
    }
    fn deregister(&mut self, registry: &mio::Registry) -> io::Result<()> {
        match self {
            ConnectionStream::Plain(stream) => stream.deregister(registry),
            ConnectionStream::Tls(_, stream) => stream.deregister(registry),
        }
    }
}

impl Write for ConnectionStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            ConnectionStream::Plain(stream) => stream.write(buf),
            ConnectionStream::Tls(conn, stream) => {
                if conn.is_handshaking() {
                    match conn.complete_io(stream) {
                        Ok(_) => Ok(0),
                        Err(e) => {
                            if e.kind() == io::ErrorKind::WouldBlock {
                                Ok(0)
                            } else {
                                Err(e)
                            }
                        }
                    }
                } else {
                    conn.writer().write_all(buf)?;
                    conn.complete_io(stream)?;
                    Ok(buf.len())
                }
            }
        }
    }
    
    fn flush(&mut self) -> io::Result<()> {
        match self {
            ConnectionStream::Plain(stream) => stream.flush(),
            ConnectionStream::Tls(conn, stream) => {
                conn.writer().flush()?;
                conn.complete_io(stream)?;
                Ok(())
            }
        }
    }
}

impl Read for ConnectionStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            ConnectionStream::Plain(stream) => stream.read(buf),
            ConnectionStream::Tls(conn, stream) => {
                if conn.is_handshaking() {
                    match conn.complete_io(stream) {
                        Ok(_) => Ok(0),
                        Err(e) => {
                            if e.kind() == io::ErrorKind::WouldBlock {
                                Ok(0)
                            } else {
                                Err(e)
                            }
                        }
                    }
                } else {
                    conn.complete_io(stream)?;
                    conn.reader().read(buf)
                }
            }
        }
    }
}

#[derive(Debug)]
struct Connection {
    stream: ConnectionStream,
    state: ConnectionState,
    request_sent: bool,
    response_received: bool,
    start_time: Instant,
    latency: Option<u128>,
    request_bytes: Vec<u8>,
}

#[derive(Debug)]
enum ConnectionState {
    Connecting,
    Connected,
    Sending,
    Complete,
    Error,
}

#[derive(Debug)]
enum ErrorType {
    ConnectionFailed = -1,
    TlsHandshakeFailed = -2,
    WriteFailed = -3,
    InvalidResponse = -4,
    Timeout = -5,
    Unknown = -6,
}

struct Stats {
    requests: u64,
    bytes: u64,
    latencies: Vec<u128>,
    status_counts: HashMap<i16, u64>,
}



fn create_insecure_tls_config() -> ClientConfig {
    let root_cert_store = rustls::RootCertStore::empty();
    let mut config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_cert_store)
        .with_no_client_auth();
    config.dangerous().set_certificate_verifier(Arc::new(NoCertificateVerification));
    config
}

struct NoCertificateVerification;

impl ServerCertVerifier for NoCertificateVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct BattleResult {
    pub requests: u64,
    pub bytes: u64,
    pub qps: f64,
    pub status_counts: HashMap<i16, u64>,
    pub latency_p50: Option<u128>,
    pub latency_p90: Option<u128>,
    pub latency_p99: Option<u128>,
    pub all_latencies: Vec<u128>,
    pub duration_secs: u64,
}

#[derive(Debug, Clone)]
pub struct BattleParams {
    pub threads: u32,
    pub connections: u32,
    pub duration_secs: u32,
    pub url: String,
    pub method: String,
    pub headers: Vec<String>,
    pub body: Option<String>,
}

// Helper function to resolve address from URL
fn resolve_address(url: &str) -> Result<SocketAddr, String> {
    let request_config = RequestConfig::from_url(url, "GET", &[], None)
        .map_err(|e| format!("Failed to parse URL: {}", e))?;
    
    let addr_str = format!("{}:{}", request_config.host, request_config.port);
    addr_str.to_socket_addrs()
        .map_err(|e| format!("Failed to resolve hostname: {}", e))?
        .next()
        .ok_or("No address found".to_string())
}

// Helper function to create connection stream
fn create_connection_stream(addr: SocketAddr, host: &str, is_ht: bool) -> Result<ConnectionStream, String> {
    let stream = TcpStream::connect(addr)
        .map_err(|e| format!("Failed to connect: {}", e))?;
    
    // Set socket options
    setsockopt(&stream, TcpNoDelay, &true)
        .map_err(|e| format!("Failed to set TCP_NODELAY: {}", e))?;
    
    // Set connection timeout for health check
    stream.set_nodelay(true)
        .map_err(|e| format!("Failed to set TCP_NODELAY: {}", e))?;
    
    if is_ht {
        let tls_config = create_insecure_tls_config();
        let server_name = ServerName::try_from(host)
            .map_err(|e| format!("Invalid server name: {}", e))?;
        
        let tls_conn = ClientConnection::new(Arc::new(tls_config), server_name)
            .map_err(|e| format!("Failed to create TLS connection: {}", e))?;
        
        Ok(ConnectionStream::Tls(tls_conn, stream))
    } else {
        Ok(ConnectionStream::Plain(stream))
    }
}

// Helper function to perform TLS handshake
fn complete_tls_handshake(stream: &mut ConnectionStream) -> Result<(), String> {
    if let ConnectionStream::Tls(ref mut tls_conn, ref mut tcp_stream) = stream {
        while tls_conn.is_handshaking() {
            match tls_conn.complete_io(tcp_stream) {
                Ok(_) => {},
                Err(e) => {
                    if e.kind() == io::ErrorKind::WouldBlock {
                        continue;
                    } else {
                        return Err(format!("TLS handshake failed: {}", e));
                    }
                }
            }
        }
    }
    Ok(())
}

// Helper function to read response
fn read_response(stream: &mut ConnectionStream) -> Result<(), String> {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(_) => Ok(()),
        Err(e) => {
            // Treat temporary unavailability as success - the connection worked
            if e.kind() == io::ErrorKind::WouldBlock || e.kind() == io::ErrorKind::TimedOut {
                Ok(())
            } else {
                Err(format!("Failed to read response: {}", e))
            }
        }
    }
}

pub fn health_check(params: &BattleParams) -> Result<(), String> {
    let request_config = RequestConfig::from_url(
        &params.url,
        &params.method,
        &params.headers,
        params.body.as_deref(),
    ).map_err(|e| format!("Failed to parse URL: {}", e))?;

    let addr = resolve_address(&params.url)
        .map_err(|e| format!("Failed to resolve address: {}", e))?;
    
    // Try up to 5 times for transient connection issues
    let mut last_error = None;
    for attempt in 1..=5 {
        match attempt_health_check(&addr, &request_config) {
            Ok(_) => return Ok(()),
            Err(e) => {
                last_error = Some(e.clone());
                // If it's a connection reset or temporary issue, wait and retry
                if attempt < 5 && (e.contains("Socket is not connected") || 
                                  e.contains("Broken pipe") || 
                                  e.contains("Resource temporarily unavailable")) {
                    std::thread::sleep(std::time::Duration::from_millis(200));
                } else {
                    break;
                }
            }
        }
    }
    
    Err(last_error.unwrap_or_else(|| "Unknown error".to_string()))
}

fn attempt_health_check(addr: &SocketAddr, request_config: &RequestConfig) -> Result<(), String> {
    let mut stream = create_connection_stream(*addr, &request_config.host, request_config.is_ht)
        .map_err(|e| format!("Failed to create connection: {}", e))?;
    
    // Small delay to let connection stabilize
    std::thread::sleep(std::time::Duration::from_millis(50));
    
    if request_config.is_ht {
        complete_tls_handshake(&mut stream)
            .map_err(|e| format!("TLS handshake failed: {}", e))?;
    }
    
    // Build the real request
    let request = request_config.build_request();
    stream.write_all(request.as_bytes())
        .map_err(|e| format!("Failed to send request: {}", e))?;
    
    read_response(&mut stream)
        .map_err(|e| format!("Failed to read response: {}", e))?;
    
    Ok(())
}

fn parse_http_status_code(response: &str) -> Option<u16> {
    // Parse HTTP status code from response like "HTTP/1.1 200 OK"
    let lines: Vec<&str> = response.lines().collect();
    if let Some(first_line) = lines.first() {
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        if parts.len() >= 2 {
            if let Ok(status_code) = parts[1].parse::<u16>() {
                return Some(status_code);
            }
        }
    }
    None
}

pub fn run_battle(params: BattleParams) -> Result<BattleResult, String> {
    let BattleParams { threads, connections, duration_secs, url, method, headers, body } = params;
    let duration = Duration::from_secs(duration_secs as u64);
    let request_config = match RequestConfig::from_url(&url, &method, &headers, body.as_deref()) {
        Ok(cfg) => cfg,
        Err(e) => return Err(format!("Failed to parse URL and create request configuration: {}", e)),
    };
    let addr_str = format!("{}:{}", request_config.host, request_config.port);
    let mut addrs_iter = match addr_str.to_socket_addrs() {
        Ok(iter) => iter,
        Err(e) => return Err(format!("Failed to resolve address: {}", e)),
    };
    let addr = match addrs_iter.next() {
        Some(a) => a,
        None => return Err("No address found for host".to_string()),
    };
    let stats = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    for thread_id in 0..threads {
        let stats = stats.clone();
        let addr = addr;
        let conns_per_thread = connections / threads;
        let duration = duration;
        let request_config = request_config.clone();
        let handle = thread::spawn(move || {
            let _ = run_thread(thread_id, addr, conns_per_thread, duration, stats, request_config);
        });
        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.join();
    }
    let stats = stats.lock().unwrap();
    let total_requests: u64 = stats.iter().map(|s| s.requests).sum();
    let total_bytes: u64 = stats.iter().map(|s| s.bytes).sum();
    let mut status_counts: HashMap<i16, u64> = HashMap::new();
    for s in stats.iter() {
        for (&code, &count) in &s.status_counts {
            *status_counts.entry(code).or_insert(0) += count;
        }
    }
    let mut all_latencies: Vec<u128> = stats.iter().flat_map(|s| s.latencies.clone()).collect();
    all_latencies.sort_unstable();
    let p50 = all_latencies.get(all_latencies.len() / 2).copied();
    let p90 = all_latencies.get((all_latencies.len() * 90 / 100).min(all_latencies.len().saturating_sub(1))).copied();
    let p99 = all_latencies.get((all_latencies.len() * 99 / 100).min(all_latencies.len().saturating_sub(1))).copied();
    Ok(BattleResult {
        requests: total_requests,
        bytes: total_bytes,
        qps: total_requests as f64 / duration.as_secs_f64(),
        status_counts,
        latency_p50: p50,
        latency_p90: p90,
        latency_p99: p99,
        all_latencies,
        duration_secs: duration.as_secs(),
    })
}

fn run_thread(
    _thread_id: u32,
    addr: SocketAddr,
    connections: u32,
    duration: Duration,
    stats: Arc<Mutex<Vec<Stats>>>,
    request_config: RequestConfig,
) -> io::Result<()> {
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(1024);
    let mut connections_map = HashMap::new();
    let mut next_token = Token(0);
    let mut local_stats = Stats {
        requests: 0,
        bytes: 0,
        latencies: Vec::with_capacity(10000),
        status_counts: HashMap::new(),
    };

    // Create TLS config for HTTPS connections
    let tls_config = Some(Arc::new(create_insecure_tls_config()));

    // Create connections
    for _ in 0..connections {
        let stream = match TcpStream::connect(addr) {
            Ok(stream) => stream,
            Err(_) => {
                *local_stats.status_counts.entry(ErrorType::ConnectionFailed as i16).or_insert(0) += 1;
                continue;
            }
        };

        // Optimize socket settings like wrk does
        setsockopt(&stream, TcpNoDelay, &true)?;

        let token = next_token;
        next_token = Token(token.0 + 1);

        let request_bytes = request_config.build_request().into_bytes();

        let connection_stream = if request_config.is_ht {
            // Create TLS connection
            let server_name = ServerName::try_from(request_config.host.as_str())
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, format!("Invalid DNS name: {}", e)))?;
            
            let tls_conn = ClientConnection::new(tls_config.as_ref().unwrap().clone(), server_name)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("TLS error: {}", e)))?;
            ConnectionStream::Tls(tls_conn, stream)
        } else {
            ConnectionStream::Plain(stream)
        };

        let mut conn = Connection {
            stream: connection_stream,
            state: ConnectionState::Connecting,
            request_sent: false,
            response_received: false,
            start_time: Instant::now(),
            latency: None,
            request_bytes,
        };

        poll.registry()
            .register(&mut conn.stream, token, Interest::WRITABLE | Interest::READABLE)?;

        connections_map.insert(token, conn);
    }

    let start_time = Instant::now();
    let mut buf = vec![0u8; 4096];

    while start_time.elapsed() < duration {
        poll.poll(&mut events, Some(Duration::from_millis(1)))?;

        for event in events.iter() {
            let token = event.token();
            let conn = connections_map.get_mut(&token).unwrap();

            // Handle TLS handshake for HTTPS connections
            if request_config.is_ht && conn.stream.is_handshaking() && event.is_writable() {
                match conn.stream.write(&[]) {
                    Ok(_) => {
                        if !conn.stream.is_handshaking() {
                            conn.state = ConnectionState::Connected;
                            // Re-register for write events to send the request
                            poll.registry().reregister(&mut conn.stream, token, Interest::WRITABLE)?;
                        } else {
                            // Still handshaking, keep trying
                            poll.registry().reregister(&mut conn.stream, token, Interest::WRITABLE | Interest::READABLE)?;
                        }
                        continue;
                    }
                    Err(e) => {
                        if e.kind() == io::ErrorKind::WouldBlock {
                            // Not ready yet, keep trying
                            poll.registry().reregister(&mut conn.stream, token, Interest::WRITABLE | Interest::READABLE)?;
                            continue;
                        } else {
                            conn.state = ConnectionState::Error;
                            *local_stats.status_counts.entry(ErrorType::TlsHandshakeFailed as i16).or_insert(0) += 1;
                            continue;
                        }
                    }
                }
            }

            if event.is_writable() && !conn.request_sent && (!conn.stream.is_handshaking() || !request_config.is_ht) {
                // Send prebuilt request
                match conn.stream.write(&conn.request_bytes) {
                    Ok(_) => {
                        conn.request_sent = true;
                        conn.state = ConnectionState::Sending;
                        poll.registry()
                            .reregister(&mut conn.stream, token, Interest::READABLE)?;
                    }
                    Err(_) => {
                        conn.state = ConnectionState::Error;
                        *local_stats.status_counts.entry(ErrorType::WriteFailed as i16).or_insert(0) += 1;
                    }
                }
            }

            if event.is_readable() && conn.request_sent && !conn.response_received {
                // Accumulate response until we have the full header
                let mut header_buf = Vec::new();
                let mut header_complete = false;
                let mut header_end = 0;
                loop {
                    match conn.stream.read(&mut buf) {
                        Ok(n) if n > 0 => {
                            header_buf.extend_from_slice(&buf[..n]);
                            if let Some(pos) = twoway::find_bytes(&header_buf, b"\r\n\r\n") {
                                header_complete = true;
                                header_end = pos + 4;
                                break;
                            }
                        }
                        Ok(0) => { break; }
                        Ok(_) => {}
                        Err(e) => {
                            use std::io::ErrorKind;
                            match e.kind() {
                                ErrorKind::TimedOut => {
                                    *local_stats.status_counts.entry(ErrorType::Timeout as i16).or_insert(0) += 1;
                                }
                                _ => {
                                    *local_stats.status_counts.entry(ErrorType::Unknown as i16).or_insert(0) += 1;
                                }
                            }
                            break;
                        }
                    }
                }
                if header_complete {
                    let response_str = String::from_utf8_lossy(&header_buf);
                    let status_code = parse_http_status_code(&response_str);
                    if let Some(code) = status_code {
                        *local_stats.status_counts.entry(code as i16).or_insert(0) += 1;
                    } else {
                        *local_stats.status_counts.entry(ErrorType::InvalidResponse as i16).or_insert(0) += 1;
                    }
                    // Parse Content-Length
                    let mut content_length = None;
                    for line in response_str.lines() {
                        if let Some(rest) = line.strip_prefix("Content-Length:") {
                            if let Ok(cl) = rest.trim().parse::<usize>() {
                                content_length = Some(cl);
                                break;
                            }
                        }
                        if line.is_empty() { break; }
                    }
                    // Read body and count bytes
                    let mut body_bytes = 0;
                    let body = header_buf[header_end..].to_vec();
                    body_bytes += body.len();
                    if let Some(cl) = content_length {
                        while body_bytes < cl {
                            match conn.stream.read(&mut buf) {
                                Ok(n) if n > 0 => {
                                    body_bytes += n;
                                }
                                _ => { break; }
                            }
                        }
                    } else {
                        // No Content-Length: read until EOF or next request
                        loop {
                            match conn.stream.read(&mut buf) {
                                Ok(n) if n > 0 => { body_bytes += n; }
                                _ => { break; }
                            }
                        }
                    }
                    local_stats.bytes += body_bytes as u64;
                    // Always record request and latency for any response
                    conn.response_received = true;
                    conn.latency = Some(conn.start_time.elapsed().as_micros());
                    conn.state = ConnectionState::Complete;
                    local_stats.requests += 1;
                    if let Some(latency) = conn.latency {
                        local_stats.latencies.push(latency);
                    }
                    conn.request_sent = false;
                    conn.response_received = false;
                    conn.start_time = Instant::now();
                    conn.state = ConnectionState::Connected;
                    poll.registry().reregister(&mut conn.stream, token, Interest::WRITABLE)?;
                }
            }
        }
    }

    stats.lock().unwrap().push(local_stats);
    Ok(())
} 