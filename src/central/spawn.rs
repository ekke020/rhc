use super::setup::ThreadSettings;
use crate::core::{dictionary::Dictionary, incremental::Incremental, result::PasswordMatch};
use std::{sync::mpsc::Sender, thread};

pub(super) fn job(tx: Sender<Option<PasswordMatch>>, settings: ThreadSettings) {
    thread::spawn(move || {
        let result = match settings.dictionary() {
            Some(dict) => {
                Dictionary::from(settings.target(), dict.wordlist(), settings.algorithm()).run()
            }
            None => None,
        };
        let result = match result {
            Some(pm) => Some(pm),
            None => Incremental::from(
                settings.target(),
                settings.incremental(),
                settings.algorithm(),
            )
            .run(),
        };
        drop(settings);
        tx.send(result);
    });
}
