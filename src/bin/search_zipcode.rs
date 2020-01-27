  
extern crate dotenv;
extern crate viacep_rs;
#[macro_use]
extern crate prettytable;

use dotenv::dotenv;
use viacep_rs::ViaCepClient;
use prettytable::Table;
use prettytable::format;

fn main(){
    dotenv().ok();

    let client = ViaCepClient::new();

    match client.get_zipcode("03177010"){
        Err(e) => eprintln!("{:?}", e),
        Ok(data) =>{
            let cep = data.unwrap();
            let mut table = Table::new();
            table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
            table.set_titles(row!["State", "City", "Neighborhood"]);
            table.add_row(row![cep.state_initials, cep.city, cep.neighborhood]);
            table.printstd();
        }
    }
}