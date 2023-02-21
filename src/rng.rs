#[cfg(target_pointer_width = "64")]
pub(crate) type SystemRng = rand_xoshiro::Xoshiro256PlusPlus;
#[cfg(not(target_pointer_width = "64"))]
pub(crate) type SystemRng = rand_xoshiro::Xoshiro256PlusPlus;
