use std::{sync::mpsc::Sender, thread};

use crate::central::{Message, setup::ThreadSettings};

use super::{dictionary::Dictionary, incremental::Incremental};

pub fn job(tx: Sender<Message>, settings: ThreadSettings) {
    let handler = thread::spawn(move || {
        if settings.dictionary().is_some() {
            Dictionary::from(&settings, &tx).run();
        }
        Incremental::from(&settings, &tx).run();
        drop(settings);
    });
}
