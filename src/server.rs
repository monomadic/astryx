use crate::{
    error::*,
    interpreter::{self, State},
};
use simple_server::{Method, Server, StatusCode};
use std::path::PathBuf;

pub(crate) fn start(file: PathBuf, port: u32) -> AstryxResult<()> {
    let host = "127.0.0.1";
    let port = port.to_string();

    let server = Server::new(move |request, mut response| {
        // info!("Request received. {} {}", request.method(), request.uri());
        let path = request.uri().path();
        let rendered_page = do_request(path, file.clone());

        match rendered_page {
            Ok(page) => Ok(response.body(page.as_bytes().to_vec())?),
            Err(e) => {
                // TODO match on error type to render page correctly
                response.status(StatusCode::INTERNAL_SERVER_ERROR);
                Ok(response.body(
                    format!("<h1>Error :(</h1><pre>{:?}</pre>", e)
                        .as_bytes()
                        .to_vec(),
                )?)
                // response.status(StatusCode::NOT_FOUND);
                // Ok(response.body("<h1>404</h1><p>Not found!<p>".as_bytes().to_vec())?)
            }
        }

        // request.method() -> &Method::GET
    });

    server.listen(host, &port);
}

fn do_request(path: &str, file: PathBuf) -> AstryxResult<String> {
    let state = &mut State::new();
    let file = crate::filesystem::read_file(file.clone())?;
    let nodes = crate::parse::parse(&file)?;
    let _ = interpreter::run(&nodes, state);

    println!("state: {:#?}", state);
    state
        .page_buffers
        .insert("/state".into(), format!("{:#?}", state));
    state
        .page_buffers
        .insert("/nodes".into(), format!("{:#?}", nodes));

    state
        .page_buffers
        .get(path)
        .map(|o| o.clone())
        .ok_or(AstryxError::ParseError("could not get page buffers".into()))
}
