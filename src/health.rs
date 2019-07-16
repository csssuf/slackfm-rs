#[get("/health")]
pub(crate) fn health_check() -> Result<(), ()> {
    Ok(())
}
