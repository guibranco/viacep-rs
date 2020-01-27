# viacep-rs
The [ViaCEP](https://viacep.com.br) client wrapper written in Rust

| Service      | Status |
| -------      | :----: |
| AppveyorCI   | [![Build status](https://ci.appveyor.com/api/projects/status/w1di231c9hr2tyhy/branch/master?svg=true)](https://ci.appveyor.com/project/guibranco/viacep-rs/branch/master) |
| crates.io    | [![crates.io](https://img.shields.io/crates/v/viacep-rs.svg)](https://crates.io/crates/viacep-rst) |

Pure Rust bindings to the [Holiday API](https://holidayapi.com).

## Dependencies and support

viacep-rs is intended to work on all tier 1 supported Rust systems:

- MacOSX
- Linux
- Windows

## Minimum Compiler Version

Due to the use of certain features holiday-api-rust requires `rustc` version 1.18 or
higher.

## Getting Started

Add the following to your `Cargo.toml`

```toml
[dependencies]
viacep_rs = "0.1.0"
serde_json = "1.0"
```

Then in your `lib.rs` or `main.rs` file add:

```rust
extern crate holiday_api_rust;

let client = ViaCepClient::new();

// Single Zip Code data 
match client.get_zipcode("03177010") {
    Err(e) => eprintln!("{:?}", e),
    Ok(data) => {
        let cep = data.unwrap();
        println!("IBGE: {} | Address: {} | Neighborhood: {} | City: {} | UF: {}", cep.ibge, cep.Address, cep.Neighborhood, cep.City, cep.state_initials);
    }
}


//Find by address data
match client.search("SP", "Sâo Paulo", "Paulista") {
    Err(e) => eprintln!("{:?}", e),
    Ok(data) => {
        let addresses = data.unwrap();
        for address in addresses {
            println!("IBGE: {} | Address: {} | City: {} | Zip: {}", address.ibge, address.address, address.city, address.zip);
        }
    }
}
```

## License

Licensed under

- MIT license ([LICENSE](https://github.com/guibranco/viacep-rs/blob/master/LICENSE) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))