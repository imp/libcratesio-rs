use api::ErrorResponse;

error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Reqwest(::reqwest::Error);
        Serde(::serde_json::Error);
    }

    errors {
        CratesIOError(err: ErrorResponse) {
            description("crates.io error")
            display("crates.io returns error '{}'", err.detail())
        }
    }
}
