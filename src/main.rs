use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug)]
struct CubeSat {
    id: u64,
    name: String,
    mailbox: Mailbox,
}
impl CubeSat {
    fn new(id: u64, name: String) -> Self {
        CubeSat {
            id: id,
            name: name,
            mailbox: Mailbox::new(),
        }
    }
    fn recv(&mut self) -> Option<Message> {
        self.mailbox.messages.pop()
    }
}

type Message = String;
#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}
impl Mailbox {
    fn new() -> Self {
        Mailbox { messages: vec![] }
    }
}
#[derive(Debug)]
struct GroundStation {
    radio_freq: f64,
}
impl GroundStation {
    fn new(r: f64) -> Self {
        GroundStation { radio_freq: r }
    }
    fn connect(&self, id: u64, name: String) -> CubeSat {
        CubeSat::new(id, name)
    }
    fn send(&self, to: &mut CubeSat, msg: Message) {
        to.mailbox.messages.push(msg)
    }
}
fn main() -> Result<(), String> {
    //Rcで所有権を共有、RefCellで可変可能にする
    let base = Rc::new(RefCell::new(GroundStation::new(118.000)));
    println!("base {:?}", base);

    {
        //base_2でbaseを可変借用する
        let mut base_2 = base.borrow_mut();
        base_2.radio_freq = 121.000;
        println!("base_2 {:?}", base_2);

        //baseは借用されてる旨が表示される
        println!("base {:?}", base);
    }

    //上記スコープのbase_2はドロップしたのでbaseは借用を外れた
    println!("base {:?}", base);

    ////base_3でbaseを可変借用する
    let mut base_3 = base.borrow_mut();
    base_3.radio_freq = 135.000;
    println!("base_3 {:?}", base_3);

    //baseは借用されてる旨が表示される
    println!("base {:?}", base);

    let mut sat_a = base_3.connect(1, String::from("Satelite_A"));
    let send_msg = Message::from("hello there");
    base_3.send(&mut sat_a, send_msg.clone());
    sat_a.recv().ok_or_else(|| Message::from("recv err"))?;

    Ok(())
}
