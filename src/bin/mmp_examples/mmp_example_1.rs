
use std::fs::File;
use std::io::{Cursor, Read};
use rand::Rng;
use memmap2::{MmapOptions};
use std::io::Write;
use std::{thread};
use std::sync::Arc;
use std::time::Duration;
use arrow::array::{Float64Array, Int32Array, RecordBatch, StringArray};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::ipc::writer::FileWriter;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .open("/home/ryan/workspace/tmp/test_mmp.txt")?;

    let file_size: usize = 2000;
    file.set_len(file_size as u64)?;

    let mut mmf = unsafe {
        MmapOptions::new().len(file_size).map_mut(&file)?
    };

    let mut rng = rand::thread_rng();

    loop {
        let int_vec: Vec<i32> = (0..5).map(|_| rng.gen::<i32>()).collect();
        let float_vec: Vec<f64> = (0..5).map(|_| rng.gen::<f64>()).collect();

        let int_array = Int32Array::from(int_vec.clone());
        let float_array = Float64Array::from(float_vec.clone());
        let string_array = StringArray::from(vec!["apple", "banana", "pineapple", "melon", "pear"]);

        let schema = Schema::new(vec![
            Field::new("id", DataType::Int32, false),
            Field::new("value", DataType::Float64, false),
            Field::new("name", DataType::Utf8, false),
        ]);

        let batch = RecordBatch::try_new(
            Arc::new(schema),
            vec![
                Arc::new(int_array),
                Arc::new(float_array),
                Arc::new(string_array)
            ]
        ).unwrap();

        let sink = Cursor::new(&mut mmf[..]);
        let mut writer = FileWriter::try_new(sink, batch.schema_ref()).unwrap();
        writer.write(&batch).unwrap();
        writer.finish().unwrap();
        let p = writer.get_ref().position();
        println!("tick. {int_vec:?}, {float_vec:?}. Position: {p}");
        thread::sleep(Duration::from_secs(5));
    }

    Ok(())
}