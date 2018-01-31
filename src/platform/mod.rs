cfg_if! {
    //TODO: Android and iOS would be *really* nice.

    if #[cfg(target_os = "windows")] {
        mod windows;
        pub use self::windows::*;
    } else if #[cfg(target_os = "macos")] {
    	mod macos;
    	pub use self::macos::*;
    } else {
        mod gtk;
        pub use self::gtk::*;
    }
}
