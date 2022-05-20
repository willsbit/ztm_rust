// Topic: Multithreading
//
// Requirements:
// * Run the provided functions in threads
// * Retrieve the data from the threads to print the message
//   "Hello, threads!"
//
// Notes:
// * Use the join function to wait for threads to finish

fn msg_hello() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "Hello, "
}

fn msg_thread() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "threads"
}

fn msg_excited() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "!"
}

fn main() {
    let msg1 = std::thread::spawn(move || {msg_hello()});
    let msg2 = std::thread::spawn(move || {msg_thread()});
    let msg3 = std::thread::spawn(move || {msg_excited()});

    let msg1 = msg1.join().expect("failed to join msg one");
    let msg2 = msg2.join().expect("failed to join msg two");
    let msg3 = msg3.join().expect("failed to join msg three");

    println!("{}{}{}", msg1, msg2, msg3);
}
