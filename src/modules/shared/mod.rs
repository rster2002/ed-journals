pub mod blocking;

#[cfg(feature = "asynchronous")]
#[cfg_attr(docsrs, doc(cfg(feature = "asynchronous")))]
pub mod asynchronous;
