fn main() {
    let gift_message = String::from("Merry Christmas! Enjoy your gift!");

    attach_message_to_present(&gift_message);

    println!("Original message is still here: {}",gift_message);

}

fn attach_message_to_present(message: &String){
    println!("The present now has this message: {}",message);
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_message_ownership() {
        let message = String::from("Test message");
        attach_message_to_present(&message);
        assert_eq!(message,"Test message");
    }
}