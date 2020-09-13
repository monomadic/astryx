use astryx::error::AstryxResult;
use simple_server::{Server, StatusCode};
use std::path::PathBuf;
use crate::render::RenderErrorAsHTML;

pub(crate) fn start(file: PathBuf, port: u32) -> AstryxResult<()> {
    let host = "127.0.0.1";
    let port = port.to_string();
    // let mut file = crate::filesystem::read_file(file)?;

    let mut server = Server::new(move |request, mut response| {
        // info!("Request received. {} {}", request.method(), request.uri());
        let path = request.uri().path();

        if path == "/ast" {
            let ast = crate::filesystem::read_file(&file)
                .map(|file| format!("{:#?}", parser::parse(&file)));

            match ast {
                Ok(page) => Ok(response.body(page.as_bytes().to_vec())?),
                Err(e) => Ok(response.body(format!("Error: {}", e.to_html()).as_bytes().to_vec())?),
            }
        } else {
            let pages = astryx::render(&file);

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
                    println!("ERROR: {}", e);

                    Ok(response.body(
                    format!("<html style='background-color: black;color: white;'><body><h1>Error :(</h1><pre>{}</pre></body></html>", &e)
                        .as_bytes()
                        .to_vec(),
                )?)
                }
            }
        }
    });

    server.set_static_directory("public");

    println!("listening on http://{}:{}/", host, port);
    server.listen(host, &port);
}
