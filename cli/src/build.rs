use astryx::{AstryxError, AstryxResult};
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

// /// Compiles an input file into an output graph
// pub fn build<P: AsRef<Path>>(input: P, check: bool, stdout: bool) -> AstryxResult<()> {
//     let file = std::fs::read_to_string(&input).map_err(|e| AstryxError::IO(e))?;
//     let path: String = input.as_ref().to_str().unwrap().into();
//     let state = Rc::new(RefCell::new(State::new()));
//
//     let (rem, lines) = nom_indent::indent(&file, &path)
//         .map_err(|_| AstryxError::Generic("indent error".into()))?;
//
//     // check for unexpected remaining content from indenter
//     if !rem.len() == 0 {
//         // fixme: return astryxerror
//         panic!("non empty!");
//     }
//
//     parser::parse(lines)
//         .and_then(|statements| interpreter::run(&statements, state))
//         .map(Site::render)
//         .map(|site| {
//             if check {
//                 println!("read only check. skipping file write...")
//             } else {
//                 if stdout {
//                     for (route, document) in site.documents {
//                         println!("{}: {}", route, document);
//                     }
//                 } else {
//                     site.write()
//                 }
//             }
//         })
//     // .map_err(AstryxError::from)
//     // nom_indent::indent(&file, &path)
//     //     .map_err(AstryxError::from)
//     //     .and_then(|(_, lines)| parser::parse(lines))
//     //     .map(|nodes| interpreter::run(&nodes, state))
//     //     .map(Site::render)
//     //     .map(|site| site.write())
// }
