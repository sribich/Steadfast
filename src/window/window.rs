use crate::instance::extensions::InstanceExtensions;

pub fn required_extensions() -> InstanceExtensions {
    let requested = InstanceExtensions {
        ..InstanceExtensions::none()
    };

    match InstanceExtensions::supported() {
        Ok(supported) => supported.intersection(&requested),
        Err(_) => InstanceExtensions::none()
    }
}
