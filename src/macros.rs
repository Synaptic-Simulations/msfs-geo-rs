#[macro_export]
macro_rules! assert_about_eq {
    ($a:expr, $b:expr, $eps:expr $(,)?) => {{
		let a = $a;
		let b = $b;
		assert!((a - b).abs() < $eps, "{} != {} ({} != {})", stringify!($a), stringify!($b), a, b);
	}};

	($a:expr, $b:expr, $eps:expr, $fmt:literal, $($args:tt)*) => {{
		let a = $a;
		let b = $b;
		assert!((a - b).abs() < $eps, $fmt, $($args)*);
	}};

	($a:expr, $b:expr $(, $fmt:literal, $($args:tt)*)?) => {
		$crate::assert_about_eq!($a, $b, 1e-6, $($fmt,  $($args)*)?);
	};
}

#[macro_export]
macro_rules! assert_uom_eq {
	($a:expr, $b:expr $(, $eps:expr)? $(,)?) => {{
		let a = $a.value;
		let b = $b.value;
		$crate::assert_about_eq!(a, b $(, $eps)?, "{} != {} ({} != {})", stringify!($a), stringify!($b), a, b);
	}};

	($a:expr, $b:expr $(, $eps:expr)?, $($message:tt)*) => {
		$crate::assert_about_eq!($a.value, $b.value $(, $eps)?, $($message)*);
	};
}
