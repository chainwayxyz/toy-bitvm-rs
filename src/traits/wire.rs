use bitcoin::Script;

pub trait WireTrait {
    fn create_commit_script(&self) -> Box<&Script>;
}
