use std::io::{Read, Write, IntoInnerError};

/// Any type which can be adapted over a Read type.
pub trait ReadAdapter<R: Read> {
    /// Wrap a Read type in this adapter.
    fn wrap(reader: R) -> Self;

    /// Unwrap this type to get its inner Read. If this action could fail, this call should panic
    /// on fail.
    fn into_inner(self) -> R;

    /// Try to unwrap this type. If this action could fail, it should yield an IntoInnerError if
    /// it fails. This method is implemented by default on the assumption that into_inner cannot
    /// fail; if it can, this method needs to be correctly implemented.
    fn try_into_inner(self) -> Result<R, IntoInnerError<Self>> where Self: Sized {
        Ok(self.into_inner())
    }
}

/// Any type which can be adapted over a Write type.
pub trait WriteAdapter<W: Write> {
    /// Wrap a Write type in this adapter.
    fn wrap(writer: W) -> Self;

    /// Unwrap this type to get its inner Write. If this action could fail, this call should panic
    /// on fail.
    fn into_inner(self) -> W;

    /// Try to unwrap this type. If this action could fail, it should yield an IntoInnerError if
    /// it fails. This method is implemented by default on the assumption that into_inner cannot
    /// fail; if it can, this method needs to be correctly implemented.
    fn try_into_inner(self) -> Result<W, IntoInnerError<Self>> where Self: Sized {
        Ok(self.into_inner())
    }
}

mod _std {
    use std::io::{self, Read, Write};
    use {ReadAdapter, WriteAdapter};

    impl<R: Read> ReadAdapter<R> for io::BufReader<R> {
        fn wrap(reader: R) -> Self {
            io::BufReader::new(reader)
        }

        fn into_inner(self) -> R {
            self.into_inner()
        }
    }

    impl<W: Write> WriteAdapter<W> for io::BufWriter<W> {
        fn wrap(writer: W) -> Self {
            io::BufWriter::new(writer)
        }

        fn into_inner(self) -> W {
            match self.into_inner() {
                Ok(writer)  => writer,
                Err(error)  => panic!("Failed to unwrap BufWriter: {:?}", error.error()),
            }
        }


        fn try_into_inner(self) -> Result<W, io::IntoInnerError<Self>> {
            self.into_inner()
        }
    }
}

mod _serde_json {
    use std::io::Write;
    use WriteAdapter;

    extern crate serde_json as json;

    impl<W: Write> WriteAdapter<W> for json::Serializer<W> {
        fn wrap(writer: W) -> Self {
            json::Serializer::new(writer)
        }

        fn into_inner(self) -> W {
            self.into_inner()
        }
    }
}
