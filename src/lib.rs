use std::str::FromStr;
use std::isize;

#[derive(Debug, PartialEq)]
pub struct PluralOperands {
	pub n: f64,
	pub i: isize,
	pub v: isize,
	pub w: isize,
	pub f: isize,
	pub t: isize,
	pub int: isize,
	pub dec: isize,
}

impl PluralOperands {
	pub fn new<S: ToString>(num: S) -> Self {
		let mut str_num_temp: String = num.to_string();

		println!("{:?}", str_num_temp);

		if str_num_temp.starts_with("-") {
			str_num_temp.remove(0);
		}

		let str_num = str_num_temp;

		let v: Vec<&str> = str_num.split('.').collect();
		
		if v.len() > 2 {
			panic!("Incorrect number passed!");
		}

		let front = v.get(0).unwrap();
		let back = v.get(1).unwrap_or(&"");

		let absolute_value = f64::from_str(&str_num).unwrap();
		let integer_digits = isize::from_str(front).unwrap();
		// let integer_digits = front.chars().count() as isize;
		let mut num_fraction_digits0 = 0;
		let mut num_fraction_digits = 0;
		let mut fraction_digits0 = 0;
		let mut fraction_digits = 0;

		if v.len() == 2 {
			let mut backtrace = String::from_str(&back).unwrap();
			while backtrace.ends_with("0") {
				backtrace.pop();
			}

			num_fraction_digits0 = back.chars().count() as isize;
			num_fraction_digits = backtrace.chars().count() as isize;
			fraction_digits0 = isize::from_str(back).unwrap();
			fraction_digits = isize::from_str(&backtrace).unwrap_or(0);
		} 

		PluralOperands {
			n: absolute_value,
			i: integer_digits,
			v: num_fraction_digits0,
			w: num_fraction_digits,
			f: fraction_digits0,
			t: fraction_digits,
			int: front.parse().unwrap(),
			dec: back.parse().unwrap_or(0)
		}
	}
}
