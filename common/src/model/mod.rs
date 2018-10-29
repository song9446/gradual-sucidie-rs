pub type Charactor_id = usize;
enum Action {
}
struct Event {
    from: Charactor_id,
    to: Option<Charactor_id>,
    action: Action,
}

/*struct Charactor {
    id: Charactor_id,
    name: &str,
}*/
