use bytecount;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: rustine <file>");
        std::process::exit(1);
    }
    let in_file = &args[1];

    let mut f = std::io::BufReader::new(std::fs::File::open(in_file).expect("open failed"));

    eprintln!("{}", count_char(&mut f, b'\n').unwrap());
}

fn count_char<R: std::io::BufRead + ?Sized>(r: &mut R, delim: u8) -> Result<usize, std::io::Error> {
    let mut count = 0;
    loop {
        let used = {
            let available = match r.fill_buf() {
                Ok(n) => n,
                Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            };
            count += bytecount::count(available, delim);
            available.len()
        };
       
        r.consume(used);
        if used == 0 {
            return Ok(count);
        }
    }
}
