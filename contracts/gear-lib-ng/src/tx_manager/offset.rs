use core::num::TryFromIntError;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub(crate) struct Offset(u32);

impl Offset {
    pub(crate) const MAX: u32 = u32::MAX / u8::MAX as u32 - 1;

    pub(crate) const fn wrapping_add(self, rhs: Self) -> Self {
        Self((self.0 + rhs.0) % (Self::MAX + 1))
    }

    pub(crate) const fn wrapping_sub(self, rhs: Self) -> Self {
        let number = self.0.wrapping_sub(rhs.0);

        Self(if number > Self::MAX {
            Self::MAX - (u32::MAX - number)
        } else {
            number
        })
    }

    pub(crate) const fn get(self) -> u32 {
        self.0
    }
}

impl TryFrom<u32> for Offset {
    type Error = TryFromIntError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value > Self::MAX {
            Err(get_try_from_int_error())
        } else {
            Ok(Self(value))
        }
    }
}

fn get_try_from_int_error() -> TryFromIntError {
    u8::try_from(256u16).unwrap_err()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrapping_add() {
        assert_eq!(Offset(123).wrapping_add(Offset(321)), Offset(444));
        assert_eq!(Offset(Offset::MAX).wrapping_add(Offset(1)), Offset(0));
        assert_eq!(Offset(Offset::MAX - 2).wrapping_add(Offset(4)), Offset(1));
    }

    #[test]
    fn wrapping_sub() {
        assert_eq!(Offset(444).wrapping_sub(Offset(321)), Offset(123));
        assert_eq!(Offset(0).wrapping_sub(Offset(1)), Offset(Offset::MAX));
        assert_eq!(Offset(2).wrapping_sub(Offset(4)), Offset(Offset::MAX - 1));
    }

    #[test]
    fn from_u32() {
        assert_eq!(123321u32.try_into(), Ok(Offset(123321u32)));
        assert_eq!(Offset::try_from(u32::MAX), Err(get_try_from_int_error()));
        assert_eq!(Offset::try_from(Offset::MAX), Ok(Offset(Offset::MAX)));
        assert_eq!(
            Offset::try_from(Offset::MAX + 1),
            Err(get_try_from_int_error())
        );
    }
}
