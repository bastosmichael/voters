#[get("/voters")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let voters = web::block(|| Voters::find_all()).await.unwrap();
    Ok(HttpResponse::Ok().json(voters))
}
#[get("/voters/{id}")]
async fn find(id: web::Path) -> Result<HttpResponse, CustomError> {
    let voter = Voters::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(voter))
}
#[post("/voters")]
async fn create(voter: web::Json) -> Result<HttpResponse, CustomError> {
    let voter = Voters::create(voter.into_inner())?;
    Ok(HttpResponse::Ok().json(voter))
}
#[put("/voters/{id}")]
async fn update(
    id: web::Path,
    voter: web::Json,
) -> Result<HttpResponse, CustomError> {
    let voter = Voters::update(id.into_inner(), voter.into_inner())?;
    Ok(HttpResponse::Ok().json(voter))
}
#[delete("/voters/{id}")]
async fn delete(id: web::Path) -> Result<HttpResponse, CustomError> {
    let deleted_voter = Voters::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_voter })))
}
pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
}
