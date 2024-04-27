use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;
use xxhash_rust::xxh3::xxh3_64;

#[derive(Deserialize)]
struct SourceText {
    pub id: String,
    #[serde(default)]
    pub text: String,
}

pub struct SearchIndex {
    index: HashMap<u64, Vec<u32>>,
    src_ids: Vec<String>
}

impl SearchIndex {
    pub fn new(src_file: &str) -> SearchIndex {
        let mut index: HashMap<u64, Vec<u32>> = HashMap::with_capacity(2^16);
        let mut src_ids: Vec<String> = vec![String::from("")];
        let mut local_id: u32 = 1;

        println!("Loading data from file {}", src_file);
        // тут вероятно есть более быстрый способ прочитать JSON файл, но когда нужна скорость, то JSON уже медленно и нужно использовать binary кодирование
        let file = File::open(src_file).expect("couldn't open source file");
        let reader = BufReader::new(file);
        // тут мы подтормаживаем при валидации текста на соответствие utf-8, правильнее сразу читать bytes
        let stream = serde_json::Deserializer::from_reader(reader).into_iter::<SourceText>();
    
        for doc in stream {
            match doc {
                Err(e) => println!("{:?}", e),
                Ok(rec) => {
                    // считаем что в большинстве документов не более 2^10 разных слов и не нужно будет увеличивать capacity
                    let mut words: HashSet<u64> = HashSet::with_capacity(2^10);
                    // делим текст документа на слова и сохраняем хеш слова
                    for word in rec.text.split_whitespace() {
                        let hash = xxh3_64(word.as_bytes());
                        words.insert(hash);
                    }
                    // добавляем локальный id документа в основной индекс по хешу слова
                    for word in words {
                        index.entry(word).or_insert_with(|| Vec::with_capacity(2^8)).push(local_id);
                    }
                    // сохраняем оригинальные названия документов
                    src_ids.push(rec.id);
                    local_id += 1;
                },
            }
        }
        SearchIndex {
            index,
            src_ids
        }
    }

    pub fn search(&self, word1: &str, word2: &str) -> Vec<String> {
        let mut found: Vec<String> = Vec::with_capacity(2^12);
        // берём векторы с id документов из основного индекса по хешу слова, если нет, то возвращаем пустой вектор
        let Some(vec1) = self.index.get(&xxh3_64(word1.as_bytes())) else { return found };
        let Some(vec2) = self.index.get(&xxh3_64(word2.as_bytes())) else { return found };
        println!("{}: {} docs, {}: {} docs", word1, vec1.len(), word2, vec2.len());
        let (vec1, vec2) = if vec2.len() < vec1.len() { (vec2, vec1) } else { (vec1, vec2) };
        println!("vec1: {} docs, vec2: {} docs", vec1.len(), vec2.len());
        // ищем пересечение двух векторов внутренних id
        let mut pos1: usize = 0;
        let mut pos2: usize = 0;
        while pos1 < vec1.len() {
            let id1 = vec1[pos1];
            loop {
                if pos2 >= vec2.len() {
                    return found;
                }
                let id2 = vec2[pos2];
                if id1 == id2 {
                    // если, нашли совпадение, то добавляем оригинальное название документа в результат
                    found.push(self.src_ids[id1 as usize].clone());
                    pos1 += 1;
                    pos2 += 1;
                    break;
                }
                if id2 < id1 {
                    pos2 += 1;
                } else {
                    pos1 += 1;
                    break;
                }
            }
        }
        return found
    }
}
