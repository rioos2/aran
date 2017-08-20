bitflags! {
    pub struct FeatureFlags: u32 {
        const ADMIN = 0b00000001;
        const SERVICE_ACCESS = 0b00000010;
        const DEFAULT_ACCESS = 0b00000100;
    }
}
