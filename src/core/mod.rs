pub mod crack;
mod error;
mod package;
mod setup;
mod spawn;
use self::error::core::CoreError;
use self::package::Package;
use crate::cli::settings::UnvalidatedSettings;
use crate::cli::Settings;

pub type Error = error::core::CoreError;

// TODO: Return a result of the successful crack.
// pub fn run(mut settings: GlobalSettings) -> Result<(), CoreError> {
//     let package = Package::assemble(&mut settings)?;
//     let wordlist = settings.get_wordlist();

//     let threads = match wordlist {
//         Some(wordlist) => spawn::resource_job(package, wordlist)?,
//         None => spawn::brute_force_job(package)?,
//     };
//     for thread in threads {
//         let result = thread.join().unwrap();
//         if result.is_some() {
//             println!("{}", result.unwrap());
//             break;
//         }
//     }
//     Ok(())
// }

pub fn run(mut package: Package) -> Result<(), CoreError> {
    let rx = spawn::test_incremental(package);
    let value = rx.recv().unwrap();
    if let Some(value) = value {
        println!("{}", value);
    } else {
        println!("sadness");
    }
    Ok(())
}


pub fn new_run(settings: Settings) -> Result<(), CoreError> {
    let threadObj = setup::generate_thread_
    Ok(())
}

// let mut chunks = wordlist
//     .chunks(chunk_size)
//     .map(|chunk| chunk.to_vec())
//     .collect::<Vec<Vec<String>>>();

// pub fn test_brute_force(mut settings: GlobalSettings) -> Result<(), CoreError> {
//     let package = Package::assemble(&mut settings)?;
//     // let rx = spawn::brute_force_job(package);
//     let counter = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));
//     // let value = rx.recv().unwrap();
//     let mut bf = crack::incremental::Incremental::from(
//         package.get_target(),
//         &constants::NO_SPECIAL_RANGE[0..6],
//         counter,
//         package.get_algorithms().get(0).unwrap().get_algorithm(),
//     );
//     bf.run();
//     // println!("{}", value.unwrap());
//     Ok(())
// }
