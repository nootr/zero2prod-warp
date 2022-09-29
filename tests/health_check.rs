use newsletter::get_filter;

#[tokio::test]
async fn test_health_check() {
    let filter = get_filter();

    let value = warp::test::request()
        .path("/healthz")
        .filter(&filter)
        .await
        .unwrap();

    assert_eq!(value, "");
}
