use core::identity::Identity;

fn main() {
    let id = Identity::generate();
    print!("Id: {:?}", id.peer_id());
}
