mod tracker;
use tracker::Tracker;

fn main() {
    let tracker = Tracker::new("127.0.0.1".to_owned(), 8080);
    tracker.start();
}
