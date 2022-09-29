use newsletter::get_filter;

#[tokio::main]
async fn main() {
    let filter = get_filter();
    warp::serve(filter).run(([127, 0, 0, 1], 8000)).await;
}
