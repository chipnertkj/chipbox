pub const fn source() -> &'static str {
    include_str!("../../../../node/chipbox-frontend/dist/main.js")
}

pub struct Runtime;

impl Runtime {
    pub async fn new() -> miette::Result<Self> {
        todo!()
    }
}
