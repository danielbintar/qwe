use ws::{Frame, Handler, Sender, Handshake, Result, Message};

pub struct Client<'a> {
    out: Sender,
    tx: &'a std::sync::mpsc::Sender<String>,
    rx: &'a std::sync::mpsc::Receiver<String>,
}

impl<'a> Client<'a> {
    pub fn new(out: Sender, tx: &'a std::sync::mpsc::Sender<String>, rx: &'a std::sync::mpsc::Receiver<String>) -> Self {
        Self {
            out,
            tx,
            rx,
        }
    }
}

impl<'a> Handler for Client<'a> {
    fn on_frame(&mut self, frame: Frame) -> Result<Option<Frame>> {
        let received = self.rx.try_recv();
        match received {
            Ok(msg) => {
                self.out.send(msg).unwrap();
            },
            Err(_) => { self.out.ping(Vec::new()).unwrap(); }
        }
        Ok(Some(frame))
    }

    fn on_open(&mut self, _: Handshake) -> Result<()> {
        self.out.ping(Vec::new()).unwrap();
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        self.tx.send(msg.to_string()).unwrap();
        Ok(())
    }
}
