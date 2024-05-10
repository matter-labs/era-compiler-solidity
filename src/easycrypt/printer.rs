//!
//! EasyCrypt AST pretty printer
//!

use super::{
    syntax::{
        BinaryOpType, Block, Definition, Expression, Function, FunctionCall, FunctionName,
        IntegerLiteral, Literal, Module, ModuleDefinition, Proc, ProcCall, ProcName, Reference,
        Signature, SignatureKind, Statement, Type, UnaryOpType,
    },
    visitor::Visitor,
};
use crate::util::printer::IPrinter;

impl<T: IPrinter> Visitor for T {
    fn visit_definition(&mut self, definition: &Definition) {
        //self.print(definition.location.full().as_str());
        self.print(definition.identifier.as_str());
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
        if kind != &SignatureKind::Function || formal_parameters.len() != 0 {
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

    fn visit_function(&mut self, function: &Function) {
        self.print("op ");
        self.visit_function_name(&function.name);
        self.visit_signature(&function.signature);
        self.print(" = ");
        self.visit_expression(&function.body);
        self.println(".");
    }

    fn visit_proc_name(&mut self, name: &ProcName) {
        self.print(name.to_string().as_str());
    }

    fn visit_function_name(&mut self, name: &FunctionName) {
        self.print(name.to_string().as_str());
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
                self.visit_definition(&local);
            }
            self.println(";");
        }

        self.visit_block(&proc.body);
        self.println("}");
        self.decrease_indent();
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

    fn visit_function_call(&mut self, call: &FunctionCall) {
        let FunctionCall { target, arguments } = call;
        self.visit_function_name(&target);
        if arguments.len() != 0 {
            self.print("(");
            for (i, arg) in arguments.iter().enumerate() {
                if i > 0 {
                    self.print(", ")
                }
                self.visit_expression(&*arg);
            }
            self.print(")");
        }
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

    fn visit_module(&mut self, module: &Module) {
        const NAME_ANONYMOUS_MODULE: &'static str = "ANONYMOUS";
        let module_name = module
            .name
            .clone()
            .unwrap_or(String::from(NAME_ANONYMOUS_MODULE));

        self.println(format!("(* Begin {} *)", module_name).as_str());

        for (_, def) in &module.definitions {
            if def.is_fun() {
                self.visit_module_definition(&def);
                self.println("")
            }
        }

        self.print("module ");
        self.print(&module_name);
        self.println(" = {");
        self.increase_indent();
        for (_, def) in &module.definitions {
            if def.is_proc() {
                self.visit_module_definition(&def);
                self.println("")
            }
        }
        self.println("");
        self.println("}");
        self.decrease_indent();
        self.println(format!("(* End {} *)", module_name).as_str());
    }

    fn visit_integer_literal(&mut self, int_literal: &IntegerLiteral) {
        match int_literal {
            IntegerLiteral::Decimal { inner } => self.print(&inner),
            IntegerLiteral::Hexadecimal { inner } => {
                self.print("0x");
                self.print(&inner)
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
            Literal::Int(int_literal) => self.visit_integer_literal(&int_literal),
            Literal::Bool(value) => self.print(format!("{value}").as_str()),
        }
    }

    fn visit_type(&mut self, r#type: &Type) {
        self.print(format!("{}", r#type).as_str())
    }

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

    fn visit_unary_op_type(&mut self, op: &UnaryOpType) {
        todo!()
    }

    fn visit_expression(&mut self, expression: &Expression) {
        match expression {
            Expression::Unary(_, _) => todo!(),
            Expression::Binary(op, lhs, rhs) => {
                self.visit_binary_op_type(&op);
                self.print("(");
                self.visit_expression(&*lhs);
                self.print(",");
                self.visit_expression(&*rhs);
                self.print(")");
            }
            Expression::ECall(ecall) => self.visit_function_call(&ecall),
            Expression::Literal(literal) => self.visit_literal(literal),
            Expression::Reference(reference) => self.visit_reference(reference),
            Expression::Tuple(expressions) => {
                self.print("(");
                for (i, expr) in expressions.iter().enumerate() {
                    self.visit_expression(expr);
                    if i > 0 {
                        self.print(", ");
                    }
                }
                self.print(")")
            }
        }
    }

    fn visit_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::VarDefinition(_, _) => todo!(),
            Statement::Expression(expression) => self.visit_expression(expression),
            Statement::Block(block) => {
                block.statements.iter().for_each(|s| {
                    self.visit_statement(&s);
                    self.println("")
                });
            }
            Statement::If(_, _, _) => todo!(),
            Statement::EAssignment(refs, rhs) => {
                for (i, r) in (*refs).iter().enumerate() {
                    self.visit_reference(&r);
                    if i > 0 {
                        self.print(",")
                    }
                }
                self.print(" <- ");
                self.visit_expression(&*rhs);
            }
            Statement::PAssignment(refs, rhs) => {
                for (i, r) in (*refs).iter().enumerate() {
                    self.visit_reference(&r);
                    if i > 0 {
                        self.print(",")
                    }
                }
                self.print(" <@ ");
                self.visit_proc_call(&*rhs);
            }

            Statement::Return(e) => {
                self.print("return ");
                self.visit_expression(e);
            }
            Statement::Pass => todo!(),
            Statement::While(_, _) => todo!(),
        }
    }

    fn visit_module_definition(&mut self, definition: &ModuleDefinition) {
        match definition {
            ModuleDefinition::ProcDef(proc_def) => self.visit_proc(&proc_def),
            ModuleDefinition::FunDef(fun_def) => self.visit_function(&fun_def),
        }
    }
}
