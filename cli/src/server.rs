use astryx::AstryxResult;
use simple_server::{Server, StatusCode};
use std::path::Path;

pub(crate) fn start<'a, P: AsRef<Path>>(input: P, port: u32) -> AstryxResult<()> {
    let host = "127.0.0.1";
    let path: String = input.as_ref().to_str().unwrap().into();

    let mut server = Server::new(move |request, mut response| {
        let request_path = request.uri().path();

        match request_path {
            // "/__ast" => {
            //     let ast = read_to_string(&path)
            //         .map_err(AstryxError::from)
            //         .map(|ref file| format!("{:#?}", parser::run(file, &path)));
            //
            //     match ast {
            //         Ok(page) => Ok(response.body(page.as_bytes().to_vec())?),
            //         Err(e) => Ok(response.body(display_error(e).as_bytes().to_vec())?),
            //     }
            // }
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

                let result = astryx::parse_from_file(&path, None);

                // if request_path.contains("svg") {
                //     response.header("content-type", "image/svg+xml");
                //     // return Ok(response.body(svgfile.as_bytes().to_vec())?);
                // }

                let body = match result {
                    Ok(site) => {
                        // println!("site.pages {:?}", site.render_pages());
                        // match site.render_pages().get(request_path) {
                        //     Some(page) => page.clone(),
                        //     None => {
                        //         response.status(StatusCode::NOT_FOUND);
                        //         format!("<h1>404</h1><p>Path not found: {}<p>", request_path)
                        //     }
                        // }

                        // println!("{}", request_path.split_at(1).1);
                        // split_at removes the /
                        match site.render_as_bytes().get(request_path) {
                            Some(page) => page.clone(),
                            None => {
                                response.status(StatusCode::NOT_FOUND);
                                format!(
                                    "<h1>404</h1><p>Path not found: {}<p><p>pages: {:?}",
                                    request_path,
                                    site.render_as_bytes().keys() // fixme: redundant, make better 404 page
                                )
                                .as_bytes()
                                .to_vec()
                            }
                        }
                    }
                    Err(e) => {
                        response.status(StatusCode::INTERNAL_SERVER_ERROR);
                        // let error_text = display_error(e);
                        format!("{:?}", e).as_bytes().to_vec()
                        // html_error_page(&error_text)
                    }
                };

                Ok(response.body(body)?)
            }
        }
    });

    server.set_static_directory("public");

    println!("listening on http://{}:{}/", host, port);
    server.listen(host, &port.to_string());
}
