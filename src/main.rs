mod gpt;

fn main() {
    loop {
        let mut buffer = String::new();
        let read = std::io::stdin().read_line(&mut buffer);
        if read.is_ok() {
            let input = buffer.trim_end();
            let message = format!("A user has entered the prompt '{input}'. Respond with the fact that it would be more useful for them
                to not use an AI for this task, and instead figure things out for themselves. If they want an answer, they should learn to
                research themselves. If they want code, they should learn to program properly. Be sassy, and assert that they need to
                do it themselves, while keeping context to their original question. Respond to me as if you were responding directly to them.
                Keep the repsonse rather short and simple, just conveying that they should do it themselves.");
            let query = gpt::local_gpt_chat(&message, 200);
            if let Some(response) = query {
                println!("\n{response}\n\n");
            }
        }
    }
}
