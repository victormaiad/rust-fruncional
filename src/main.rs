fn main() {
    let mut tabuleiro = [[' '; 3]; 3];
    let jogador_x = true;

    let mut input = String::new();
    let stdin = std::io::stdin();
    match stdin.read_line(&mut input) {
        Ok(_) => {
            println!("Vocẽ digitou {input}")
        }
        Err(_) => return
    }
}
