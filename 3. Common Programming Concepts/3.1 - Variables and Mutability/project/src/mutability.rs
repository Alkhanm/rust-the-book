fn main() {
    /* MUTABILIDADE:
     ** Variaveis são imutaveis por padrão.
     ** No entanto, com a sintaxe "mut" é possivel torná-las mutaveis */
    let mut x = 5;
    println!("{}", x);
    x = 6;
    println!("{}", x);
}
