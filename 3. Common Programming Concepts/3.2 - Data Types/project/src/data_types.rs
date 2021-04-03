fn main() {
    /* Rust é uma linguagem estaticamente tipada, portanto toda valor deve possuir um tipo.
     * Muitas das vezes o compilador consegue inferir esse tipo com base no valor passado,
     * mas em alguns casos isso não é possivel, como quando há muitos valores possiveis.
     * Nessas situações você deve informa-lo */
    let guess: u32 = "42".parse().expect("Not a number");
    println!("{}", guess);
}
