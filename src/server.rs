use crate::{
    error::{display_error, html_error_page, AstryxError, AstryxResult},
    render::render,
};
use simple_server::{Server, StatusCode};
use std::fs::read_to_string;

pub(crate) fn start<'a>(path: String, port: u32) -> AstryxResult<'a, ()> {
    let host = "127.0.0.1";
    let port = port.to_string();

    let mut server = Server::new(move |request, mut response| {
        // info!("Request received. {} {}", request.method(), request.uri());
        let request_path = request.uri().path();

        match request_path {
            "/__ast" => {
                let ast = read_to_string(&path)
                    .map_err(AstryxError::from)
                    .map(|ref file| format!("{:#?}", parser::run(file)));

                match ast {
                    Ok(page) => Ok(response.body(page.as_bytes().to_vec())?),
                    Err(e) => Ok(response.body(display_error(&e, &path).as_bytes().to_vec())?),
                }
            }
            _ => {
                println!("{} {}", request.method(), request_path);

                let file = read_to_string(&path)?;

                // if request_path.contains("svg") {
                //     response.header("content-type", "image/svg+xml");
                //     // return Ok(response.body(svgfile.as_bytes().to_vec())?);
                // }

                let body = match render(&file) {
                    Ok(project) => match project.pages.get("/") {
                        Some(page) => page.into(),
                        None => {
                            response.status(StatusCode::NOT_FOUND);
                            format!("<h1>404</h1><p>Path not found: {}<p>", request_path)
                        }
                    },
                    Err(e) => {
                        response.status(StatusCode::INTERNAL_SERVER_ERROR);
                        let error_text = display_error(&e, &path);
                        println!("{}", error_text);

                        html_error_page(&error_text)
                    }
                };

                Ok(response.body(body.as_bytes().to_vec())?)
            }
        }
    });

    server.set_static_directory("public");

    println!("listening on http://{}:{}/", host, port);
    server.listen(host, &port);
}
