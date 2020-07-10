use crate::{
    error::*,
    interpreter::{self, State},
    parse,
};
use simple_server::{Method, Server, StatusCode};
use std::path::PathBuf;

pub(crate) fn start(file: PathBuf, port: u32) -> AstryxResult<()> {
    let host = "127.0.0.1";
    let port = port.to_string();
    let server = Server::new(move |request, mut response| {
        // info!("Request received. {} {}", request.method(), request.uri());
        let state = &mut State::new();
        let file = crate::filesystem::read_file(file.clone()).expect("file unwrap");
        let (_, nodes) =
            crate::parse::run(&file).expect("method parse");
        let _ = interpreter::run(&nodes, state);

        let path = request.uri().path();

        println!("buffers: {:?}", state.page_buffers);

        match state.page_buffers.get(path) {
            Some(page) => Ok(response.body(page.as_bytes().to_vec())?),
            None => {
                response.status(StatusCode::NOT_FOUND);
                Ok(response.body("<h1>404</h1><p>Not found!<p>".as_bytes().to_vec())?)
            }
        }

        // request.method() -> &Method::GET
    });

    server.listen(host, &port);
    Ok(())
}
