use crate::{gf, kg, mp};

// A chosen cyphertext attack takes an unknown message, and gets the recipient to decrypt it
// without him knowing he was decrypting that specific message

pub fn test() {
    let bob = kg::Keypair::new(64);

    let unknown_message = mp::Msg::new("Hello World").encrypt_blocks(11, bob.get_public());
    // Eve is doing stuff here, she does not know what the message originally was
    let r = gf::big(2);
    // Generate new message as follows: c' = c * r^e
    let chosen_message =
        &unknown_message.display()[0] * r.pow(gf::unbig(&bob.get_public().1) as u32);

    let decrypted_message = gf::int_to_str(&(bob.decrypt_num(&chosen_message) / &r));
    println!("{decrypted_message}");
}
