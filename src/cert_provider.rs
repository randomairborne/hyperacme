pub trait CertProvider {
    fn directory() -> impl AsRef<str>;
}
