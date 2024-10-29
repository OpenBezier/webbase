use uuid::Uuid;

pub fn get_uuid() -> Uuid {
    Uuid::new_v4()
}

pub fn get_uuid_string() -> String {
    Uuid::new_v4().to_string()
}

pub fn get_uuid_string_without_minus() -> String {
    let tmp = Uuid::new_v4().to_string();
    tmp.replace("-", "")
}

// length 12
pub fn create_app_code() -> String {
    let id = uuid::Uuid::new_v4().to_string();
    let id = id.replace("-", "");
    let id = id.split_at(12).1;
    id.to_string()
}

// length 32
pub fn create_app_secret() -> String {
    let id = uuid::Uuid::new_v4().to_string();
    let id = id.replace("-", "");
    id
}

use md5::Digest;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Seek;

// 获取文件所有内容的MD5值，非一次性加载到内存计算，对于大文件很友好，同时返回了文件的长度
pub fn get_file_md5(file_path: &str) -> (String, u64) {
    let mut hasher = md5::Md5::new();
    let file = std::fs::File::open(&file_path).unwrap();
    let len = file.metadata().unwrap().len();
    let buf_len = len.min(1_000_000_00) as usize;
    let mut buf = BufReader::with_capacity(buf_len, file);
    loop {
        let part = buf.fill_buf().unwrap();
        if part.is_empty() {
            break;
        }
        hasher.update(part);
        let part_len = part.len();
        buf.consume(part_len);
    }
    let md5_str = format!("{:x}", hasher.finalize());
    (md5_str, len)
}

// 获取文件尾部size长度内容的MD5值，在一些应用场景是蛮有用的
pub fn get_file_end_md5(file_path: &str, size: u32) -> String {
    let mut file = std::fs::File::open(&file_path).unwrap();
    let mut hasher = md5::Md5::new();
    // let mut buf = [0u8; 1024];
    let mut buf: Vec<u8> = vec![0u8; size as usize];
    let md5_str = if file.metadata().unwrap().len() <= size as u64 {
        let n = file.read(&mut buf).unwrap();
        hasher.update(&buf[..n]);
        let md5_last = format!("{:x}", hasher.finalize());
        md5_last
    } else {
        file.seek(std::io::SeekFrom::End((!size + 1).into()))
            .unwrap();
        let n = file.read(&mut buf).unwrap();
        hasher.update(&buf[..n]);
        let md5_last = format!("{:x}", hasher.finalize());
        md5_last
    };
    md5_str
}

pub fn strip_url_last_slash(url: &String) -> String {
    let url = if url.ends_with("/") {
        url.strip_suffix("/").unwrap().to_string()
    } else {
        url.clone()
    };
    url
}
