use tokio;
use warp::Filter;

fn get_healthz() -> &'static str {
    ""
}

#[tokio::main]
async fn main() {
    let get_routes = warp::get().and(warp::path("healthz").map(get_healthz));

    warp::serve(get_routes).run(([127, 0, 0, 1], 8000)).await;
}
