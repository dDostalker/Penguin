use crate::tools_api::HashInfo;
use crate::{GLOBAL_HASH_INFO, GLOBAL_THREAD_POOL, i18n};
use file_hashing::get_hash_file;
use md5::{Digest, Md5};
use sha1::Sha1;
use std::path::PathBuf;
use std::sync::Mutex;

use std::sync::Arc;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    _max_threads: usize,
    _current_threads: usize,
    _work: Vec<Work>,
    sender: Sender<Job>,
}

impl ThreadPool {
    pub fn new(max_threads: usize) -> Self {
        let mut works = Vec::with_capacity(max_threads);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for i in 0..max_threads {
            works.push(Work::new(i, Arc::clone(&receiver)));
        }
        Self {
            _max_threads: max_threads,
            _current_threads: 0,
            _work: works,
            sender: sender,
        }
    }
    pub fn execute<T>(&self, func: T) -> Result<(), &'static str>
    where
        T: FnOnce() + Send + 'static,
    {
        let job = Box::new(func);
        self.sender.send(job).map_err(|_| "线程池发送任务失败")?;
        Ok(())
    }
}

#[allow(unused)]
struct Work {
    id: usize,
    thread: JoinHandle<()>,
}

impl Work {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        Self {
            id,
            thread: thread::spawn(move || {
                loop {
                    // 改进错误处理，避免 panic
                    let job = match receiver.lock() {
                        Ok(guard) => match guard.recv() {
                            Ok(job) => job,
                            Err(_) => break,
                        },
                        Err(_) => break,
                    };
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
fn calc_hash(file_path: &PathBuf) {
    let hash_info = HashInfo {
        md5: calc_md5(file_path),
        sha1: calc_sha1(file_path),
        path: file_path.clone(),
    };
    if let Ok(mut guard) = GLOBAL_HASH_INFO.lock() {
        guard.push(hash_info);
    }
}

pub fn start_calc_hash(file_path: PathBuf) -> anyhow::Result<()> {
    GLOBAL_THREAD_POOL
        .execute(move || {
            calc_hash(&file_path);
        })
        .map_err(|e| anyhow::anyhow!(e))?;
    Ok(())
}

pub fn get_hash_info(path: PathBuf) -> Option<HashInfo> {
    let mut hash_info_vec = GLOBAL_HASH_INFO.lock().ok()?;

    // 查找匹配的哈希信息的索引
    let index = hash_info_vec
        .iter()
        .position(|hash_info| hash_info.is_same(&path))?;

    Some(hash_info_vec.swap_remove(index))
}
