#[cfg(target_pointer_width = "64")]
pub(crate) type InnerRng = rand_xoshiro::Xoshiro256PlusPlus;
#[cfg(not(target_pointer_width = "64"))]
pub(crate) type InnerRng = rand_xoshiro::Xoshiro256PlusPlus;
