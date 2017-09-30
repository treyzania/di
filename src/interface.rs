use std::collections::HashMap;

use std::fs;
use std::io::{BufRead, BufReader, Lines};
use std::path::PathBuf;

use std::sync::mpsc;
use std::thread;

use discord;

use io;
use net;

pub struct DiState {
    root: PathBuf,
    chans: HashMap<discord::model::ChannelId, DiChannel>
}

impl DiState {

    pub fn new(root: PathBuf) -> Result<DiState, &'static str> {
        match fs::create_dir(root.clone()) {
            Ok(_) => unimplemented!(),
            Err(_) => Err("could not create output directory")
        }
    }

    pub fn add_channel(&mut self, si: &discord::model::LiveServer, ci: &discord::model::PublicChannel) -> Result<(), &'static str> {
        match DiChannel::new(&self, si.name.clone(), ci.name.clone()) {
            Ok(dc) => { self.chans.insert(ci.id, dc); Ok(()) },
            Err(r) => Err(r)
        }
    }

    pub fn remove_channel(&mut self, ci: discord::model::ChannelId) {
        self.chans.remove(&ci);
    }

    pub fn put_message(&mut self, msg: discord::model::Message) {
        match self.chans.get(&msg.channel_id) {
            _ => {}
        }
    }

}

struct InputMessage(String);

pub struct DiChannel {
    path: PathBuf,
    fout: io::FifoFile,
    fin: mpsc::Receiver<InputMessage>
}

impl DiChannel {
    fn new(ds: &DiState, mut sname: String, mut cname: String) -> Result<DiChannel, &'static str> {

        let mut pb = ds.root.clone();
        let mut nsn = String::default();
        for c in sname.chars() {
            nsn.push(if c.is_alphanumeric() { c } else { '-' });
        }

        pb.push(nsn);
        cname.insert(0, '#');
        pb.push(cname);

        match fs::create_dir_all(pb.clone()) {
            Ok(_) => match io::FifoFile::new_io_pair(pb.clone()) {
                Ok((fout, fin)) => Ok(DiChannel {
                    path: pb,
                    fout: fout,
                    fin: new_reader_thread(fin)
                }),
                Err(_) => Err("error creating fifos")
            },
            Err(_) => Err("error creating channel directory")
        }

    }
}

/// Spawns a new thread reading from the given fifo and writing to the other end of the returned
/// MPSC receiver.  This isn't the best way to do it as the thread will be blocked if we're
/// waiting for the end of a line, which we usually are.
fn new_reader_thread(fin: io::FifoFile) -> mpsc::Receiver<InputMessage> {

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || for le in BufReader::new(fin).lines() {
        match le {
            Ok(l) => match tx.send(InputMessage(l)) {
                Ok(_) => {},
                Err(_) => return // The other end is closed, we can exit.
            },
            Err(_) => {}
        }
    });

    rx

}

impl Drop for DiChannel {
    fn drop(&mut self) {
        match fs::remove_dir_all(self.path.clone()) {
            Ok(_) => {},
            Err(_) => println!("warning: could not remove dir")
        }
    }
}
