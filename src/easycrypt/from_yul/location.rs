use crate::util::counter::Counter;

type CodeID = u32;
type BlockID = u32;
type IfID = u32;
type ForID = u32;
type Name = String;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum LocationStep {
    Module(Name),
    Function(Name),
    Code(CodeID),
    Block(BlockID),
    IfCondition(IfID),
    IfThen(IfID),
    IfElse(IfID),
    For1(ForID),
    For2(ForID),
    For3(ForID),
}

impl LocationStep {
    fn full_name_contribution(&self) -> String {
        match self {
            LocationStep::Module(name) => name.to_string(),
            LocationStep::Function(id) => id.to_string(),
            LocationStep::Code(id) => format!("code{id}"),
            LocationStep::Block(id) => format!("block{id}"),
            LocationStep::IfCondition(id) => format!("if-{id}-cond"),
            LocationStep::IfThen(id) => format!("if-{id}-true"),
            LocationStep::IfElse(id) => format!("if-{id}-false"),
            LocationStep::For1(id) => format!("for-{id}-1"),
            LocationStep::For2(id) => format!("for-{id}-2"),
            LocationStep::For3(id) => format!("for-{id}-3"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Location {
    stack: Vec<LocationStep>,
}

impl Location {
    pub fn full(&self) -> String {
        self.stack.iter().fold(String::from(""), |x, y| -> String {
            let contribution = y.full_name_contribution();
            format!("{x}_{contribution}")
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LocationBuilder {
    pub elements: Location,
    block_counter: Counter,
    code_counter: Counter,
    if_counter: Counter,
    for_counter: Counter,
}

impl LocationBuilder {
    pub fn new() -> Self {
        Self {
            elements: Location { stack: [].to_vec() },
            block_counter: Counter::new(),
            code_counter: Counter::new(),
            if_counter: Counter::new(),
            for_counter: Counter::new(),
        }
    }

    pub fn pop(&mut self) {
        self.elements.stack.pop();
    }

    pub fn enter_block(&mut self) {
        self.elements
            .stack
            .push(LocationStep::Block(u32::from(self.block_counter)));
        self.block_counter.increment();
    }
    pub fn enter_function(&mut self, ident: String) {
        self.elements.stack.push(LocationStep::Function(ident));
        self.block_counter.increment();
    }

    pub fn enter_code(&mut self) {
        self.elements
            .stack
            .push(LocationStep::Code(u32::from(self.code_counter)));
        self.code_counter.increment();
    }
    pub fn enter_object(&mut self, identifier: &str) {
        self.elements
            .stack
            .push(LocationStep::Module(String::from(identifier)));
    }

    pub fn enter_if_cond(&mut self) {
        self.elements
            .stack
            .push(LocationStep::IfCondition(u32::from(self.if_counter)));
        self.if_counter.increment();
    }
}
