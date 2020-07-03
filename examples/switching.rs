
#[macro_use] extern crate serde_derive;

use rustbreak::{FileDatabase, backend::FileBackend};
use rustbreak::deser::{Ron, Yaml};

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
enum Country {
    Italy, UnitedKingdom
}

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
struct Person {
    name: String,
    country: Country,
}

fn do_main() -> anyhow::Result<()> {
    use std::collections::HashMap;

    let db = FileDatabase::<HashMap<String, Person>, Ron>::from_path("test.ron", HashMap::new())?;

    println!("Writing to Database");
    db.write(|db| {
        db.insert("john".into(), Person {
            name: String::from("John Andersson"),
            country: Country::Italy
        });
        db.insert("fred".into(), Person {
            name: String::from("Fred Johnson"),
            country: Country::UnitedKingdom
        });
        println!("Entries: \n{:#?}", db);
    })?;

    println!("Syncing Database");
    db.save()?;

    // Now lets switch it

    let db = db.with_deser(Yaml).with_backend(FileBackend::open("test.yml")?);
    db.save()?;

    Ok(())
}

fn main() {
    if let Err(e) = do_main() {
        eprintln!("An error has occurred at: \n{}", e.chain().fold(String::new(), |mut msg, x|
                {msg.push_str(&x.to_string()); msg}));
        ::std::process::exit(1);
    }
}

