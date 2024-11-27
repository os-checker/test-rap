fn main() {
    let cmd = duct::cmd!("echo", "hi");
    for _ in 0..1 {
        match cmd.clone().run() {
            Err(err) => {
                eprintln!("{err}");
            }
            Ok(a) => _ = a, // no double free by rap

                            // Ok(_) =>(), // double free by rap
                            // _ =>(),// double free by rap
        }
    }
}
