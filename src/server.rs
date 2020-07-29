use astryx::error::AstryxResult;
use simple_server::{Server, StatusCode};
use std::path::PathBuf;

pub(crate) fn start(file: PathBuf, port: u32) -> AstryxResult<()> {
    let host = "127.0.0.1";
    let port = port.to_string();

    let mut server = Server::new(move |request, mut response| {
        // info!("Request received. {} {}", request.method(), request.uri());
        let path = request.uri().path();
        let pages = astryx::render(file.clone());

        println!("{} {}", request.method(), path);

        if path.contains("svg") {
            response.header("content-type", "image/svg+xml");
            // return Ok(response.body(svgfile.as_bytes().to_vec())?);
        }

        match pages {
            Ok(pages) => match pages.get(path) {
                Some(page) => Ok(response.body(page.as_bytes().to_vec())?),
                None => {
                    response.status(StatusCode::NOT_FOUND);
                    Ok(response.body(
                        format!("<h1>404</h1><p>Path not found: {}<p>", path)
                            .as_bytes()
                            .to_vec(),
                    )?)
                }
            },
            Err(e) => {
                response.status(StatusCode::INTERNAL_SERVER_ERROR);
                println!("ERROR: {:#?}", e);

                Ok(response.body(
                    format!("<html style='background-color: black;color: white;'><body><h1>Error :(</h1><pre>{}</pre>\n\n<pre>{:#?}</pre></body></html>", &e.msg, &e.state)
                        .as_bytes()
                        .to_vec(),
                )?)
            }
        }

        // request.method() -> &Method::GET
    });

    server.set_static_directory("public");

    println!("listening on http://{}:{}/", host, port);
    server.listen(host, &port);
}
