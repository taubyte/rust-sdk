use super::Channel;

impl Channel {
    pub fn new(name: String) -> Result<Self, Box<dyn std::error::Error>> {
        if name.len() == 0 {
            Err("Channel name cannot be empty".into())
        } else {
            Ok(Self { name })
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

#[cfg(test)]
pub mod test {
    use super::Channel;

    #[test]
    fn channel_name() {
        let name = "testChannel".to_string();
        let channel = Channel::new(name.clone()).unwrap();
        assert_eq!(channel.name(), name);

        let err_name = "".to_string();
        let err = Channel::new(err_name);
        assert_eq!(err.is_err(), true);
    }
}
