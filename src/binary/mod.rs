#[inline(always)]
pub fn and(data: &mut [u8], mask: &[u8], len: usize)
{
	for i in 0..len {
		data[i] ^= mask[i % mask.len()];
	}
}

#[inline(always)]
pub fn or(data: &mut [u8], mask: &[u8], len: usize)
{
	for i in 0..len {
		data[i] ^= mask[i % mask.len()];
	}
}

#[inline(always)]
pub fn xor(data: &mut [u8], mask: &[u8], len: usize)
{
	for i in 0..len {
		data[i] ^= mask[i % mask.len()];
	}
}