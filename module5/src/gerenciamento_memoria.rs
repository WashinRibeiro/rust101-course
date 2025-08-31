#[test]
fn broken_test_scope_fails() {
    {
        let y = 456;
    }
    // Corrija o escopo de `y` ou o ponto de acesso para que o teste passe.
    // assert_eq!(y, 456);
}

#[test]
fn broken_test_ownership_transfer() {
    let s1 = String::from("hello");
    let s2 = s1;
    // Corrija a linha abaixo para que o teste reflita corretamente a propriedade após a transferência.
    // assert_eq!(s1, "hello");
}

// #[test]
// fn broken_test_borrowing() {
//     let s = String::from("hello");
//     let len = calculate_length(&s);
//     // Corrija a função `calculate_length` ou sua chamada para que o teste passe.
//     assert_eq!(len, 0);

//     // Função para correção:
//     // fn calculate_length(s: &String) -> usize {
//     //    s.len()
//     // }
// }

// #[test]
// fn broken_test_mutable_borrowing() {
//     let mut s = String::from("hello");
//     change(&s);
//     // Corrija a função `change` ou sua chamada para que o teste reflita a mudança esperada.
//     assert_eq!(s, "world");

//     // Função para correção:
//     // fn change(some_string: &String) {
//     //    some_string.push_str(", world");
//     // }
// }

// #[test]
// fn broken_test_lifetime_issue() {
//     let r;
//     {
//         let x = 5;
//         r = &x;
//     }
//     // Corrija o problema de tempo de vida para que `r` possa ser usado de forma válida.
//     assert_eq!(*r, 5);
// }

// #[test]
// fn broken_test_slice_usage() {
//     let word = String::from("hello, world");
//     let hello = first_word(&word);
//     // Corrija a função `first_word` ou sua chamada para que o teste reflita a fatia correta.
//     assert_eq!(hello, "hello");

//     // Função para correção:
//     // fn first_word(s: &String) -> &str {
//     //    let bytes = s.as_bytes();
//     //    for (i, &item) in bytes.iter().enumerate() {
//     //        if item == b' ' {
//     //            return &s[0..i];
//     //        }
//     //    }
//     //    &s[..]
//     // }
// }

// #[test]
// fn broken_test_double_borrow() {
//     let mut s = String::from("hello");
//     let r1 = &s;
//     let r2 = &s;
//     let r3 = &mut s; // Corrija o erro de empréstimo mutável enquanto houver empréstimos imutáveis ativos.
//     assert_eq!(r1, &String::from("hello"));
// }

// #[test]
// fn broken_test_vector_borrowing() {
//     let mut v = vec![1, 2, 3, 4];
//     let first = &v[0];
//     v.push(5); // Corrija o erro de não poder alterar `v` enquanto o `first` estiver emprestado.
//     assert_eq!(*first, 1);
// }

// #[test]
// fn broken_test_struct_lifetime() {
//     struct ImportantExcerpt<'a> {
//         part: &'a str,
//     }

//     let novel = String::from("Call me Ishmael. Some years ago...");
//     let first_sentence = novel.split('.').next().expect("Could not find a '.'");
//     let excerpt = ImportantExcerpt { part: first_sentence };

//     // Corrija a declaração de `excerpt` ou sua validação para que o teste passe.
//     assert_eq!(excerpt.part, "Call me Ishmael");
// }

// #[test]
// fn broken_test_clone_instead_of_move() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone();
//     // Corrija a linha abaixo para refletir a operação correta que permite que `s1` e `s2` sejam usados.
//     assert_eq!(s1, "");
// }