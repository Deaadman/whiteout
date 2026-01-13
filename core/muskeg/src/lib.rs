pub mod game_launchers;
pub mod melonloader;

pub async fn test() -> Result<(), reqwest::Error> {

    let res = reqwest::get("https://tldmods.com/api.json?details=").await?;

    // eprintln!("Response: {:?} {}", res.version(), res.status());
    // eprintln!("Headers: {:#?}\n", res.headers());

    let body = res.text().await?;
    // println!("{body}");

    let json: Vec<Mod> = serde_json::from_str(&body).expect("Something went wrong.");
    for i in json {
        println!("{} - {}", i.name, i.version);
    }

    Ok(())

    // let response = Request::get("https://tldmods.com/api.json?details=");
    //     // .header("User-Agent", "AutoUpdatingPlugin")
    //     // .body(())
    //     // .unwrap();

    // println!("{:?}", response);

    // //
}

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
