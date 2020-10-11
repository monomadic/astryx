use crate::{errorpage::error_page, error::{AstryxError, AstryxResult, display_error}};
use simple_server::{Server, StatusCode};

pub(crate) fn start<'a>(path: String, port: u32) -> AstryxResult<'a, ()> {
    let host = "127.0.0.1";
    let port = port.to_string();

    let mut server = Server::new(move |request, mut response| {
        // info!("Request received. {} {}", request.method(), request.uri());
        let request_path = request.uri().path();

        if request_path == "/ast" {
            let ast = std::fs::read_to_string(&path)
                .map_err(AstryxError::from)
                .map(|file| format!("{:#?}", parser::run(&*file)));

            match ast {
                Ok(page) => Ok(response.body(page.as_bytes().to_vec())?),
                Err(e) => Ok(response.body(error_page(e))?),
            }
        } else {
            let file = std::fs::read_to_string(&path).expect("file to read");
            let pages = crate::render::render(&file);

            println!("{} {}", request.method(), request_path);

            if request_path.contains("svg") {
                response.header("content-type", "image/svg+xml");
                // return Ok(response.body(svgfile.as_bytes().to_vec())?);
            }

            match pages {
                Ok(pages) => match pages.get(0) {
                    // Some(page) => Ok(response.body(page.as_bytes().to_vec())?),
                    Some(page) => Ok(response.body(format!("{:?}", page).as_bytes().to_vec())?),
                    None => {
                        response.status(StatusCode::NOT_FOUND);
                        Ok(response.body(
                            format!("<h1>404</h1><p>Path not found: {}<p>", request_path)
                                .as_bytes()
                                .to_vec(),
                        )?)
                    }
                },
                Err(e) => {
                    response.status(StatusCode::INTERNAL_SERVER_ERROR);
                    let error_view = display_error(e, &path);
                    println!("ERROR: {}", error_view);

                    Ok(response.body(
                    format!("<html style='background-color: black;color: white;'><body><h1>Error :(</h1><pre>{}</pre></body></html>", error_view)
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
