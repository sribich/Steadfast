use std::sync::mpsc::{Sender, Receiver};

pub enum Message {
    Exit {
        force: bool,
    },
}

pub(crate) struct Editor {
    pub(crate) sender: Sender<Message>,
    pub(crate) receiver: Receiver<Message>,

    pub(crate) exit: bool,
}

impl Editor {
    pub(crate) fn new() -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();

        Editor {
            sender,
            receiver,
            exit: false,
        }
    }

    pub(crate) fn update(&mut self) {
        while let Ok(message) = self.receiver.try_recv() {
            match message {
                Message::Exit { force } => {
                    self.exit = true;
                }
            }
        }
    }
}
