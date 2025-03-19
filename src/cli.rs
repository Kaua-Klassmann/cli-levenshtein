use clap::{Arg, Command};

pub struct Cli {
    pub dicionario: Vec<String>,
    pub palavra: String,
}

pub fn create_cli() -> Cli {
    let matches = Command::new("CLI")
        .arg(
            Arg::new("dicionario")
                .short('D')
                .long("dicionario")
                .help("Lista de palavras separadas por vírgula que serão usadas como referência.")
                .required(true),
        )
        .arg(
            Arg::new("palavra")
                .short('P')
                .long("palavra")
                .help("A palavra digitada que será comparada com o dicionário.")
                .required(true),
        )
        .get_matches();

    let dicionario: Vec<String> = matches
        .get_one::<String>("dicionario")
        .unwrap()
        .split(",")
        .map(|s| s.to_string())
        .collect();

    let palavra = matches.get_one::<String>("palavra").unwrap().clone();

    Cli {
        dicionario,
        palavra,
    }
}
