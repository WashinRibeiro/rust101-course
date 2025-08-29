//! # Os tipos são seus amigos mano
//!
//! Rust é uma linguagem de programação com tipagem estática e forte, o que significa que:
//! - Tipagem estática: O tipo de cada variável é determinado em tempo de compilação e não muda.
//! Uma variável declarada com um tipo específico não pode ser reatribuída a um valor de outro tipo sem uma conversão explícita.
//! - Tipagem forte: Rust é rigoroso com as operações entre tipos. Não é possível, por exemplo:
//! somar diretamente um número e uma string sem converter explicitamente um deles para o tipo compatível do outro.
//!
//! O compilador de Rust é inteligente o suficiente para inferir o tipo de muitas variáveis,
//! o que significa que nem sempre é necessário especificar o tipo explicitamente.
//!
//! Nesta aula, focaremos nos tipos primitivos, que são os blocos de construção básicos para a criação de programas em Rust.

#[test]
fn test_integer_types() {
    let small_int: i8 = -128;
    let big_uint: u128 = 340282366920938463463374607431768211455;
    assert_eq!(small_int, -128);
    assert_eq!(big_uint, 340282366920938463463374607431768211455);
}

#[test]
fn test_float_types() {
    let float_num: f32 = 3.14;
    let double_num: f64 = 2.71828;
    assert!(float_num - 3.14 < f32::EPSILON);
    assert!(double_num - 2.71828 < f64::EPSILON);
}

#[test]
fn test_boolean_type() {
    let is_rust_awesome: bool = true;
    assert_eq!(is_rust_awesome, true);
}

#[test]
fn test_char_type() {
    let letter: char = 'R';
    assert_eq!(letter, 'R');
}

#[test]
fn test_tuple_type() {
    let tuple: (i32, f64, char) = (500, 6.4, '🚀');
    assert_eq!(tuple, (500, 6.4, '🚀'));
}

#[test]
fn test_array_type() {
    let array: [i32; 3] = [1, 2, 3];
    assert_eq!(array, [1, 2, 3]);
}

#[test]
fn test_size_of_integer_types() {
    assert_eq!(std::mem::size_of::<i8>(), 1);
    assert_eq!(std::mem::size_of::<u128>(), 16);
}

#[test]
fn test_size_of_float_types() {
    assert_eq!(std::mem::size_of::<f32>(), 4);
    assert_eq!(std::mem::size_of::<f64>(), 8);
}

#[test]
fn test_size_of_bool_type() {
    assert_eq!(std::mem::size_of::<bool>(), 1);
}

#[test]
fn test_size_of_char_type() {
    assert_eq!(std::mem::size_of::<char>(), 4);
}

#[test]
fn test_size_of_composite_types_using_padding() {
    assert_eq!(std::mem::size_of::<(i32, f64)>(), 16);
    assert_eq!(std::mem::size_of::<(char, i32, i128)>(), 32);
    assert_eq!(std::mem::size_of::<[i32; 3]>(), 12);
}

#[test]
fn broken_test_integer_overflow() {
    // let num: u8 = 255;
    // Corrija o seguinte código para evitar o overflow e faça o teste passar.
    // let result = num + 1;
    // assert_eq!(result, 0);
}

#[test]
#[ignore]
fn broken_test_type_mismatch() {
    let boolean: bool = true;
    // Corrija o tipo esperado no assert para que o teste passe.
    assert_eq!(boolean, false);
}

#[test]
fn broken_test_tuple_access() {
    let my_tuple: (i32, f64, char) = (500, 6.4, '🚀');
    // Corrija o acesso ao elemento da tupla para que o teste passe.
    assert_eq!(1.1, 6.4);
}

#[test]
fn broken_test_array_size() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    // Corrija o tamanho esperado do array para que o teste passe.
    assert_eq!(std::mem::size_of_val(&array), 0);
}