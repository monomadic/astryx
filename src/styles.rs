
pub(crate) struct LayoutContainer {
    direction: LayoutDirection,
    justification: LayoutJustification,
}

pub(crate) enum LayoutDirection {
    Rows,
    Columns,
}

pub(crate) enum LayoutJustification {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

// pub(crate) struct StyleRule {
//     rule: String,
//     styles: String, // TODO: enum type
// }

// enum Style {
//     Display(DisplayStyle)
// }

// enum DisplayStyle {
//     Block,
//     Flex,
// }

// // REVERSE MATCHING PROBABLY WORKS BETTER! eg. the rule is the "large" bit of the rule
// // and each rule has only 1 style applied
// // this allows compression (you can't compress border: 1px solid red but you can compress the styles it matches)

// #[derive(Debug, Clone)]
// pub struct StyleCollector {
//     styles: HashMap<String, String> // <Style, Vec<String>>
// }

// impl StyleCollector {
//     pub fn add(matcher: &str, style: &str) { // result?

//     }

//     pub fn as_compressed_css(&self) -> String {
//         // compress class names eg .button becomes .a etc
//         String::new()
//     }
// }
