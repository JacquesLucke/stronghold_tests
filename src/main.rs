#[actix::main]
async fn main() {
    let system = riker::actors::ActorSystem::new().unwrap();
    let client_path = vec![1, 2, 3];
    let mut stronghold =
        iota_stronghold::Stronghold::init_stronghold_system(system, client_path.clone(), vec![]);
    let location = iota_stronghold::Location::Generic {
        vault_path: vec![4, 5, 6],
        record_path: vec![7, 8, 9],
    };
    let key_data = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32,
    ];
    let filename = "my_snapshot.txt";
    let path = "/home/jacques/Documents/my_snapshot.txt";
    stronghold
        .write_to_store(location.clone(), vec![100, 101, 102], None)
        .await;
    stronghold
        .write_all_to_snapshot(&key_data, Some(filename.to_owned()), Some(path.into()))
        .await;

    stronghold
        .read_snapshot(
            client_path.clone(),
            None,
            &key_data,
            Some(filename.to_owned()),
            Some(path.into()),
        )
        .await;

    let read_result = stronghold.read_from_store(location.clone()).await.0;
    print!("Read result: ");
    for value in read_result.iter() {
        print!("{}, ", value);
    }
    println!();

    println!("Done!");
}
