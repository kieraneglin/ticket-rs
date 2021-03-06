#[macro_use]
extern crate qml;
extern crate ticket;

mod controllers;

use self::ticket::*;
use std::io::{stdin, Write, stdout};
use self::ticket::models::*;
use qml::*;

pub fn main() {
    show(*Ticket::all());
    // let mut qqae = QmlEngine::new();
    // let mut qalm = QListModel::new(&["title", "desc", "timestamp"]);
    // qalm.append_row(qvarlist![controllers::tickets::Tickets.index()].into_iter());
    // qqae.set_property("listModel", &qalm.get_qvar());
    // qqae.load_file("src/views/main.qml");
    // qqae.exec();
    // qqae.quit();

    // loop {
    //     print!("What action do you want to perform? (q to quit): ");
    //     stdout().flush().unwrap();
    //     let mut answer = String::new();
    //     stdin().read_line(&mut answer).unwrap();
    //
    //     let answer = answer.trim_right();
    //
    //     match answer {
    //         "index" => controllers::tickets::index(),
    //         "create" => controllers::tickets::create(),
    //         "q" => break,
    //         _ => println!("I didn't get that"),
    //
    //     }
    // }

    // println!("{:#?}", Ticket::create("test", "test"));
}

fn show(gathered: Vec<Ticket>) {
    let mut engine = QmlEngine::new();
    let list = form_list(&gathered);
    let qvar: QVariant = list.get_qvar();

    engine.set_property("tickets", &qvar);
    engine.load_data(include_str!("views/main.qml"));
    engine.exec();
}

fn form_list(gathered: &[Ticket]) -> QTicketList {
    let mut qalm = QTicketList::new();
    qalm.set_data(Ticket::to_qformat(gathered));
    qalm
}

Q_LISTMODEL!{
    pub QTicketList {
        title: String,
        description: String,
        created_at: String
    }
}
