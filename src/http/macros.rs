/// Unwrap an `Option<T>` or return an `Err` if it's None
#[macro_export]
macro_rules! option {
	($expr:expr, $msg:expr) => (match $expr {
		Some(val) => val,
		None => return Err(Error::new(ErrorKind::Other, $msg))
	})
}

/// Quickly build a `std::collections::HashMap`
///
/// This macro is quite similar to the `vec!` macro but is used to build
/// a `HashMap`. The types will be deduced from the arguments.
/// # Example
/// ```ignore
/// let my_map = map!{"a" => 1, "b" => 2};
/// // Is similar to :
/// use std::collections::HashMap;
/// let mut my_map: HashMap<&str, i32> = HashMap::new();
/// my_map.insert("a", 1);
/// my_map.insert("b", 2);
/// ```
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

/// Print debug message only if built in debug mode
///
/// This macro will print given message if `debug_assertions` build configuration is set.
/// The message will be formated with its additional arguments with `format!` macro and will
/// contain a header with file name and line number
/// # Example
/// ```ignore
/// debug!("This is a debug message");
/// debug!("This is a debug {}", "message");
/// ```
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

/// Return a String if conversion is possible by calling to_string() on the provided expression
#[macro_export]
macro_rules! str {
	($expr:expr) => ($expr.to_string());
}

/// Build a `std::collections::HashMap` the same way the `map!` macro does, except
/// that all keys and values are converted to `String` with `str!` macro
#[macro_export]
macro_rules! smap {
	(
		$($key:expr => $val:expr), *
	) => (
		map!($(str!($key) => str!($val)), *)
	)
}