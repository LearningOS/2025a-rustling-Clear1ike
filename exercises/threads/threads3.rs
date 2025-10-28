// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.

use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

// 修改返回值：返回两个发送线程的 JoinHandle，用于主线程同步
fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> (thread::JoinHandle<()>, thread::JoinHandle<()>) {
    let qc = Arc::new(q);
    let qc1 = Arc::clone(&qc);
    let qc2 = Arc::clone(&qc);

    // 克隆 Sender，给第一个线程独立的发送端
    let tx1 = tx.clone();
    let handle1 = thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            tx1.send(*val).unwrap(); // 使用克隆的 tx1 发送
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 给第二个线程使用原始 tx（或再克隆，此处用原始 tx 即可）
    let handle2 = thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            tx.send(*val).unwrap(); // 使用原始 tx 发送
            thread::sleep(Duration::from_secs(1));
        }
    });

    (handle1, handle2) // 返回两个线程的 JoinHandle
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    // 调用 send_tx，获取两个发送线程的 JoinHandle
    let (handle1, handle2) = send_tx(queue, tx);

    // 等待两个发送线程完成，确保所有数据发送完毕
    handle1.join().unwrap();
    handle2.join().unwrap();

    let mut total_received: u32 = 0;
    // rx 迭代接收所有数据（直到所有 Sender 被销毁，通道关闭）
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length); // 验证总数是否为 10
}
