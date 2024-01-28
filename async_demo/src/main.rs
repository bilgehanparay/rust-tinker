use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

#[tokio::main]
async fn main(){
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    // Accept loop
    loop{
        println!("loop 1");
        let(socket, _) = listener.accept().await.unwrap();
        println!("loop 2");
        // Tasks require single allocation and 64kbytes of memory
        // Tasks should not contain any references to data
        // owned outside the task, thats the reason of move!
        // But cannot use socket after 'spawn'
        // If the data has to be shared between tasks, We must use
        // synch interfaces such as Arc
        tokio::spawn(async move{
            process(socket).await;
        });
        println!("loop 3");
    }
}

async fn process(socket: TcpStream){
    println!("Process");
    let mut connection = Connection::new(socket);
    if let Some(frame) = connection.read_frame().await.unwrap(){
        println!("GOT: {:?}", frame);

        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}