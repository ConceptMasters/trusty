pub async fn healthz() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::http::StatusCode::OK)
}
