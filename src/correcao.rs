pub fn corrigir_palavra(
    dicionario: Vec<String>,
    palavra_digitada: String,
    variacao_maxima: u32,
) -> Option<String> {
    let (mut palavra_proxima, mut variacao) = (None, u32::MAX);

    for palavra in dicionario.iter() {
        let distancia = distancia_com_levenshtein(&palavra, &palavra_digitada);

        if distancia > variacao_maxima || distancia >= variacao {
            continue;
        }

        palavra_proxima = Some(palavra);
        variacao = distancia;
    }

    palavra_proxima.cloned()
}

fn distancia_com_levenshtein(palavra1: &String, palavra2: &String) -> u32 {
    let len_palavra1 = palavra1.chars().count();
    let len_palavra2 = palavra2.chars().count();

    let mut tabela = vec![vec![0; len_palavra1 + 1]; len_palavra2 + 1];

    for x in 0..=len_palavra1 {
        tabela[0][x] = x;
    }

    for y in 0..=len_palavra2 {
        tabela[y][0] = y;
    }

    let mut custo;
    let palavra1_chars: Vec<char> = palavra1.chars().collect();
    let palavra2_chars: Vec<char> = palavra2.chars().collect();

    for y in 1..=len_palavra2 {
        for x in 1..=len_palavra1 {
            if palavra1_chars[x - 1] == palavra2_chars[y - 1] {
                custo = 0;
            } else {
                custo = 1;
            }

            tabela[y][x] = *vec![
                tabela[y - 1][x] + 1,
                tabela[y][x - 1] + 1,
                tabela[y - 1][x - 1] + custo,
            ]
            .iter()
            .min()
            .unwrap();
        }
    }

    *tabela.last().unwrap().last().unwrap() as u32
}
