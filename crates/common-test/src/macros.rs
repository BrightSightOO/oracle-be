#[doc = "[//]: <> (setup! {})"]
#[macro_export]
macro_rules! setup {
    ($($tt:tt)*) => {
        $crate::log::trace!(target: "test", "===== begin setup =====");

        $($tt)*

        $crate::log::trace!(target: "test", "===== end setup =====");
    };
}
