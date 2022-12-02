#![allow(dead_code)]
pub const OP_CONSTANT: u8       = 0;
pub const OP_RETURN: u8         = 1;
pub const OP_NEGATE: u8         = 2;
pub const OP_ADD:u8             = 3;
pub const OP_SUBTRACT:u8        = 4;
pub const OP_MULTIPLY:u8        = 5;
pub const OP_DIVIDE:u8          = 6;
pub const OP_NOT:u8             = 7;
pub const OP_EQUAL:u8           = 8;
pub const OP_OR:u8              = 9;
pub const OP_AND:u8             = 10;
pub const OP_GT:u8              = 11;
pub const OP_LT:u8              = 12;
pub const OP_GE:u8              = 13;
pub const OP_LE:u8              = 14;
pub const OP_NE :u8             = 15;
pub const OP_ASSIGN:u8          = 16;
pub const OP_PRINT:u8           = 17;


pub fn is_binary_op(opcode: u8) -> bool {
    match opcode {
        OP_ADD | OP_SUBTRACT | OP_MULTIPLY | OP_DIVIDE | OP_AND | OP_OR | OP_EQUAL | OP_GT | OP_LT | OP_GE | OP_LE | OP_NE => true,
        _ => false
    }
}

pub fn is_unary_op(opcode: u8) -> bool {
    match opcode {
        OP_NEGATE | OP_NOT | OP_PRINT => true,
        _ => false
    }
}

pub fn opcode2string(opcode: u8) -> String {
    match opcode {
        OP_CONSTANT => "OP_CONSTANT",
        OP_RETURN => "OP_RETURN",
        OP_NEGATE => "OP_NEGATE",
        OP_ADD => "OP_ADD",
        OP_SUBTRACT => "OP_SUBTRACT",
        OP_MULTIPLY => "OP_MULTIPLY",
        OP_DIVIDE => "OP_DIVIDE",
        OP_NOT => "OP_NOT",
        OP_EQUAL => "OP_EQUAL",
        OP_OR => "OP_OR",
        OP_AND => "OP_AND",
        OP_GT => "OP_GT",
        OP_LT => "OP_LT",
        OP_GE => "OP_GE",
        OP_LE => "OP_LE",
        OP_NE => "OP_NE",
        OP_ASSIGN => "OP_ASSIGN",
        OP_PRINT => "OP_PRINT",
        _ => "UNKNOWN"
    }.to_string()
}


