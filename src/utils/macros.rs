#[macro_export]
macro_rules! readln {
    () => {
        {
            let mut content_read: String = String::new();
            ::std::io::stdin().read_line(&mut content_read).unwrap_or_else(|err|
            {
                eprintln!("Error while reading from stdin. {}", err);
                0
            });
            content_read = content_read.trim().to_string();
            content_read
        }
    }
}