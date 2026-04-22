pub fn spawn_thread(){
     let handle = thread::spawn(||{
       for i in 1..10{
        println!("this is spawned thread and i: {i}");
        thread::sleep(Duration::from_secs(1));
       }
    });

    handle.join().unwrap();

    for i in 1..5{
        println!("this is main thread and i: {i}");
        thread::sleep(Duration::from_secs(1));
    }

    // handle.join().unwrap();
}

pub fn move_keyword(){
    let v = vec![1,2,3,4];

    let handle = thread::spawn(move ||{
        println!("{:?}", v);
    });

    handle.join().unwrap();
}
