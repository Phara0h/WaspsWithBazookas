use worker::*;
use crate::types::BazookaCommand;

pub async fn handle_boop(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    Response::ok("Oh hi from Cloudflare Wasp!")
}

pub async fn handle_fire(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    // Get the durable object namespace
    let namespace = ctx.durable_object("WASP_DURABLE_OBJECT")?;
    let stub = namespace.id_from_name("wasp")?.get_stub()?;
    
    // Forward the request to the durable object
    let response = stub.fetch_with_request(req).await?;
    Ok(response)
}

pub async fn handle_ceasefire(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    // Get the durable object namespace
    let namespace = ctx.durable_object("WASP_DURABLE_OBJECT")?;
    let stub = namespace.id_from_name("wasp")?.get_stub()?;

    
    // Create a GET request to the durable object
    let mut req_init = RequestInit::new();
    req_init.with_method(Method::Get);
    let req = Request::new_with_init("https://durable-object/ceasefire", &req_init)?;
    
    // Forward the request to the durable object
    let response = stub.fetch_with_request(req).await?;
    Ok(response)
}

pub async fn handle_die(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    // Get the durable object namespace
    let namespace = ctx.durable_object("WASP_DURABLE_OBJECT")?;
    let stub = namespace.id_from_name("wasp")?.get_stub()?;
    
    // Create a DELETE request to the durable object
    let mut req_init = RequestInit::new();
    req_init.with_method(Method::Delete);
    let req = Request::new_with_init("https://durable-object/die", &req_init)?;
    
    // Forward the request to the durable object
    let response = stub.fetch_with_request(req).await?;
    Ok(response)
}

pub async fn handle_battlereport(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    // Get the durable object namespace
    let namespace = ctx.durable_object("WASP_DURABLE_OBJECT")?;
    let stub = namespace.id_from_name("wasp")?.get_stub()?;
    
    // Create a GET request to the durable object
    let mut req_init = RequestInit::new();
    req_init.with_method(Method::Get);
    let req = Request::new_with_init("https://durable-object/battlereport", &req_init)?;
    
    // Forward the request to the durable object
    let response = stub.fetch_with_request(req).await?;
    Ok(response)
}

pub async fn handle_health(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let hive_url = ctx.var("HIVE_URL").ok();
    let bazooka_worker_url = ctx.var("BAZOOKA_WORKER_URL").ok();
    
    let response = serde_json::json!({
        "hive": if hive_url.is_some() { "ok" } else { "not configured" },
        "bazooka": if bazooka_worker_url.is_some() { "ok" } else { "not configured" },
        "wasp_id": "CloudflareWasp",
        "port": "443"
    });
    
    Response::from_json(&response)
}

// New endpoint for Bazooka workers to report completion
pub async fn handle_bazooka_completed(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    // Get the durable object namespace
    let namespace = ctx.durable_object("WASP_DURABLE_OBJECT")?;
    let stub = namespace.id_from_name("wasp")?.get_stub()?;
    
    // Create a POST request to the durable object with the bazooka completion data
    let command: BazookaCommand = req.json().await?;
    let mut req_init = RequestInit::new();
    req_init.with_method(Method::Post);
    req_init.with_body(Some(serde_json::to_string(&command)?.into()));
    let req = Request::new_with_init("https://durable-object/bazooka_completed", &req_init)?;
    
    // Forward the request to the durable object
    let response = stub.fetch_with_request(req).await?;
    Ok(response)
} 