mod cli;
mod correcao;

fn main() {
    let cli = cli::create_cli();

    let palavra_proxima =
        correcao::corrigir_palavra(cli.dicionario, cli.palavra, cli.variacao_maxima);

    match palavra_proxima {
        Some(palavra) => {
            println!("A palavra mais próxima é {}.", palavra)
        }
        None => {
            println!("Não há nenhuma palavra que corresponde.")
        }
    }
}
