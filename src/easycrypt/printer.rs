//!
//! EasyCrypt AST pretty printer
//!

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

use super::visitor::Visitor;
use crate::util::printer::IPrinter;

impl<T: IPrinter> Visitor for T {
    fn visit_binary_op_type(&mut self, op: &BinaryOpType) {
        self.print(match op {
            BinaryOpType::Add => "+",
            BinaryOpType::Sub => "-",
            BinaryOpType::Mul => "*",
            BinaryOpType::Mod => "%",
            BinaryOpType::And => "/\\",
            BinaryOpType::Or => "\\/",
            BinaryOpType::Xor => "^",
            BinaryOpType::Div => "/",
            BinaryOpType::Eq => "=",
            BinaryOpType::Shl => "<<",
            BinaryOpType::Shr => ">>",
            BinaryOpType::Exp => "**",
        });
    }

    fn visit_block(&mut self, block: &Block) {
        self.increase_indent();
        self.println("{");
        for statement in &block.statements {
            self.visit_statement(statement);
            self.println(";");
        }
        self.println("");
        self.println("}");
        self.decrease_indent()
    }

    fn visit_definition(&mut self, definition: &Definition) {
        //self.print(definition.location.full().as_str());
        self.print(definition.identifier.as_str());
    }

    fn visit_expression(&mut self, expression: &Expression) {
        match expression {
            Expression::Unary(op, expr) => {
                self.visit_unary_op_type(op);
                self.visit_expression(expr);
            }
            Expression::Binary(op, lhs, rhs) => {
                self.visit_expression(lhs);
                self.visit_binary_op_type(op);
                self.visit_expression(rhs);
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
        self.visit_function_name(&function.name);
        self.visit_signature(&function.signature);
        self.print(" = ");
        self.visit_expression(&function.body);
        self.println(".");
    }

    fn visit_function_call(&mut self, call: &FunctionCall) {
        let FunctionCall { target, arguments } = call;
        self.visit_function_name(target);
        if !arguments.is_empty() {
            self.print("(");
            for (i, arg) in arguments.iter().enumerate() {
                if i > 0 {
                    self.print(", ")
                }
                self.visit_expression(arg);
            }
            self.print(")");
        }
    }

    fn visit_function_name(&mut self, name: &FunctionName) {
        self.print(name.to_string().as_str());
    }

    fn visit_integer_literal(&mut self, int_literal: &IntegerLiteral) {
        match int_literal {
            IntegerLiteral::Decimal { inner } => self.print(inner),
            IntegerLiteral::Hexadecimal { inner } => {
                self.print("0x");
                self.print(inner)
            }
        }
    }

    fn visit_literal(&mut self, literal: &Literal) {
        match literal {
            Literal::String(s) => {
                self.print("\"");
                self.print(s.as_str());
                self.print("\"")
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

        for def in module.definitions.values() {
            if def.is_fun_def() {
                self.visit_module_definition(def);
                self.println("")
            }
        }

        self.print("module ");
        self.print(&module_name);
        self.println(" = {");
        self.increase_indent();
        for def in module.definitions.values() {
            if def.is_proc_def() {
                self.visit_module_definition(def);
                self.println("")
            }
        }
        self.println("");
        self.println("}");
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
        for arg in arguments {
            self.visit_expression(arg);
        }
        self.print(")");
    }

    fn visit_proc(&mut self, proc: &Proc) {
        self.print("proc ");
        self.visit_proc_name(&proc.name);
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

        self.visit_block(&proc.body);
        self.println("}");
        self.decrease_indent();
    }

    fn visit_proc_name(&mut self, name: &ProcName) {
        self.print(name.to_string().as_str());
    }

    fn visit_reference(&mut self, reference: &Reference) {
        //self.print(reference.location.full().as_str());
        self.print(reference.identifier.as_str());
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
                self.print(format!("{} : {}", param.identifier, ty).as_str());
            }
            self.print(")");
        }
        self.print(format!(": {}", return_type).as_str());
    }

    fn visit_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::VarDefinition(_, _) => todo!(),
            Statement::Expression(expression) => self.visit_expression(expression),
            Statement::Block(block) => self.visit_block(block),
            Statement::If(if_conditional) => self.visit_if_conditional(if_conditional),
            Statement::EAssignment(refs, rhs) => {
                for (i, r) in (*refs).iter().enumerate() {
                    self.visit_reference(r);
                    if i > 0 {
                        self.print(",")
                    }
                }
                self.print(" <- ");
                self.visit_expression(rhs);
            }
            Statement::PAssignment(refs, rhs) => {
                for (i, r) in (*refs).iter().enumerate() {
                    self.visit_reference(r);
                    if i > 0 {
                        self.print(",")
                    }
                }
                self.print(" <@ ");
                self.visit_proc_call(rhs);
            }

            Statement::Return(e) => {
                self.print("return ");
                self.visit_expression(e);
            }
            Statement::Pass => todo!(),
            Statement::While(_, _) => todo!(),
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
}