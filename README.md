# mongo_archive

A parser for MongoDB archive.

## Example 

```rust
use std::{
    fs::OpenOptions,
    io::{BufWriter, Cursor},
};

use mongo_archive::MongoArchive;

fn main() -> anyhow::Result<()> {
    // read and parse as bson::Bson
    let docs = MongoArchive::from_reader(Cursor::new(include_bytes!("./createshiprecords"))).parse();
    let json_value: serde_json::Value = bson::from_bson(docs)?;
    let out = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .read(false)
        .open("createshiprecords.json")?;
    let buf_writer = BufWriter::new(out);
    serde_json::to_writer(buf_writer, &json_value)?;
    Ok(())
}
```