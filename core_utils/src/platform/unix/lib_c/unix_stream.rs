use libc::{self, c_int};

use std::cmp;
use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::mem;
use std::os::unix::net::UnixStream;
use std::os::unix::prelude::*;
use std::process::{Child, ExitStatus};
use std::sync::{Mutex, Once};
use std::time::{Duration, Instant};

/// 从管道中读取数据，直到结束
pub fn drain(mut file: &UnixStream) -> bool {
    let mut ret = false;
    let mut buf = [0u8; 16];
    loop {
        // 从管道读取数据
        match file.read(&mut buf) {
            Ok(0) => return true, // EOF == something happened
            Ok(..) => ret = true, // data read, but keep draining
            Err(e) => {
                if e.kind() == io::ErrorKind::WouldBlock {
                    // 非阻塞模式下，没有数据，则立即返回 WOULDBLOCK 错误
                    return ret;
                } else {
                    panic!("bad read: {}", e)
                }
            }
        }
    }
}

/// 向管道发送数据
pub fn notify(mut file: &UnixStream) {
    match file.write(&[1]) {
        Ok(..) => {}
        Err(e) => {
            if e.kind() != io::ErrorKind::WouldBlock {
                panic!("bad error on write fd: {}", e)
            }
        }
    }
}
