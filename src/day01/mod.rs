pub mod solution {
    pub fn solve_challenge() -> String {
        let gift_message = String::from("Merry Christmas! Enjoy your gift!");

        // Simulate attaching message to present
        print_gift_message(&gift_message);

        //Return original message to prove ownership is maintained
        gift_message
    }

    fn print_gift_message(message: &String){
        println!("Attaching message to present: {}",message)
    }
}