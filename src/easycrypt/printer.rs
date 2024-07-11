//!
//! EasyCrypt AST pretty printer
//!

use std::collections::HashMap;

use super::syntax::definition::Definition;
use super::syntax::expression::binary::BinaryOpType;
use super::syntax::expression::call::FunctionCall;
use super::syntax::expression::unary::UnaryOpType;
use super::syntax::expression::Expression;
use super::syntax::function::name::FunctionName;
use super::syntax::function::Function;
use super::syntax::literal::integer::IntegerLiteral;
use super::syntax::literal::Literal;
use super::syntax::module::definition::TopDefinition;
use super::syntax::module::Module;
use super::syntax::proc::name::ProcName;
use super::syntax::proc::Proc;
use super::syntax::r#type::Type;
use super::syntax::reference::Reference;
use super::syntax::signature::Signature;
use super::syntax::signature::SignatureKind;
use super::syntax::statement::block::Block;
use super::syntax::statement::call::ProcCall;
use super::syntax::statement::if_conditional::IfConditional;
use super::syntax::statement::Statement;

use super::syntax::statement::while_loop::WhileLoop;
use super::visitor::Visitor;
use crate::util::printer::IPrinter;
use crate::yul::path::full_name::FullName;
use crate::yul::path::Path;
use crate::yul::visitor::IMPLICIT_CODE_FUNCTION_NAME;
use crate::WritePrinter;

// FIXME dirty hack
fn resolve<'a>(names: impl Iterator<Item = &'a FullName>) -> HashMap<String, Vec<FullName>> {
    let mut result: HashMap<String, Vec<FullName>> = HashMap::new();
    for item in names {
        if !result.contains_key(&item.name) {
            result.insert(item.name.clone(), vec![]);
        }
        result.get_mut(&item.name).unwrap().push(item.clone());
    }
    result
}

fn drop_common_prefix(duplicates: &Vec<FullName>) -> Vec<String> {
    let min_length = duplicates
        .iter()
        .zip(duplicates.iter().skip(1))
        .map(|(a, b)| a.path.common_prefix_length(&b.path))
        .min()
        .unwrap();
    duplicates
        .iter()
        .map(|fp| fp.path.suffix(min_length))
        .collect()
}

fn display_names<'a>(names: impl Iterator<Item = &'a FullName>) -> HashMap<FullName, String> {
    let mut result: HashMap<FullName, String> = HashMap::new();
    let resolved: HashMap<String, Vec<FullName>> = resolve(names);

    for (name, resolution_results) in resolved {
        if resolution_results.len() > 1 {
            let without_prefix: Vec<_> = drop_common_prefix(&resolution_results);
            dbg!(&without_prefix);
            for (duplicate, display_name_part) in resolution_results.iter().zip(without_prefix) {
                let name = Path::combine(&display_name_part, &name);
                result.insert(duplicate.clone(), name);
            }
        } else {
            result.insert(resolution_results[0].clone(), name.clone());
        }
    }
    result
}
#[cfg(test)]
mod test {
    use crate::{
        easycrypt::printer::display_names,
        yul::path::{builder::Builder, full_name::FullName, tracker::PathTracker, Path},
    };

    #[test]
    fn test() {
        let mut builder = Builder::new(Path::empty());

        builder.enter_function("T");
        builder.enter_function("F");
        builder.enter_block();
        let path_x = builder.here().clone();
        builder.leave();
        builder.leave();
        builder.enter_function("G");
        builder.enter_block();
        let path_y = builder.here().clone();

        let names = vec![
            FullName {
                name: "x".to_string(),
                path: path_x,
            },
            FullName {
                name: "x".to_string(),
                path: path_y,
            },
        ];
        dbg!(&names);
        //dbg!(drop_common_prefix(&names));
        let disp_names = display_names(names.iter());
        dbg!(disp_names);
    }
}

fn sanitize_identifier(identifier: &str) -> String {
    const KEYWORDS: &[&str] = &["end", "var"];
    let mut result = identifier.replace('$', "_");
    if identifier
        .chars()
        .next()
        .map_or(false, |c| c.is_uppercase())
        || KEYWORDS.contains(&identifier)
    {
        result.insert(0, '_');
    }
    result
}

fn disambiguate_and_sanitize(
    identifier: &str,
    location: Option<&Path>,
    names_disambiguation: &HashMap<FullName, String>,
) -> String {
    let identifier = identifier.to_string();
    if let Some(location) = location {
        let full_name = FullName::new(identifier.clone(), location.clone());
        if let Some(mapped) = names_disambiguation.get(&full_name) {
            sanitize_identifier(mapped)
        } else {
            sanitize_identifier(&identifier)
        }
    } else {
        sanitize_identifier(&identifier)
    }
}
fn statement_followed_by_semicolon(statement: &Statement) -> bool {
    match &statement {
        Statement::Block(_) | Statement::If(_) => false,
        Statement::VarDefinition(_, _)
        | Statement::Expression(_)
        | Statement::EAssignment(_, _)
        | Statement::PAssignment(_, _) => true,
        Statement::Return(_) => true,
        Statement::WhileLoop(_) => false,
        Statement::Pass => true,
    }
}

pub struct ECPrinter {
    printer: WritePrinter,
    names_disambiguation: HashMap<FullName, String>,
}

impl ECPrinter {
    pub fn new<'a>(ordered_names: impl Iterator<Item = &'a FullName>) -> Self {
        let display_names = display_names(ordered_names);
        Self {
            printer: WritePrinter::default(),
            names_disambiguation: display_names,
        }
    }

    pub fn print_all(&mut self, module: &Module) {
        self.visit_module(module)
    }
}

impl IPrinter for ECPrinter {
    fn print(&mut self, s: &str) {
        <WritePrinter as IPrinter>::print(&mut self.printer, s)
    }

    fn println(&mut self, s: &str) {
        <WritePrinter as IPrinter>::println(&mut self.printer, s)
    }

    fn increase_indent(&mut self) {
        <WritePrinter as IPrinter>::increase_indent(&mut self.printer)
    }

    fn decrease_indent(&mut self) {
        <WritePrinter as IPrinter>::decrease_indent(&mut self.printer)
    }
}

impl Visitor for ECPrinter {
    fn visit_binary_op_type(&mut self, op: &BinaryOpType) {
        self.print(match op {
            BinaryOpType::Add => "+",
            BinaryOpType::Sub => "-",
            BinaryOpType::Mul => "*",
            BinaryOpType::Mod => "%%",
            BinaryOpType::And => "/\\",
            BinaryOpType::Or => "\\/",
            BinaryOpType::Xor => "^",
            BinaryOpType::Div => "/",
            BinaryOpType::Eq => "=",
            BinaryOpType::Exp => "**",
        });
    }

    fn visit_block(&mut self, block: &Block) {
        self.increase_indent();
        self.println("{");
        for statement in &block.statements {
            self.visit_statement(statement);
            if statement_followed_by_semicolon(statement) {
                self.print(";");
            }
            self.println("");
        }
        self.println("");
        self.println("}");
        self.decrease_indent()
    }

    fn visit_definition(&mut self, definition: &Definition) {
        self.print(
            disambiguate_and_sanitize(
                &definition.identifier,
                definition.location.clone().as_ref(),
                &self.names_disambiguation,
            )
            .as_str(),
        );
    }

    fn visit_expression(&mut self, expression: &Expression) {
        match expression {
            Expression::Unary(op, expr) => {
                self.print("(");
                self.visit_unary_op_type(op);
                self.print(" ");
                self.visit_expression(expr);
                self.print(")");
            }
            Expression::Binary(op, lhs, rhs) => {
                self.print("(");
                self.visit_expression(lhs);
                self.print(" ");
                self.visit_binary_op_type(op);
                self.print(" ");
                self.visit_expression(rhs);
                self.print(")");
            }
            Expression::ECall(ecall) => self.visit_function_call(ecall),
            Expression::Literal(literal) => self.visit_literal(literal),
            Expression::Reference(reference) => self.visit_reference(reference),
            Expression::Tuple(expressions) => {
                self.print("(");
                for (i, expr) in expressions.iter().enumerate() {
                    if i > 0 {
                        self.print(", ");
                    }
                    self.visit_expression(expr);
                }
                self.print(")")
            }
        }
    }

    fn visit_function(&mut self, function: &Function) {
        self.print("op ");
        self.visit_function_name(&function.name.clone());
        self.visit_signature(&function.signature);
        self.print(" = ");
        self.visit_expression(&function.body);
        self.println(".");
    }

    fn visit_function_call(&mut self, call: &FunctionCall) {
        let FunctionCall { target, arguments } = call;
        if !arguments.is_empty() {
            self.print("(");
            self.visit_function_name(target);
            for arg in arguments {
                self.print(" ");
                self.visit_expression(arg);
            }
            self.print(")");
        } else {
            self.visit_function_name(target);
        }
    }

    fn visit_function_name(&mut self, name: &FunctionName) {
        if let Some(module) = &name.module {
            self.print(&module);
            self.print(".");
        }
        match name {
            FunctionName { name, yul_name, .. } => {
                let full_name = yul_name.clone();
                let sanitized_name = disambiguate_and_sanitize(
                    &name.to_string(),
                    full_name.map(|name| name.path).as_ref(),
                    &self.names_disambiguation,
                );
                self.print(&sanitized_name);
            }
        }
    }

    fn visit_integer_literal(&mut self, int_literal: &IntegerLiteral) {
        let inner = match int_literal {
            IntegerLiteral::Decimal { inner } => inner,
        };
        self.print(inner);
    }

    fn visit_literal(&mut self, literal: &Literal) {
        match literal {
            Literal::String(s) => {
                self.print("STRING (*");
                self.print(s.as_str());
                self.print("*)")
            }
            Literal::Int(int_literal) => self.visit_integer_literal(int_literal),
            Literal::Bool(value) => self.print(format!("{value}").as_str()),
        }
    }
    fn visit_module(&mut self, module: &Module) {
        const NAME_ANONYMOUS_MODULE: &str = "ANONYMOUS";
        let module_name = module
            .name
            .clone()
            .unwrap_or(String::from(NAME_ANONYMOUS_MODULE));

        self.println(format!("(* Begin {} *)", module_name).as_str());

        for full_name in &module.dependency_order {
            let def = module
                .definitions
                .get(&Reference::from(full_name))
                .expect(format!("Printer cannot find reference {:?}", full_name).as_str());
            if def.is_fun_def() {
                self.visit_module_definition(def);
                self.println("")
            }
        }

        self.print("module ");
        self.print(&module_name);
        self.println(" = {");
        self.increase_indent();
        for full_name in &module.dependency_order {
            let def = module.definitions.get(&Reference::from(full_name)).unwrap();
            if def.is_proc_def() {
                self.visit_module_definition(def);
                self.println("")
            }
        }

        self.println("");
        self.println("}.");
        self.decrease_indent();
        self.println(format!("(* End {} *)", module_name).as_str());
    }

    fn visit_module_definition(&mut self, definition: &TopDefinition) {
        match definition {
            TopDefinition::Proc(proc_def) => self.visit_proc(proc_def),
            TopDefinition::Function(fun_def) => self.visit_function(fun_def),
        }
    }

    fn visit_proc_call(&mut self, call: &ProcCall) {
        let ProcCall { target, arguments } = call;
        self.visit_proc_name(target);
        self.print("(");
        for (i, arg) in arguments.iter().enumerate() {
            if i > 0 {
                self.print(", ")
            }
            self.visit_expression(arg);
        }
        self.print(")");
    }

    fn visit_proc(&mut self, proc: &Proc) {
        // FIXME temp
        if proc.name.name == IMPLICIT_CODE_FUNCTION_NAME {
            return;
        }
        self.print("proc ");
        self.visit_proc_name(&proc.name.clone());
        self.visit_signature(&proc.signature);
        self.println(" = {");
        self.increase_indent();
        if !proc.locals.is_empty() {
            self.print("var ");

            for (i, local) in proc.locals.iter().enumerate() {
                if i > 0 {
                    self.print(", ");
                }
                self.visit_definition(local);
            }
            self.println(";");
        }

        for statement in &proc.body.statements {
            self.visit_statement(statement);
            if statement_followed_by_semicolon(statement) {
                self.print(";");
            }
            self.println("");
        }
        self.println("}");
        self.decrease_indent();
    }

    fn visit_proc_name(&mut self, name: &ProcName) {
        self.visit_function_name(name)
    }

    fn visit_reference(&mut self, reference: &Reference) {
        let location = reference.location.as_ref();
        let disambiguate_and_sanitize = &disambiguate_and_sanitize(
            reference.identifier.as_str(),
            location,
            &self.names_disambiguation,
        );
        self.print(disambiguate_and_sanitize.as_str())
    }

    fn visit_signature(&mut self, signature: &Signature) {
        let Signature {
            formal_parameters,
            return_type,
            kind,
        } = signature;
        if kind != &SignatureKind::Function || !formal_parameters.is_empty() {
            self.print("(");

            for (i, (param, ty)) in formal_parameters.iter().enumerate() {
                if i > 0 {
                    self.print(", ")
                }
                let name = sanitize_identifier(&param.identifier);
                self.print(format!("{} : {}", name, ty).as_str());
            }
            self.print(")");
        }
        self.print(format!(": {}", return_type).as_str());
    }

    fn visit_statement(&mut self, statement: &Statement) {
        fn print_lhs_references<T>(s: &mut T, references: &[Reference])
        where
            T: IPrinter + Visitor,
        {
            match references.len() {
                0 => (),
                1 => {
                    s.visit_reference(&references[0]);
                }
                _ => {
                    s.print("(");
                    for (i, r) in references.iter().enumerate() {
                        if i > 0 {
                            s.print(",")
                        }
                        s.visit_reference(r);
                    }
                    s.print(")");
                }
            }
        }

        match statement {
            Statement::VarDefinition(_, _) => todo!(),
            Statement::Expression(expression) => self.visit_expression(expression),
            Statement::Block(block) => self.visit_block(block),
            Statement::If(if_conditional) => self.visit_if_conditional(if_conditional),
            Statement::EAssignment(refs, rhs) => {
                print_lhs_references(self, refs);
                if !refs.is_empty() {
                    self.print(" <- ");
                }
                self.visit_expression(rhs);
            }
            Statement::PAssignment(refs, rhs) => {
                print_lhs_references(self, refs);
                if !refs.is_empty() {
                    self.print(" <@ ");
                }
                self.visit_proc_call(rhs);
            }

            Statement::Return(e) => {
                self.print("return ");
                self.visit_expression(e);
            }
            Statement::Pass => todo!(),
            Statement::WhileLoop(while_loop) => self.visit_while_loop(while_loop),
        }
    }

    fn visit_type(&mut self, r#type: &Type) {
        self.print(format!("{}", r#type).as_str())
    }

    fn visit_unary_op_type(&mut self, op: &UnaryOpType) {
        self.print(match op {
            UnaryOpType::Neg => "-",
            UnaryOpType::Not => "!",
        })
    }

    fn visit_if_conditional(&mut self, conditional: &IfConditional) {
        let IfConditional { condition, yes, no } = conditional;

        self.print("if (");
        self.visit_expression(condition);
        self.println(")");

        if !yes.is_block() {
            self.print(" { ")
        }
        self.visit_statement(yes);
        if !yes.is_block() {
            self.println(" } ")
        }
        if let Some(no) = no {
            self.println("");
            self.print("else ");
            self.visit_statement(no);
        }
    }

    fn visit_while_loop(&mut self, while_loop: &super::syntax::statement::while_loop::WhileLoop) {
        let WhileLoop { condition, body } = while_loop;
        self.print("while (");

        self.visit_expression(condition);
        self.println(")");

        if !body.is_block() {
            self.print(" { ")
        }
        self.visit_statement(body);
        if !body.is_block() {
            self.println(" } ")
        }
    }
}
