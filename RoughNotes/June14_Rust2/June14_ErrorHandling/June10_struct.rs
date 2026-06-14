
trait Greet {
    fn greet(&self);
}

fn make_letter<T:Greet> (obj:T) {
    obj.greet();
    println!("you are welcome conference!!!! at bangalore" );  
}

#[derive(Debug)]
struct FriendNode {
    name: String,
    age : u8,
    friends : Vec<String>
}

impl Greet for FriendNode {
    fn greet(&self) 
    {
        println!("Hello {}", self.name);
    }
}

impl FriendNode {
    fn new(name_input: &str) -> FriendNode {
        return FriendNode{
                age: 30,
                friends: Vec::new(),
                name: name_input.to_string()}; 
    }

    fn add_friend(&mut self, name:&str) {
        self.friends.push(name.to_string());
    }

    // add member function to print name //
}

fn main() {
    let mut david = FriendNode {
                name: "Davide".to_string(),
                age: 60,
                friends: Vec::new()};
    david.age += 1;

    let mut john = FriendNode {
                name: "John".to_string(),
                friends: Vec::new(),
                .. david};

    println!("{:?}", david);

    let mut sri = FriendNode::new("Srinivas A");
    sri.add_friend("kamal");
    println!("sri = {:?}", sri);

    sri.greet();
    make_letter(sri);
}
