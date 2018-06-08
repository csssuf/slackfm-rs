#[get("/health")]
fn health_check() -> Result<(), ()> {
    Ok(())
}
