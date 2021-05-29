use crate::*;
use std::sync::mpsc;

#[derive(Debug, Clone)]
pub struct Link<Message> {
    sender: mpsc::Sender<Message>,
}

pub(crate) struct LinkReceiver<Message> {
    pub(crate) receiver: mpsc::Receiver<Message>,
}

pub(crate) fn create_link<Message>() -> (Link<Message>, LinkReceiver<Message>) {
    let (sender, receiver) = mpsc::channel();

    let link = Link {
        sender,
    };

    let link_receiver = LinkReceiver {
        receiver,
    };

    (link, link_receiver)
}
