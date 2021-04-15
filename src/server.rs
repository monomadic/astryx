use error::{
    display::{display_error, html_error_page},
    AstryxError, AstryxResult,
};
use models::{Site, State};
use simple_server::{Server, StatusCode};
use std::cell::RefCell;
use std::fs::read_to_string;
use std::path::Path;
use std::rc::Rc;

// todo: give the server an already complete project
pub(crate) fn start<'a, P: AsRef<Path>>(path: P, port: u32) -> AstryxResult<()> {
    let host = "127.0.0.1";
    let port = port.to_string();
    let path: String = path.as_ref().to_str().unwrap().into();

    let mut server = Server::new(move |request, mut response| {
        // info!("Request received. {} {}", request.method(), request.uri());
        let request_path = request.uri().path();

        match request_path {
            "/__ast" => {
                let ast = read_to_string(&path)
                    .map_err(AstryxError::from)
                    .map(|ref file| format!("{:#?}", parser::run(file, &path)));

                match ast {
                    Ok(page) => Ok(response.body(page.as_bytes().to_vec())?),
                    Err(e) => Ok(response.body(display_error(&e, &path).as_bytes().to_vec())?),
                }
            }
            // "/__program" => {
            //     let file = read_to_string(&path)?;
            //     let program = parser::run(&file)
            //         .map_err(AstryxError::from)
            //         .and_then(|nodes| {
            //             interpreter::run(&nodes, Rc::new(RefCell::new(State::new())))
            //                 .map_err(AstryxError::from)
            //         });

            //     match program {
            //         Ok(prog) => Ok(response.body(
            //             prog.iter()
            //                 .map(|inst| format!("{:?}", inst))
            //                 .collect::<Vec<String>>()
            //                 .join("\n")
            //                 .as_bytes()
            //                 .to_vec(),
            //         )?),
            //         Err(e) => Ok(response.body(display_error(&e, &path).as_bytes().to_vec())?),
            //     }
            // }
            // "/__pages" => {
            //     // let file = read_to_string(&path)?;
            //     // let body = match render(&file) {
            //     //     Ok(project) => project
            //     //         .pages
            //     //         .iter()
            //     //         .map(|(path, page)| {
            //     //             format!(
            //     //                 "<p>{}</p><pre style='background-color:#AAA;padding:10px'>{}</pre><hr>",
            //     //                 path, page
            //     //             )
            //     //         })
            //     //         .collect(),
            //     //     Err(e) => {
            //     //         response.status(StatusCode::INTERNAL_SERVER_ERROR);
            //     //         let error_text = display_error(&e, &path);
            //     //         println!("{}", error_text);

            //     //         html_error_page(&error_text)
            //     //     }
            //     // };
            //     let body = String::from("fixme");

            //     Ok(response.body(body.as_bytes().to_vec())?)
            // }
            _ => {
                println!("{} {}", request.method(), request_path);

                // let file = read_to_string(&path)?;

                let state = Rc::new(RefCell::new(State::new()));

                let result = parser::run(&read_to_string(&path)?, &path)
                    .map_err(AstryxError::from)
                    .and_then(|nodes| interpreter::run(&nodes, state))
                    .map(Site::render);

                // if request_path.contains("svg") {
                //     response.header("content-type", "image/svg+xml");
                //     // return Ok(response.body(svgfile.as_bytes().to_vec())?);
                // }

                let body = match result {
                    Ok(site) => match site.documents.get(request_path) {
                        Some(page) => page.clone(),
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
