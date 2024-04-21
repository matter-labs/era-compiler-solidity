use crate::util::counter::Counter;

use crate::yul::parser::identifier::Identifier as YulIdentifier;

type CodeID = u32;
type BlockID = u32;

#[derive(Debug, Clone, PartialEq, Eq)]
enum LocationStep {
    Module(YulIdentifier),
    Function(YulIdentifier),
    Code(CodeID),
    Block(BlockID),
}

impl LocationStep {
    fn full_name_contribution(&self) -> String {
        match self {
            LocationStep::Module(id) => id.inner.to_string(),
            LocationStep::Function(id) => id.inner.to_string(),
            LocationStep::Code(id) => format!("code{id}"),
            LocationStep::Block(id) => format!("block{id}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location {
    stack: Vec<LocationStep>,
}

// impl Identifier for Location {
//     fn name(&self) -> String {
//         self.stack.iter().fold(String::from(""), |x, y| -> String {
//             let contribution = y.full_name_contribution();
//             format!("{x}_{contribution}")
//         })
//     }
// }

pub(crate) struct LocationBuilder {
    pub elements: Vec<LocationStep>,
    block_counter: Counter,
    code_counter: Counter,
}

impl LocationBuilder {
    pub fn new() -> Self {
        Self {
            elements: [].to_vec(),
            block_counter: Counter::new(),
            code_counter: Counter::new(),
        }
    }

    pub fn pop(&mut self) {
        self.elements.pop();
    }

    pub fn enter_block(&mut self) {
        self.elements
            .push(LocationStep::Block(u32::from(self.block_counter)));
        self.block_counter.increment();
    }
    pub fn enter_function(&mut self, ident: YulIdentifier) {
        self.elements.push(LocationStep::Function(ident));
        self.block_counter.increment();
    }

    pub fn enter_code(&mut self) {
        self.elements
            .push(LocationStep::Code(u32::from(self.code_counter)));
        self.code_counter.increment();
    }
}
