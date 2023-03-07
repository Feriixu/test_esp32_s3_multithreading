use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    // Make 50 Threads, which just print their ID and the i they are at
    let mut threads = Vec::new();
    for i in 0..50 {
        threads.push(std::thread::spawn(move || {
            for j in 0..100 {
                println!("Thread {} at {}", i, j);
            }
        }));
    }

    // Join all threads
    for thread in threads {
        thread.join().unwrap();
    }

    println!("Done!");
}
