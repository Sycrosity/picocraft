/// Extends [`Write`] with methods for writing numbers. (For `std::io`.)
///
/// Most of the methods defined here have an unconstrained type parameter that
/// must be explicitly instantiated. Typically, it is instantiated with either
/// the [`BigEndian`] or [`LittleEndian`] types defined in this crate.
///
/// # Examples
///
/// Write unsigned 16 bit big-endian integers to a [`Write`]:
///
/// ```rust
/// use byteorder::{BigEndian, WriteBytesExt};
///
/// let mut wtr = vec![];
/// wtr.write_u16::<BigEndian>(517).unwrap();
/// wtr.write_u16::<BigEndian>(768).unwrap();
/// assert_eq!(wtr, vec![2, 5, 3, 0]);
/// ```
///
/// [`BigEndian`]: enum.BigEndian.html
/// [`LittleEndian`]: enum.LittleEndian.html
/// [`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
#[allow(async_fn_in_trait)]
pub trait WriteBytesExt: embedded_io_async::Write {
    /// Writes an unsigned 8 bit integer to the underlying writer.
    ///
    /// Note that since this writes a single byte, no byte order conversions
    /// are used. It is included for completeness.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    ///
    /// # Examples
    ///
    /// Write unsigned 8 bit integers to a `Write`:
    ///
    /// ```rust
    /// use byteorder::WriteBytesExt;
    ///
    /// let mut wtr = Vec::new();
    /// wtr.write_u8(2).unwrap();
    /// wtr.write_u8(5).unwrap();
    /// assert_eq!(wtr, b"\x02\x05");
    /// ```
    #[inline]
    async fn write_u8(&mut self, n: u8) -> Result<(), Self::Error> {
        self.write_all(&[n]).await
    }

    /// Writes a signed 8 bit integer to the underlying writer.
    ///
    /// Note that since this writes a single byte, no byte order conversions
    /// are used. It is included for completeness.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    ///
    /// # Examples
    ///
    /// Write signed 8 bit integers to a `Write`:
    ///
    /// ```rust
    /// use byteorder::WriteBytesExt;
    ///
    /// let mut wtr = Vec::new();
    /// wtr.write_i8(2).unwrap();
    /// wtr.write_i8(-5).unwrap();
    /// assert_eq!(wtr, b"\x02\xfb");
    /// ```
    #[inline]
    async fn write_i8(&mut self, n: i8) -> Result<(), Self::Error> {
        self.write_all(&[n as u8]).await
    }
}

/// Extends [`Read`] with methods for reading numbers. (For `std::io`.)
///
/// Most of the methods defined here have an unconstrained type parameter that
/// must be explicitly instantiated. Typically, it is instantiated with either
/// the [`BigEndian`] or [`LittleEndian`] types defined in this crate.
///
/// # Examples
///
/// Read unsigned 16 bit big-endian integers from a [`Read`]:
///
/// ```rust
/// use std::io::Cursor;
///
/// use byteorder::{BigEndian, ReadBytesExt};
///
/// let mut rdr = Cursor::new(vec![2, 5, 3, 0]);
/// assert_eq!(517, rdr.read_u16::<BigEndian>().unwrap());
/// assert_eq!(768, rdr.read_u16::<BigEndian>().unwrap());
/// ```
///
/// [`BigEndian`]: enum.BigEndian.html
/// [`LittleEndian`]: enum.LittleEndian.html
/// [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
#[allow(async_fn_in_trait)]
pub trait ReadBytesExt: embedded_io_async::Read {
    /// Reads an unsigned 8 bit integer from the underlying reader.
    ///
    /// Note that since this reads a single byte, no byte order conversions
    /// are used. It is included for completeness.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    ///
    /// # Examples
    ///
    /// Read unsigned 8 bit integers from a `Read`:
    ///
    /// ```rust
    /// use std::io::Cursor;
    ///
    /// use byteorder::ReadBytesExt;
    ///
    /// let mut rdr = Cursor::new(vec![2, 5]);
    /// assert_eq!(2, rdr.read_u8().unwrap());
    /// assert_eq!(5, rdr.read_u8().unwrap());
    /// ```
    #[inline]
    async fn read_u8(&mut self) -> Result<u8, embedded_io::ReadExactError<Self::Error>> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf).await?;
        Ok(buf[0])
    }

    /// Reads a signed 8 bit integer from the underlying reader.
    ///
    /// Note that since this reads a single byte, no byte order conversions
    /// are used. It is included for completeness.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    ///
    /// # Examples
    ///
    /// Read signed 8 bit integers from a `Read`:
    ///
    /// ```rust
    /// use std::io::Cursor;
    ///
    /// use byteorder::ReadBytesExt;
    ///
    /// let mut rdr = Cursor::new(vec![0x02, 0xfb]);
    /// assert_eq!(2, rdr.read_i8().unwrap());
    /// assert_eq!(-5, rdr.read_i8().unwrap());
    /// ```
    #[inline]
    async fn read_i8(&mut self) -> Result<i8, embedded_io::ReadExactError<Self::Error>> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf).await?;
        Ok(buf[0] as i8)
    }
}

/// All types that implement `Read` get methods defined in `ReadBytesExt`
/// for free.
impl<R: embedded_io_async::Read + ?Sized> ReadBytesExt for R {}

/// All types that implement `Write` get methods defined in `WriteBytesExt`
/// for free.
impl<W: embedded_io_async::Write + ?Sized> WriteBytesExt for W {}
