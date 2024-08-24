//!
//! Printers for all YUL AST node types
//!

use crate::util::printer::print_list_comma_separated;
use crate::util::printer::IPrinter;
use crate::yul::parser::statement::assignment::Assignment;
use crate::yul::parser::statement::block::Block;
use crate::yul::parser::statement::code::Code;
use crate::yul::parser::statement::expression::function_call::name::Name;
use crate::yul::parser::statement::expression::function_call::FunctionCall;
use crate::yul::parser::statement::expression::literal::Literal;
use crate::yul::parser::statement::expression::Expression;
use crate::yul::parser::statement::for_loop::ForLoop;
use crate::yul::parser::statement::function_definition::FunctionDefinition;
use crate::yul::parser::statement::if_conditional::IfConditional;
use crate::yul::parser::statement::object::Object;
use crate::yul::parser::statement::switch::Switch;
use crate::yul::parser::statement::variable_declaration::VariableDeclaration;
use crate::yul::parser::statement::Statement;
use crate::yul::visitor::Visitor;

impl<T> Visitor for T
where
    T: IPrinter,
{
    fn visit_object(&mut self, obj: &Object) {
        self.print("object \"");
        self.print(obj.identifier.as_str());
        self.println("\" {");
        self.increase_indent();
        self.visit_code(&obj.code);
        self.println("");
        if let Some(inner) = &obj.inner_object {
            self.visit_object(inner)
        }
        self.println("}");
        self.decrease_indent();
    }

    fn visit_code(&mut self, code: &Code) {
        self.print("code ");
        self.visit_block(&code.block);
    }

    fn visit_switch(&mut self, s: &Switch) {
        self.print("switch ");
        self.visit_expression(&s.expression);
        self.println("");
        for clause in s.cases.iter() {
            self.print("case ");
            self.visit_literal(&clause.literal);
            self.print("   ");
            self.visit_block(&clause.block);
            self.println("");
        }
        if let Some(block) = &s.default {
            self.print("default");
            self.print("   ");
            self.visit_block(block);
            self.println("");
        }
    }

    fn visit_for_loop(&mut self, def: &ForLoop) {
        self.print("for ");
        self.visit_block(&def.initializer);
        self.visit_expression(&def.condition);
        self.visit_block(&def.finalizer);
        self.println("");
        self.visit_block(&def.body);
        self.println("");
    }

    fn visit_variable_declaration(&mut self, def: &VariableDeclaration) {
        self.print("let ");
        print_list_comma_separated(def.bindings.iter().map(|b| b.inner.as_str()), self);
        if let Some(expr) = &def.expression {
            self.print(" := ");
            self.visit_expression(expr);
        }
    }

    fn visit_function_definition(&mut self, def: &FunctionDefinition) {
        let identifier: &str = def.identifier.as_str();
        self.print(format!("function {identifier}(").as_str());
        let arguments = def.arguments.iter().map(|a| a.inner.as_str());
        print_list_comma_separated(arguments, self);
        self.print(") -> ");
        let result_identifiers = def.result.iter().map(|r| r.inner.as_str());
        print_list_comma_separated(result_identifiers, self);
        self.print(" ");
        self.visit_block(&def.body);
        self.println("");
    }

    fn visit_name(&mut self, name: &Name) {
        self.print(&name_identifier(name));
    }

    fn visit_function_call(&mut self, call: &FunctionCall) {
        self.visit_name(&call.name);
        self.print("(");
        for (idx, a) in call.arguments.iter().enumerate() {
            if idx > 0 {
                self.print(", ")
            }
            self.visit_expression(a);
        }
        self.print(")");
    }

    fn visit_if_conditional(&mut self, if_conditional: &IfConditional) {
        self.print("if ");
        self.visit_expression(&if_conditional.condition);
        self.print(" ");
        self.visit_block(&if_conditional.block);
        self.println("");
    }

    fn visit_literal(&mut self, lit: &Literal) {
        let inner: &crate::yul::lexer::token::lexeme::literal::Literal = &lit.inner;
        self.print(format!("{inner}").as_str());
    }

    fn visit_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::FunctionCall(fc) => self.visit_function_call(fc),
            Expression::Identifier(i) => self.print(i.inner.as_str()),
            Expression::Literal(l) => self.visit_literal(l),
        }
    }
    fn visit_assignment(&mut self, assignment: &Assignment) {
        for (idx, a) in assignment.bindings.iter().enumerate() {
            if idx > 0 {
                self.print(", ")
            }
            self.print(a.inner.as_str());
        }
        self.print(" := ");
        self.visit_expression(&assignment.initializer);
    }

    fn visit_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Object(o) => self.visit_object(o),
            Statement::Code(c) => self.visit_code(c),
            Statement::Block(b) => self.visit_block(b),
            Statement::Expression(e) => self.visit_expression(e),
            Statement::FunctionDefinition(fd) => self.visit_function_definition(fd),
            Statement::VariableDeclaration(vd) => self.visit_variable_declaration(vd),
            Statement::Assignment(a) => self.visit_assignment(a),
            Statement::IfConditional(i) => self.visit_if_conditional(i),
            Statement::Switch(s) => self.visit_switch(s),
            Statement::ForLoop(f) => self.visit_for_loop(f),
            Statement::Continue(_) => self.print("continue"),
            Statement::Break(_) => self.print("break"),
            Statement::Leave(_) => self.print("leave"),
        }
    }

    fn visit_block(&mut self, block: &Block) {
        if block.statements.is_empty() {
            self.print(" { }");
            return;
        }

        if block.statements.len() == 1 {
            self.print("{ ");
            self.visit_statement(block.statements.first().unwrap());
            self.print(" }");
            return;
        }
        self.println(" {");
        self.increase_indent();
        for s in block.statements.iter() {
            self.visit_statement(s);
            self.println("");
        }
        self.println(" }");
        self.decrease_indent();
    }
}

/// Shows how an instance of [`Name`] is displayed in YUL code.
pub fn name_identifier(name: &Name) -> String {
    if let Name::Verbatim {
        input_size,
        output_size,
    } = name
    {
        format!("verbatim_{input_size}i_{output_size}o")
    } else if let Name::UserDefined(inner) = name {
        inner.to_string()
    } else {
        String::from(match name {
            Name::Add => "add",
            Name::Sub => "sub",
            Name::Mul => "mul",
            Name::Div => "div",
            Name::Mod => "mod",
            Name::Sdiv => "sdiv",
            Name::Smod => "smod",

            Name::Lt => "lt",
            Name::Gt => "gt",
            Name::Eq => "eq",
            Name::IsZero => "iszero",
            Name::Slt => "slt",
            Name::Sgt => "sgt",

            Name::Or => "or",
            Name::Xor => "xor",
            Name::Not => "not",
            Name::And => "and",
            Name::Shl => "shl",
            Name::Shr => "shr",
            Name::Sar => "sar",
            Name::Byte => "byte",
            Name::Pop => "pop",

            Name::AddMod => "addmod",
            Name::MulMod => "mulmod",
            Name::Exp => "exp",
            Name::SignExtend => "signextend",

            Name::Keccak256 => "keccak256",

            Name::MLoad => "mload",
            Name::MStore => "mstore",
            Name::MStore8 => "mstore8",
            Name::MCopy => "mcopy",

            Name::SLoad => "sload",
            Name::SStore => "sstore",
            Name::TLoad => "tload",
            Name::TStore => "tstore",
            Name::LoadImmutable => "loadimmutable",
            Name::SetImmutable => "setimmutable",

            Name::CallDataLoad => "calldataload",
            Name::CallDataSize => "calldatasize",
            Name::CallDataCopy => "calldatacopy",
            Name::CodeSize => "codesize",
            Name::CodeCopy => "codecopy",
            Name::ReturnDataSize => "returndatasize",
            Name::ReturnDataCopy => "returndatacopy",
            Name::ExtCodeSize => "extcodesize",
            Name::ExtCodeHash => "extcodehash",

            Name::Return => "return",
            Name::Revert => "revert",

            Name::Log0 => "log0",
            Name::Log1 => "log1",
            Name::Log2 => "log2",
            Name::Log3 => "log3",
            Name::Log4 => "log4",

            Name::Call => "call",
            Name::DelegateCall => "delegatecall",
            Name::StaticCall => "staticcall",

            Name::Create => "create",
            Name::Create2 => "create2",
            Name::ZkCreate => "$zk_create",
            Name::ZkCreate2 => "$zk_create2",
            Name::DataSize => "datasize",
            Name::DataOffset => "dataoffset",
            Name::DataCopy => "datacopy",

            Name::Stop => "stop",
            Name::Invalid => "invalid",

            Name::LinkerSymbol => "linkersymbol",
            Name::MemoryGuard => "memoryguard",

            Name::Address => "address",
            Name::Caller => "caller",

            Name::CallValue => "callvalue",
            Name::Gas => "gas",
            Name::Balance => "balance",
            Name::SelfBalance => "selfbalance",

            Name::GasLimit => "gaslimit",
            Name::GasPrice => "gasprice",
            Name::Origin => "origin",
            Name::ChainId => "chainid",
            Name::Timestamp => "timestamp",
            Name::Number => "number",
            Name::BlockHash => "blockhash",
            Name::BlobHash => "blobhash",
            Name::Difficulty => "difficulty",
            Name::Prevrandao => "prevrandao",
            Name::CoinBase => "coinbase",
            Name::BaseFee => "basefee",
            Name::BlobBaseFee => "blobbasefee",
            Name::MSize => "msize",

            Name::CallCode => "callcode",
            Name::Pc => "pc",
            Name::ExtCodeCopy => "extcodecopy",
            Name::SelfDestruct => "selfdestruct",

            Name::ZkToL1 => "$zk_to_l1",
            Name::ZkCodeSource => "$zk_code_source",
            Name::ZkPrecompile => "$zk_precompile",
            Name::ZkMeta => "$zk_meta",
            Name::ZkSetContextU128 => "$zk_set_context_u128",
            Name::ZkSetPubdataPrice => "$zk_set_pubdata_price",
            Name::ZkIncrementTxCounter => "$zk_increment_tx_counter",
            Name::ZkEventInitialize => "$zk_event_initialize",
            Name::ZkEventWrite => "$zk_event_write",

            Name::ZkMimicCall => "$zk_mimic_call",
            Name::ZkSystemMimicCall => "$zk_system_mimic_call",
            Name::ZkMimicCallByRef => "$zk_mimic_call_byref",
            Name::ZkSystemMimicCallByRef => "$zk_system_mimic_call_byref",
            Name::ZkRawCall => "$zk_raw_call",
            Name::ZkRawCallByRef => "$zk_raw_call_byref",
            Name::ZkSystemCall => "$zk_system_call",
            Name::ZkSystemCallByRef => "$zk_system_call_byref",
            Name::ZkStaticRawCall => "$zk_static_raw_call",
            Name::ZkStaticRawCallByRef => "$zk_static_raw_call_byref",
            Name::ZkStaticSystemCall => "$zk_static_system_call",
            Name::ZkStaticSystemCallByRef => "$zk_static_system_call_byref",
            Name::ZkDelegateRawCall => "$zk_delegate_raw_call",
            Name::ZkDelegateRawCallByRef => "$zk_delegate_raw_call_byref",
            Name::ZkDelegateSystemCall => "$zk_delegate_system_call",
            Name::ZkDelegateSystemCallByRef => "$zk_delegate_system_call_byref",

            Name::ZkLoadCalldataIntoActivePtr => "$zk_load_calldata_into_active_ptr",
            Name::ZkLoadReturndataIntoActivePtr => "$zk_load_returndata_into_active_ptr",
            Name::ZkPtrAddIntoActive => "$zk_ptr_add_into_active",
            Name::ZkPtrShrinkIntoActive => "$zk_ptr_shrink_into_active",
            Name::ZkPtrPackIntoActive => "$zk_ptr_pack_into_active",

            Name::ZkMultiplicationHigh => "$zk_multiplication_high",

            Name::ZkGlobalLoad => "$zk_global_load",
            Name::ZkGlobalExtraAbiData => "$zk_global_extra_abi_data",
            Name::ZkGlobalStore => "$zk_global_store",

            Name::UserDefined(_) | Name::Verbatim { .. } => {
                unreachable!()
            }
        })
    }
}
