use crate::error::AstryxError;

pub(crate) fn error_page(e: AstryxError) -> Vec<u8> {
    format!("Error: {:?}", e).as_bytes().to_vec()
}
