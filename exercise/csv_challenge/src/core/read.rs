use std::path::PathBuf;
use std::fs::File;
use super::Error;
use std::io::prelude::*;

pub fn load_csv(path:PathBuf) -> Result<String,Error>{
    let content=read(path)?;
    Ok(content)
}

pub fn write_csv(content:&str,filename:&str) -> Result<(),Error>{
    write(content,filename)?;
    Ok(())
}

fn read(path:PathBuf) -> Result<String,Error>{
    let mut f=open(path)?;
    let mut buffer=String::new();
    f.read_to_string(&mut buffer)?;
    if buffer.is_empty(){
        Err("empty file")?
    }
    Ok(buffer)

}

fn open(path:PathBuf) -> Result<File,Error>{
    let f=File::open(path)?;
    Ok(f)
}

fn write(content:&str,filename:&str) ->Result<(),Error>{
    let mut file=File::create(filename)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}



