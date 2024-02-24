pub enum JavaDistribution {
    OpenJdk,
}

// maybe is should have multiple struct depending on what i want or i can
// use underlying lazyly initialize attribute on this struct
// like checking if the candidate is the curent one or if it is installed
pub struct JavaCandidate {
    version: u16,
    exact_version: u16,
    download_url: Option<String>,
    latest: bool,
    distribution: JavaDistribution,
}
