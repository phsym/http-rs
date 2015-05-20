#[macro_export]
macro_rules! option {
	($expr:expr, $msg:expr) => (match $expr {
		Some(val) => val,
		None => return Err(Error::new(ErrorKind::Other, $msg))
	})
}

#[macro_export]
macro_rules! map {
	(
		$($key:expr => $val:expr), *
	) => (
		{
			let mut m = std::collections::HashMap::new();
			$(
				m.insert($key, $val);
			)*
			m
		}
	)
}

#[macro_export]
macro_rules! debug {
	($fmt:expr) => (
		if cfg!(debug_assertions){
			println!(concat!("[DEBUG ", file!(), " line ", line!(), "] ", $fmt))
		}
	);
    ($fmt:expr, $($arg:tt)*) => (
    	if cfg!(debug_assertions){
    		println!(concat!("[DEBUG ", file!(), " line ", line!(), "] ", $fmt), $($arg)*)
		}
	);
}