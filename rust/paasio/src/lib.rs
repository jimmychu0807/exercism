use std::io::{Read, Result, Write};

pub struct ReadStats<R: Read> {
	reader: R,
	num_read: usize,
	bytes_through: usize,
}

impl<R: Read> ReadStats<R> {
	// _wrapped is ignored because R is not bounded on Debug or Display and therefore
	// can't be passed through format!(). For actual implementation you will likely
	// wish to remove the leading underscore so the variable is not ignored.
	pub fn new(wrapped: R) -> ReadStats<R> {
		Self { reader: wrapped, num_read: 0, bytes_through: 0 }
	}

	pub fn get_ref(&self) -> &R {
		&self.reader
	}

	pub fn bytes_through(&self) -> usize {
		self.bytes_through
	}

	pub fn reads(&self) -> usize {
		self.num_read
	}
}

impl<R: Read> Read for ReadStats<R> {
	fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
		let bytes_read = self.reader.read(buf)?;
		self.bytes_through += bytes_read;
		self.num_read += 1;

		Ok(bytes_read)
	}
}

pub struct WriteStats<W: Write> {
	writer: W,
	num_write: usize,
	bytes_through: usize,
}

impl<W: Write> WriteStats<W> {
	// _wrapped is ignored because W is not bounded on Debug or Display and therefore
	// can't be passed through format!(). For actual implementation you will likely
	// wish to remove the leading underscore so the variable is not ignored.
	pub fn new(wrapped: W) -> WriteStats<W> {
		Self { writer: wrapped, num_write: 0, bytes_through: 0 }
	}

	pub fn get_ref(&self) -> &W {
		&self.writer
	}

	pub fn bytes_through(&self) -> usize {
		self.bytes_through
	}

	pub fn writes(&self) -> usize {
		self.num_write
	}
}

impl<W: Write> Write for WriteStats<W> {
	fn write(&mut self, buf: &[u8]) -> Result<usize> {
		let bytes_write = self.writer.write(buf)?;
		self.bytes_through += bytes_write;
		self.num_write += 1;

		Ok(bytes_write)
	}

	fn flush(&mut self) -> Result<()> {
		self.writer.flush()
	}
}
