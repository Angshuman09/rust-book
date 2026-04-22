pub fn message_parsing() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hii");
        tx.send(val.clone()).unwrap();
        println!("{:?}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}")
}

pub fn multiple_producers(){
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec!["hii", "me", "hun", "doraemon"];

        for val in vals{
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for rec in rx{
        println!("{:?}", rec);
    }
}