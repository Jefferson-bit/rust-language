use std::env;
use std::str::FromStr;

fn main() {
    println!("Hello, world!");
    linecommand();
}

// fn significa funcao
// o token -> significa oq vamos retornar. no caso um inteiro de 64 bytes

// por padrao uma vez iniciada uma variavel, seu valor n pode ser alterado
// mas colocar a palvra-chave MUT antes do parametro N e M permite que  o corpo
// da nossa função os modifique
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            // let indica uma variavel local
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    println!("resultado do calculo {}", n);
    n
}

fn linecommand() {
    let mut numbers = Vec::new();

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("erro parsing argument"));
    }

    if numbers.len() == 0 {
        //printa msg de erro
        eprint!("Usage: gcd NUMBER...");
        //
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    print!("the greatest commom divisor of {:?} is {}", numbers, d);
}

//#[test] são como atributos em C++ ou @anotacoesJava
#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}
