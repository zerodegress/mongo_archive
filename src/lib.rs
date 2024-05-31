use byteorder::{LittleEndian, ReadBytesExt};
use bytes::Buf;
use memchr::memmem;
use std::io::{Cursor, Read, Seek, SeekFrom};

pub struct MongoArchive<'a> {
    inner: Box<dyn Read + 'a>,
}

impl<'a> MongoArchive<'a> {
    pub fn from_reader<R: Read + 'a>(read: R) -> Self {
        Self {
            inner: Box::new(read),
        }
    }

    pub fn parse(&mut self) -> bson::Bson {
        let mut buf = Vec::new();
        if self.inner.read_to_end(&mut buf).is_err() {
            return bson::Bson::Array(vec![]);
        }
        let finder = memmem::Finder::new("\x07_id");
        let buf = if let Some(id_prefix_index) = finder.find(&buf) {
            buf[(id_prefix_index - 4)..].to_vec()
        } else {
            return bson::Bson::Array(vec![]);
        };
        let mut buf_reader = Cursor::new(buf);

        let mut docs = Vec::new();

        loop {
            if buf_reader.remaining() < 4 {
                break;
            }
            let doc_size = if let Ok(doc_size) = buf_reader.read_i32::<LittleEndian>() {
                doc_size
            } else {
                break;
            };
            if buf_reader.remaining() < doc_size as usize {
                break;
            }
            buf_reader.seek(SeekFrom::Current(-4)).unwrap();
            let doc_reader = std::io::Read::take(buf_reader.by_ref(), doc_size as u64);
            docs.push(bson::Bson::Document(
                match bson::Document::from_reader(doc_reader) {
                    Ok(doc) => doc,
                    Err(err) => {
                        println!("{}", err);
                        break;
                    }
                },
            ));
        }
        bson::Bson::Array(docs)
    }
}
