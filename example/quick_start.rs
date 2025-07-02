use std::{error::Error, fs::File, io::BufReader};

use disposable_email_domains::EmailChecker;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("example/wildcard.json")?;
    let reader = BufReader::new(file);
    let available_suffix: Vec<String> = serde_json::from_reader(reader)?;

    let check = EmailChecker::form_vec(&available_suffix);
    for waiting_check in ["sdads@dsd.com", "adad@062e.com", "dasd@33mail.com"] {
        println!("check:{waiting_check} result:{}",check.check(waiting_check));
    }
    Ok(())
}
