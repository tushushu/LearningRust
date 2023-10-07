fn main() {
    let m = Message::Move(MoveMessage { x: 1, y: 2 });
    println!("{:?}", m);
}

#[derive(Debug)]
enum Message {
    Quit(QuitMessage),
    Move(MoveMessage),
    Write(WriteMessage),
    ChangeColor(ChangeColorMessage),
}

#[derive(Debug)]
struct QuitMessage;
#[derive(Debug)]
struct MoveMessage {
    x: i32,
    y: i32,
}
#[derive(Debug)]
struct WriteMessage(String);
#[derive(Debug)]
struct ChangeColorMessage(i32, i32, i32);
