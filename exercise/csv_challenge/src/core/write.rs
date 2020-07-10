use super::Error;

pub fn replace_column(data:String,column:&str,replacement:&str) -> Result<String,Error>{
    let mut lines=data.lines();
    let header=lines.next().unwrap();

    let columns:Vec<&str>=header.split(",").collect();
    let column_number=columns.iter().position(|&s| s==column);
    let column_number=match column_number {
        Some(x) => x,
        None => Err("column name does not exist")?
    };

    let mut result=String::with_capacity(data.capacity());
    result.push_str(header);
    for line in lines{
        result.push('\n');
        let mut records: Vec<&str> = line.split(',').collect();
        records[column_number] = replacement;
        result.push_str(&records.join(","));

    }


    Ok(result)

}