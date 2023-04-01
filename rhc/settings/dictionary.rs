use super::validator::DictionaryValues;

pub struct DictionarySettings {
    wordlist: Vec<String>,
}

impl<'a> DictionarySettings {
    pub fn from(values: Option<DictionaryValues<'a>>, count: usize) -> Option<Self> {
        match values {
            Some(values) => {
                let chunk_size = values.wordlist().len() / values.thread_count();
                let words = values.wordlist().chunks(chunk_size).nth(count).unwrap();
                Some(DictionarySettings {
                    wordlist: words.to_vec(),
                })
            }
            None => None,
        }
    }

    pub fn wordlist(&self) -> &[String] {
        &self.wordlist
    }
}