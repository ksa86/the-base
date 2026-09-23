use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

pub const BUFFER_SIZE: usize = 64 * 1024;

// -----------------------------------------------------------------------------
// MyBufReader
// -----------------------------------------------------------------------------


#[allow(dead_code)]
pub struct MyBufReader {
    // TODO: добавьте необходимые поля для буферизации чтения
    // возможно, вам понадобится что-то вроде:
    // поле для хранения данных, прочитанных из файла, но ещё не отданных пользователю,
    // или позиция внутри этого буфера, откуда отдавать следующий байт.
    // ну и возможно что-то ещё, что поможет вам реализовать read_byte() эффективно

    file: File,
    buffer: [u8; BUFFER_SIZE],
    position: usize,
    capacity: usize
}

impl MyBufReader {
    pub fn open(path: impl AsRef<Path>) -> io::Result<Self> {
        let file = File::open(path)?;
        Ok(Self {
            file,
            buffer: [0; BUFFER_SIZE],
            position: 0,
            capacity: 0
        })
    }

    #[inline]
    pub fn read_byte(&mut self) -> io::Result<Option<u8>> {
        
        if self.position >= self.capacity {
            // просим зачитать новую порцию
            self.capacity = self.file.read(&mut self.buffer)?;
            self.position = 0;

            if self.capacity == 0 {
                return Ok(None);
            }
        }

        let byte = self.buffer[self.position];
        self.position += 1;
        Ok(Some(byte))
        

        /*
        1. без разбивки на горячую и холодную ветки и inline-а read_byte
        opy_slow: 2.43s, 0.39 MiB/s
        copy_fast: 4.74ms, 201.07 MiB/s

        2. без разбивки с inline-ом
        copy_slow: 2.46s, 0.39 MiB/s
        copy_fast: 4.06ms, 234.76 MiB/s

        3. с разбивкой
        copy_slow: 2.46s, 0.39 MiB/s
        copy_fast: 4.29ms, 222.47 MiB/s

        */

        // TODO: в качестве теста попытка разделить грячую ветку (отдать байт)
        // и холодную (подгрузить данные)
        /*
        if self.position < self.capacity {
            // горячий
            let byte = self.buffer[self.position];
            self.position += 1;
            return Ok(Some(byte));
        }

        // холодный
        self.read_next_block()
        */
    }

    
    /*
    #[inline(never)]
    fn read_next_block(&mut self) -> io::Result<Option<u8>>{
        self.capacity = self.file.read(&mut self.buffer)?;
        self.position = 0;

        if self.capacity == 0 {
            Ok(None)
        } else {
            let byte = self.buffer[self.position];
            self.position += 1;
            Ok(Some(byte))
        }
    }
    */
}

// -----------------------------------------------------------------------------
// MyBufWriter
// -----------------------------------------------------------------------------

pub struct MyBufWriter {
    // TODO: добавьте необходимые поля для буферизации записи
    // наверное, вам понадобится буфер для хранения данных, которые пользователь уже передал в
    // write_buffered(),
    // но ещё не записал в файл, и, конечно же, еще какие-то поля для работы с файлом

    file: File,
    buffer: [u8; BUFFER_SIZE],
    current_len: usize,     // сколько сейчас в буфере
}

impl MyBufWriter {
    pub fn create(path: impl AsRef<Path>) -> io::Result<Self> {
        let file = File::create(path)?;
        Ok(Self {
            file,
            buffer: [0; BUFFER_SIZE],
            current_len: 0
        })
    }

    pub fn write_buffered(&mut self, data: &[u8]) -> io::Result<()> {

        
        let mut pos = 0;
        while pos < data.len() {
            // свободного места в буфере
            let buf_remainder = BUFFER_SIZE - self.current_len;

            // сколько осталось записать из data
            let data_remainder = data.len() - pos;
            
            let to_copy = if data_remainder < buf_remainder {
                data_remainder
            } else {
                buf_remainder
            };

            // пишем в буфер
            self.buffer[self.current_len..self.current_len+to_copy].copy_from_slice(&data[pos..pos+to_copy]);
            self.current_len += to_copy;
            pos += to_copy;

            // если заполнился, flash
            if self.current_len == BUFFER_SIZE{
                self.flush()?;
            }
        }

        Ok(())
    }

  

    pub fn flush(&mut self) -> io::Result<()> {
        // TODO: эта функция должна записать в файл все данные, которые сейчас лежат во внутреннем
        // буфере, верно? И после этого внутренний буфер должен быть пустым, готовым для новых
        // данных от пользователя.
        // но пока что просто заглушка, чтобы код компилировался, вам нужно реализовать эту функцию
        

        if self.current_len > 0 {
            self.file.write_all(&self.buffer[..self.current_len])?;
            self.current_len = 0;
        }
        
        Ok(())
    }

    pub fn close(mut self) -> io::Result<()> {
        self.flush()
        //self.file.flush()?;
    }
}

impl Drop for MyBufWriter {
    fn drop(&mut self) {
        // Ошибку из Drop вернуть нельзя.
        // Поэтому в реальном коде лучше явно вызывать close() или flush().
        let _ = self.flush();
    }
}

// -----------------------------------------------------------------------------
// Медленная версия
// -----------------------------------------------------------------------------

pub fn copy_slow(input: impl AsRef<Path>, output: impl AsRef<Path>) -> io::Result<u64> {
    let mut input = File::open(input)?;
    let mut output = File::create(output)?;

    let mut copied = 0;
    let mut byte = [0u8; 1];

    loop {
        let n = input.read(&mut byte)?;
        if n == 0 {
            break;
        }

        output.write_all(&byte[..n])?;
        copied += n as u64;
    }

    output.flush()?;

    Ok(copied)
}

// -----------------------------------------------------------------------------
// Быстрая версия
// -----------------------------------------------------------------------------
// copy_fast специально тоже использует побайтный API.
// Разница должна быть не в коде копирования, а в реализации MyBufReader и MyBufWriter
// эту функцию не нужно менять, она должна работать с любыми реализациями MyBufReader и MyBufWriter,
// которые вы сделаете
pub fn copy_fast(input: impl AsRef<Path>, output: impl AsRef<Path>) -> io::Result<u64> {
    println!("test..");
    let mut reader = MyBufReader::open(input)?;
    let mut writer = MyBufWriter::create(output)?;

    let mut copied = 0;

    while let Some(byte) = reader.read_byte()? {
        writer.write_buffered(&[byte])?;
        copied += 1;
    }

    writer.close()?;

    Ok(copied)
}

pub const RECORD_SIZE: usize = 10;

pub fn make_record(index: usize) -> [u8; RECORD_SIZE] {
    let mut record = [0u8; RECORD_SIZE];

    (0..RECORD_SIZE).for_each(|i| {
        record[i] = ((index + i) % 251) as u8;
    });

    record
}

pub fn generate_input_file(path: impl AsRef<Path>, records: usize) -> io::Result<()> {
    let mut file = File::create(path)?;

    for i in 0..records {
        let record = make_record(i);
        file.write_all(&record)?;
    }

    file.flush()?;

    Ok(())
}
