# gpt-4-app
This app is reponsible for generating basic backend systems, CRUD operations using the Rust. A high level overview is that, it takes a prompt using a CLI, converts that to a function that is then consumed by gpt-4. It then generates a file as the basis of the backend system. I essentially pass around a piece of state (fact sheet) that the agents in my app use to communicate and update different stages as they complete the individual tasks they are given. 
To run -- cargo run 
Tests are in place to test each AI agent. 
