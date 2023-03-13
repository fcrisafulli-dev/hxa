macro_rules! whereami {
    ($reader:tt) => {
        {
            let file_location = $reader.seek(std::io::SeekFrom::Current(0)).expect("Could not get current position!");
            println!("I am here:  {:X?}  !",file_location)
        }
    };
}

/// creates a buffer with the size of the specified type
macro_rules!  buffer{
    ($typ:ty) => {
        [0u8; std::mem::size_of::<$typ>()]
    };

    (exactly $size:expr) => {
        vec![0u8; $size as usize]
    }
}

/// Reads enough bytes to produce the desired type
/// # Example
/// ```rust
/// let foo:u32 = read_bytes!(input u32);
/// ```
macro_rules!  read_bytes{
    ($r:tt $typ:tt) => {
        {
            let mut buffer = buffer!($typ);
            let result = $r.read_exact(&mut buffer);
            match result {
                Ok(_) => {
                    // whereami!($r);
                    // println!("^^ Location, OK read type:{}",stringify!($typ))
                },
                Err(_) => {
                    println!("read_type: {}",stringify!($typ));
                    panic!("Failed read_bytes")
                },
            }
            $typ::from_le_bytes(buffer)
        }
    }
}

/// Reads enough bytes to produce an str
macro_rules!  read_str{
    ($reader:ident $buffer:tt) => {
        {
            let result = $reader.read_exact(&mut $buffer);
            match result {
                Ok(_) => (),
                Err(_) => {
                    whereami!($reader);
                    println!("read_type: {}",stringify!($buffer));
                    panic!("Expected to read more bytes")
                },
            }
            str::from_utf8(&$buffer).expect("Expected a valid utf8 format")
        }
    }
}

pub(crate) use read_str;
pub(crate) use read_bytes;
pub(crate) use buffer;
pub(crate) use whereami;