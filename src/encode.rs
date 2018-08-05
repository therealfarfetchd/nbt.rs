//! Encode NBT values and write them to files or other writable sinks.

use super::{Error, Result, Tag, Encodable, Compression};

use std::fs::OpenOptions;
use std::io::Write;

use flate2::write::GzEncoder;
use flate2;


fn write_primitive<W: Write, T: Encodable>(writer: &mut W, i: T) -> Result<()> {
    Ok(writer.write(&i.to_bytes_nbt()).map(|_| ())?)
}

fn write_string<W: Write>(writer: &mut W, s: &str) -> Result<()> {
    write_primitive(writer, s.len() as i16)?;

    Ok(writer.write(s.as_bytes()).map(|_| ())?)
}

fn write_value<W: Write>(writer: &mut W, tag: &Tag) -> Result<()> {
    match *tag {
        Tag::End       => return Err(Error::Invalid),
        Tag::Byte(x)   => write_primitive(writer, x)?,
        Tag::Short(x)  => write_primitive(writer, x)?,
        Tag::Int(x)    => write_primitive(writer, x)?,
        Tag::Long(x)   => write_primitive(writer, x)?,
        Tag::Float(x)  => write_primitive(writer, x)?,
        Tag::Double(x) => write_primitive(writer, x)?,

        Tag::ByteArray(ref x) => {
            write_primitive(writer, x.len() as i32)?;
            writer.write(&*x)?;
        },

        Tag::String(ref x) => write_string(writer, x)?,
        Tag::List(ref x) => {
            write_primitive(writer, x.element_type.to_binary() as i8)?;
            write_primitive(writer, x.elements.len() as i32)?;

            for i in x.elements.iter() {
                write_value(writer, i)?;
            }

        },

        Tag::Compound(ref x) => {
            for (nam, val) in x.elements.iter() {
                write_tag(writer, (nam, val))?;
            }

            write_primitive(writer, 0_i8)?;
        },

        Tag::IntArray(ref x) => {
            write_primitive(writer, x.len() as i32)?;

            for i in x {
                write_primitive(writer, *i)?;
            }
        }
    };

    Ok(())
}

fn write_tag<W: Write>(writer: &mut W, tag: (&str, &Tag)) -> Result<()> {
    write_primitive(writer, tag.1.get_type().to_binary() as i8)?;
    write_string(writer, tag.0)?;
    write_value(writer, tag.1)?;

    Ok(writer.flush()?)
}

/// Encode NBT tags.
pub struct Encoder {
    writer: Box<Write>
}

// TODO: get rid of the box
impl Encoder {
    /// Create a new Encoder from an existing writer that will be taken
    /// ownership over.
    pub fn from_writer<W: Write + 'static>(writer: W) -> Encoder {
        Encoder {
            writer: Box::new(writer)
        }
    }

    /// Create a new Encoder for the given file `file`, with the given
    /// compression method.
    pub fn from_file(file: &str, c: Compression) -> Result<Encoder> {
        Ok(Encoder {
            writer: match c {
                Compression::Uncompressed =>
                    Box::new(OpenOptions::new()
                      .create(true)
                      .truncate(true)
                      .write(true)
                      .open(file)?),

                Compression::GZip =>
                    Box::new(GzEncoder::new(
                        OpenOptions::new()
                          .create(true)
                          .truncate(true)
                          .write(true)
                          .open(file)?,
                        flate2::Compression::default()))
            }
        })
    }

    /// Write a named tag to the stream.
    pub fn write_tag(&mut self, tag: (&str, &Tag)) -> Result<()> {
        write_tag(&mut self.writer, tag)
    }
}
