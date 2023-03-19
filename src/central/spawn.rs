use super::{setup::ThreadSettings, Message};
use crate::core::{dictionary::Dictionary, incremental::Incremental, result::PasswordMatch};
use std::{sync::mpsc::Sender, thread};

pub(super) fn job(tx: Sender<Message>, settings: ThreadSettings) {
    thread::spawn(move || {
        if settings.dictionary().is_some() {
            Dictionary::from(&settings, &tx).run();
        }
        Incremental::from(&settings, &tx).run();
        drop(settings);
    });
}
