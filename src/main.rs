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

        let cedula = match ler_entrada("Insira o número da cédula (1-9):") {
            Ok(entrada) => {
                if let Ok(num) = entrada.trim().parse::<usize>() {
                    num - 1
                } else {
                    println!("Número inválido!");
                    continue;
                }
            }
            Err(e) => {
                // Se retonar um erro vamos printá-lo
                println!("{e}");
                continue;
            }
        };

        if let Err(e) = marcar_jogada(&mut tabuleiro, jogador_x, cedula) {
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
    println!(); // Uma linha em branco para o espaçamento
    for (index, linha) in tabuleiro.iter().enumerate() {
        if index > 0 {
            println!("---+---+---"); // Linha divisória entre as linhas do tabuleiro
        }
        println!(" {} | {} | {}", linha[0], linha[1], linha[2]);
    }
    println!(); // Uma linha em branco para o espaçamento após o tabuleiro
}

fn ler_entrada(texto: &str) -> Result<String, String> {
    println!("{}", texto);

    let mut entrada = String::new();

    return match std::io::stdin().read_line(&mut entrada) {
        Err(_) => Err("Falha ao ler entrada.".to_string()),
        Ok(_) => Ok(entrada)
    };
}

fn marcar_jogada(
    tabuleiro: &mut [[char; 3]; 3],
    jogador_x: bool,
    ponto: usize,
) -> Result<(), String> {
    let (x, y) = (ponto / 3, ponto % 3);

    if tabuleiro[x][y] != ' ' {
        return Err("Essa cédula já foi marcada".to_string());
    }

    tabuleiro[x][y] = if jogador_x { 'X' } else { 'O' };
    Ok(())
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
