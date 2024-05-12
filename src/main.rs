fn main() {
    // Tabuleiro inicializado com [[' ', ' ', ' '], [' ', ' ', ' '], [' ', ' ', ' ']]
    let mut tabuleiro = [[' '; 3]; 3];
    // O jogo começa pelo jogado "X"
    let mut jogador_x = true;

    loop {
        // Mostra uma imagem do tabuleiro atual
        exibir_tabuleiro(&tabuleiro);

        if jogador_x {
            println!("Vez do jogador X:");
        } else {
            println!("Vez do jogador O:");
        }

        let entrada = match ler_entrada() {
            Ok(num) => num - 1,
            Err(e) => {
                // Se retonar um erro vamos printá-lo
                println!("{e}");
                continue;
            }
        };

        if let Err(e) = marcar_jogada(&mut tabuleiro, jogador_x, entrada) {
            // Se retonar um erro vamos printá-lo
            println!("{e}");
            continue;
        }

        if verificar_vitoria(&tabuleiro) {
            if jogador_x {
                println!("Jogador X ganhou a partida");
            } else {
                println!("Jogador O ganhou a partida");
            }
            break;
        } else if deu_empate(&tabuleiro) {
            println!("Deu velha!!");
            break;
        }

        jogador_x = !jogador_x;
    }
}

fn exibir_tabuleiro(tabuleiro: &[[char; 3]; 3]) {
    println!(); // Uma linha em branco para espaçamento
    for (index, linha) in tabuleiro.iter().enumerate() {
        if index > 0 {
            println!("---+---+---"); // Linha divisória entre as linhas do tabuleiro
        }
        println!(" {} | {} | {}", linha[0], linha[1], linha[2]);
    }
    println!(); // Uma linha em branco para espaçamento após o tabuleiro
}

fn ler_entrada() -> Result<usize, String> {
    println!("Insira o número da cédula (1-9):");

    let mut entrada = String::new();

    match std::io::stdin().read_line(&mut entrada) {
        Err(_) => return Err("Falha ao ler jogada.".to_string()),

        Ok(_)=> return Ok(entrada.trim().parse::<usize>().unwrap())
    }
}

fn marcar_jogada(
    tabuleiro: &mut [[char; 3]; 3],
    jogador_x: bool,
    ponto: usize
) -> Result<(), String> {
    todo!()
}

fn verificar_vitoria(tabuleiro: &[[char; 3]; 3]) -> bool {
    for i in 0..=2 {
        // Verifica posições iguais verticalmente e horizontalmente
        if linha_igual(tabuleiro[i][0], tabuleiro[i][1], tabuleiro[i][2])
            || linha_igual(tabuleiro[0][i], tabuleiro[1][i], tabuleiro[2][i])
        {
            return true;
        }
    }

    // Verifica posições iguais nas diagonais
    if linha_igual(tabuleiro[0][0], tabuleiro[1][1], tabuleiro[2][2])
        || linha_igual(tabuleiro[2][0], tabuleiro[1][1], tabuleiro[0][2])
    {
        return true;
    }

    false
}

fn deu_empate(tabuleiro: &[[char; 3]; 3]) -> bool {
    !tabuleiro.iter().any(|row| row.iter().any(|col| col == &' '))
}

fn linha_igual(a: char, b: char, c: char) -> bool {
    a == b && b == c && a != ' '
}