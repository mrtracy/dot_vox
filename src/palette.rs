use nom::sequence::tuple;
use nom::{combinator::all_consuming, multi::many0, number::complete::le_u8, IResult};

lazy_static! {
  /// The default palette used by [MagicaVoxel](https://ephtracy.github.io/) -- this is supplied if no palette
  /// is included in the `.vox` file.
  pub static ref DEFAULT_PALETTE: Vec<Color> =
    include_bytes!("resources/default_palette.bytes")
        .chunks(4)
        .map(|bytes| parse_color(bytes).unwrap().1)
        .collect();
}

pub fn extract_palette(i: &[u8]) -> IResult<&[u8], Vec<Color>> {
    all_consuming(many0(parse_color))(i)
}

fn parse_color(input: &[u8]) -> IResult<&[u8], Color> {
    let (input, (r, g, b, a)) = tuple((le_u8, le_u8, le_u8, le_u8))(input)?;
    Ok((input, Color { r, g, b, a }))
}


#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<Color> for [u8; 4] {
    fn from(color: Color) -> Self {
        [color.r, color.g, color.b, color.a]
    }
}
impl From<&Color> for [u8; 4] {
    fn from(color: &Color) -> Self {
        [color.r, color.g, color.b, color.a]
    }
}

pub fn parse_imap(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
    all_consuming(many0(le_u8))(input)
}