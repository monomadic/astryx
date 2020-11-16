use html::HTMLNode;
use program::*;
use rctree::Node;

// #[test]
// fn test() {
//     let mut program = Node::new(ProgramInstruction::Root);
//     assert_eq!("(root)", &program.inspect());

//     let element = Node::new(ProgramInstruction::HTMLElement(HTMLNode::Text(String::from(
//         "text",
//     ))));
//     assert_eq!("(el)", &element.inspect());

//     program.append(element);
//     assert_eq!("(root (el))", &program.inspect());

//     let element2 = Node::new(ProgramInstruction::HTMLElement(HTMLNode::Text(String::from(
//         "two",
//     ))));
//     program.append(element2);
//     assert_eq!("(root (el), (el))", &program.inspect());
// }
