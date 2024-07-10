#![allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct OPERATORS_S {
    pub ADD: char,
    pub SUB: char,
    pub MUL: char,
    pub EQL: char,
    pub NOT: char,
    pub MOD: char,
    pub DIV: char,
}

pub const OPERATORS: OPERATORS_S = OPERATORS_S {
    ADD: '+',
    SUB: '-',
    MUL: '*',
    EQL: '=',
    NOT: '!',
    MOD: '%',
    DIV: '/',
};