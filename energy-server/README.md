# Energy Server!

I decided to do this project in Rust due to personal interest and thinking it would be the best tool for the job especially for performance and future scalability. I utilized the Iron web framework and the serde serialization/deserialization library.

There is some error handling but I did not defend against malicious input so passing incorrect datatypes as parameters might crash the server.

# Building

First you'll need to install the latest stable Rust compiler from here if you don't already have it: https://www.rust-lang.org/en-US/install.html

I tested on 1.29.1 stable on both Mac and Windows.

Once that's installed, just run `cargo run` from inside this directory (same level as the Cargo.toml file and the src directory) and it should pull and compile all dependencies and run the server on port 8080

This will set your working directory and the server will be able to find the json file there.
