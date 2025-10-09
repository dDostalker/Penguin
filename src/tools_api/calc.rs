use crate::tools_api::HashInfo;
use crate::{i18n, GLOBAL_HASH_INFO, GLOBAL_THREAD_POOL};
use std::sync::Mutex;
use file_hashing::get_hash_file;
use md5::{Digest, Md5};
use sha1::Sha1;
use std::path::PathBuf;

use std::sync::mpsc::{self, Sender, Receiver};
use std::sync::Arc;
use std::thread::{self, JoinHandle};

type Job = Box<dyn FnOnce() + Send + 'static>;
pub struct ThreadPool
{
    max_threads: usize,
    current_threads: usize,
    work: Vec<Work>,
    sender: Sender<Job>,
}
impl ThreadPool 
{
    pub fn new(max_threads: usize) -> Self {
        let mut works = Vec::with_capacity(max_threads);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for i in 0..max_threads {
            works.push(Work::new(i, Arc::clone(&receiver)));
        }
        Self { 
            max_threads, 
            current_threads: 0, 
            work: works, 
            sender: sender,
        }
    }
    pub fn execute<T>(&self,func: T) 
    where T: FnOnce() + Send + 'static,
     {
        let job= Box::new(func);
        self.sender.send(job).unwrap();
    }
}

struct Work{
    id: usize,
    thread: JoinHandle<()>,
}

impl Work
{
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        Self { 
            id,
            thread: thread::spawn(move || {
                loop{
                    let job = receiver.lock().unwrap().recv().unwrap();
                    job();
                }
            }),
        }
    }
}



#[inline]
pub fn calc_md5(file_path: &PathBuf) -> String {
    let mut hasher = Md5::new();
    match get_hash_file(file_path, &mut hasher) {
        Ok(hash) => hash,
        Err(_e) => i18n::CALC_MD5_FAILED.to_string(),
    }
}
#[inline]
pub fn calc_sha1(file_path: &PathBuf) -> String {
    let mut hasher = Sha1::new();
    match get_hash_file(file_path, &mut hasher) {
        Ok(hash) => hash,
        Err(_e) => i18n::CALC_SHA1_FAILED.to_string(),
    }
}
fn calc_hash(file_path: &PathBuf){
    let hash_info = HashInfo {
        md5: calc_md5(file_path),
        sha1: calc_sha1(file_path),
        path: file_path.clone(),
    };
    GLOBAL_HASH_INFO.lock().unwrap().push(hash_info);
}

pub fn start_calc_hash(file_path: PathBuf) -> anyhow::Result<()> {
    GLOBAL_THREAD_POOL.execute(move || {
        calc_hash(&file_path);
    });
    Ok(())
}

pub fn get_hash_info(path: PathBuf) -> Option<HashInfo> {
    let hash_info = GLOBAL_HASH_INFO.lock().unwrap().iter().find(|hash_info| hash_info.is_same(&path)).cloned();
    if hash_info.is_none() {
        return None;
    }
    GLOBAL_HASH_INFO.lock().unwrap().pop_if(|hash_info| hash_info.is_same(&path));
    hash_info
}




