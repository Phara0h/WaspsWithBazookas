use worker::*;
use console_error_panic_hook::set_once as set_panic_hook;

mod types;
mod hive_client;
mod bazooka_client;
mod routes;
mod wasp_state;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    // Set panic hook for better error messages
    set_panic_hook();

    // Create router
    let router = Router::new();

    // Add routes
    let router = router
        .get_async("/boop", routes::handle_boop)
        .put_async("/fire", routes::handle_fire)
        .get_async("/ceasefire", routes::handle_ceasefire)
        .delete_async("/die", routes::handle_die)
        .get_async("/battlereport", routes::handle_battlereport)
        .get_async("/health", routes::handle_health)
        .post_async("/bazooka_completed", routes::handle_bazooka_completed);

    // Handle the request
    router.run(req, env).await
}
