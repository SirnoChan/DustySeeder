/* 计算一个文件的MD5值，再计算前256KiB的MD5值，&获取到文件的大小（以byte计），以及文件名，再用#号把它们分隔开。 */
/*!
#DustySeed：百度网盘秒传种子（标准格式）生成器，支持单个文件，不限大小

用法：`执行文件 <待处理文件>`

跨平台，可配合对应平台下的批处理脚本实现更多功能。

TODO：
0. 错误处理
1. 更完善的命令行参数支持。功能包括限制消耗内存、设置输出的MD5大小写
2. 支持处理文件夹，且支持不同排序方式
3. 对操作系统中不同的字符的处理，目前只支持Unicode字符
4. 支持生成秒传直链
5. 支持GUI
*/

use std::io::{self, Read};
use std::fs::{self, File};
use std::path::Path;
use std::env;
use md5::{Md5, Digest};


const CHUNK_SIZE: usize=0x10000000;     //期待未来可在命令行参数中配置

fn main() -> io::Result<()>{

    //处理命令行参数
    let args: Vec<String> = env::args().collect();
    if args.len()<2 {
        println!("Argument not enough, exiting...");
        return Ok(());
    }
    let path=&args[1];
    let is_hex_uppercase = true;    //期待未来可在命令行参数中配置
    
    //准备处理
    let mut f=File::open(path)?;
    let mut buffer = Box::new([0u8; CHUNK_SIZE]);
    let mut headbuf=Vec::new();
    let headig;
    let digest;
    let mut hasher = Md5::new();

    //读取并处理前256k的内容
    let reference = f.by_ref();
    let headlen = reference.take(256*1024).read_to_end(&mut headbuf)?;
    hasher.update(& mut headbuf);
    let mut lhasher = Md5::new();

    //处理小于256k的文件
    if headlen < 256*1024 {
    lhasher.update(&headbuf[..headlen]);    
    } else {
    lhasher.update(&headbuf[..256*1024]);
    }

//    let mut i =1;

    loop{
        let count=f.read(&mut *buffer)?;    //read方法会从上一次没有读完的地方继续

//        i+=1;
        hasher.update(&mut (*buffer)[..count]);

        if count!=CHUNK_SIZE {
            break;
        }
    }

/*
    while f.read(&mut buffer)? != 0 {
        print!("{:?} ", &buffer[..20]);
        println!("{:?}", &buffer[&buffer.len()-20..]);
        println!("{}", &buffer.len());
        i+=1;
        hasher.update(& mut buffer);
    }
*/
/*
    if f.read_to_end(&mut buffer).unwrap() != 0 {
        print!("{:?} ", &buffer[..20]);
        println!("{:?}", &buffer[&buffer.len()-20..]);
        println!("{}", &buffer.len());
        i+=1;
        hasher.update(& mut buffer);

    }
*/

    digest=hasher.finalize();

    if (headlen) < 256 * 1024 {
        headig=digest;
    } else {
        headig=lhasher.finalize();
    }

    let result: String=md5_format(digest.into(), is_hex_uppercase);
    let lresult: String=md5_format(headig.into(), is_hex_uppercase);
    let len=fs::metadata(path).unwrap().len();
    let filename=Path::new(path).file_name().unwrap().to_string_lossy().into_owned();

    println!("{result}#{lresult}#{len}#{filename}");

    Ok(())
}

fn md5_format(digest: [u8; 16], is_hex_uppercase: bool) -> String {
    digest.iter().map(|num| -> String {
        if is_hex_uppercase {
            format!("{:02X}", num) 
        } else {
        format!("{:02x}", num)
        }
    }).collect()
}