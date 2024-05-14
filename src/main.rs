use chrono::Local;

fn main() {
    let jogador_x_nome = ler_entrada("Digite seu nome Jogador X: ")
        .unwrap()
        .replace("\n", "");
    let jogador_o_nome = ler_entrada("Digite seu nome Jogador O: ")
        .unwrap()
        .replace("\n", "");

    // Tabuleiro inicializado com [[' ', ' ', ' '], [' ', ' ', ' '], [' ', ' ', ' ']]
    let mut tabuleiro = [[' '; 3]; 3];
    // O jogo começa pelo jogado "X"
    let mut jogador_x = true;
    let inicio = Local::now();

    loop {
        // Mostra uma imagem do tabuleiro atual
        exibir_tabuleiro(&tabuleiro);

        if jogador_x {
            println!("Vez do jogador X = {jogador_x_nome}");
        } else {
            println!("Vez do jogador O = {jogador_o_nome}");
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

        let deu_empate = deu_empate(&tabuleiro);
        let deu_vitoria = verificar_vitoria(&tabuleiro);

        if deu_empate || deu_vitoria {
            println!("\nRelatório da Partida:");
            println!("Jogadores Participantes: {} vs {}", &jogador_x_nome, &jogador_o_nome);

            if deu_vitoria {
                let jogador_vencedor = if jogador_x {
                    &jogador_x_nome
                } else {
                    &jogador_o_nome
                };

                println!("Vencedor: {jogador_vencedor}");
            } else {
                println!("Vencedor: Empate");
            }

            let data_hora_atual = Local::now(); //Capturar o horário atual
            let duracao_jogo = data_hora_atual.signed_duration_since(inicio); //Calcular duração do jogo

            println!("Hora e Data Inicial do Jogo: {}", inicio.format("%Y-%m-%d %H:%M:%S"));
            println!("Hora e Data Final do Jogo: {}", data_hora_atual.format("%Y-%m-%d %H:%M:%S"));
            println!("Duração do Jogo: {} segundos", duracao_jogo.num_seconds());

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
    println!(); // Uma linha em branco para espaçamento após o tabuleiro
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
