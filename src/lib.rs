use warp::Filter;

fn get_health_check() -> &'static str {
    ""
}

pub fn get_filter() -> impl Filter<Extract = (&'static str,), Error = warp::Rejection> + Copy {
    warp::get().and(warp::path("healthz").map(get_health_check))
}
