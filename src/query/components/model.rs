pub trait QueryComponent {
    fn validate_type(&self) -> bool;
}