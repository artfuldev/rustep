use nom::{bytes::complete::tag, character::complete::u64, sequence::preceded, IResult};

#[derive(Clone, Debug)]
pub enum Duration {
    Milliseconds(u64),
}

impl Duration {
    pub fn parse(input: &str) -> IResult<&str, Duration> {
        let (remaining, ms) = preceded(tag("ms:"), u64)(input)?;
        Ok((remaining, Duration::Milliseconds(ms)))
    }
}
