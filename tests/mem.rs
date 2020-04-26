#[cfg(test)]
mod test {
	use::libhermit_rs_bench::mem;

	#[test]
	fn memcmp_single_byte_pointers() {
		unsafe {
			assert_eq!(mem::cmp(&0xFAu8, &0xFAu8, 1), 0x00);
			assert!(mem::cmp(&0xEFu8, &0xFEu8, 1) < 0x00);
		}
	}

	#[test]
	fn memcmp_strings() {
		{
			let (x, z) = ("Hello!", "Good Bye.");
			let l = x.len();
			unsafe {
				assert_eq!(mem::cmp(x.as_ptr(), x.as_ptr(), l), 0);
				assert!(mem::cmp(x.as_ptr(), z.as_ptr(), l) > 0);
				assert!(mem::cmp(z.as_ptr(), x.as_ptr(), l) < 0);
			}
		}
		{
			let (x, z) = ("hey!", "hey.");
			let l = x.len();
			unsafe {
				assert!(mem::cmp(x.as_ptr(), z.as_ptr(), l) < 0);
			}
		}
	}

	#[test]
	fn memset_single_byte_pointers() {
		let mut x: u8 = 0xFF;
		unsafe {
			mem::set(&mut x, 0xAA, 1);
			assert_eq!(x, 0xAA);
			mem::set(&mut x, 0x00, 1);
			assert_eq!(x, 0x00);
			x = 0x01;
			mem::set(&mut x, 0x12, 0);
			assert_eq!(x, 0x01);
		}
	}

	#[test]
	fn memset_array() {
		let mut buffer = [b'X'; 100];
		unsafe {
			mem::set(buffer.as_mut_ptr(), b'#' as i32, buffer.len());
		}
		for byte in buffer.iter() {
			assert_eq!(*byte, b'#');
		}
	}

	#[test]
	fn memcpy_and_memcmp_arrays() {
		let (src, mut dst) = ([b'X'; 100], [b'Y'; 100]);
		unsafe {
			assert!(mem::cmp(src.as_ptr(), dst.as_ptr(), 100) != 0);
			let _ = mem::cpy(dst.as_mut_ptr(), src.as_ptr(), 100);
			assert_eq!(mem::cmp(src.as_ptr(), dst.as_ptr(), 100), 0);
		}
	}

	#[test]
	fn memmove_overlapping() {
		{
			let mut buffer = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'];
			unsafe {
				mem::mve(&mut buffer[4], &buffer[0], 6);
				let mut i = 0;
				for byte in b"0123012345".iter() {
					assert_eq!(buffer[i], *byte);
					i += 1;
				}
			}
		}
		{
			let mut buffer = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'];
			unsafe {
				mem::mve(&mut buffer[0], &buffer[4], 6);
				let mut i = 0;
				for byte in b"4567896789".iter() {
					assert_eq!(buffer[i], *byte);
					i += 1;
				}
			}
		}
	}
}
