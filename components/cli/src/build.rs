use astryx::error::*;

pub(crate) fn build(file: String) -> AstryxResult<()> {
    match astryx::render(&file) {
        Ok(_) => println!("ok."),
        Err(e) => println!("error: {:?}", e)
    }
    Ok(())
}
