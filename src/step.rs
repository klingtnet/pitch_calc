use std::cmp::Ordering;
use std::ops::{Add, Sub, Mul, Div, Rem, Neg};
use super::{
    calc,
    DEFAULT_SCALE_WEIGHT,
    Hz,
    LetterOctave,
    Letter,
    Mel,
    Octave,
    Perc,
    ScaledPerc,
    ScaleWeight,
    hz_from_step,
    letter_octave_from_step,
    mel_from_step,
    perc_from_step,
    scaled_perc_from_step,
};

/// Pitch representation in the form of a MIDI-esque Step.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde_serialization", derive(Serialize, Deserialize))]
pub struct Step(pub calc::Step);

impl Step {
    /// Return the value in steps.
    #[inline]
    pub fn step(self) -> calc::Step {
        let Step(step) = self;
        step
    }

    /// Return the unit value of the equivalent frequency Hz.
    #[inline]
    pub fn hz(self) -> calc::Hz {
        let Step(step) = self;
        hz_from_step(step)
    }

    /// Convert to the equivalent frequency in Hz.
    #[inline]
    pub fn to_hz(self) -> Hz {
        Hz(self.hz())
    }

    /// Convert to the closest equivalent (Letter, Octave).
    #[inline]
    pub fn letter_octave(self) -> (Letter, Octave) {
        letter_octave_from_step(self.step())
    }

    /// Convert to the closest equivalent Letter.
    #[inline]
    pub fn letter(self) -> Letter {
        let (letter, _) = self.letter_octave();
        letter
    }

    /// Convert to the closest equivalent Octave.
    #[inline]
    pub fn octave(self) -> Octave {
        let (_, octave) = self.letter_octave();
        octave
    }

    /// Convert to the closest equivalent LetterOctave.
    #[inline]
    pub fn to_letter_octave(self) -> LetterOctave {
        let (letter, octave) = self.letter_octave();
        LetterOctave(letter, octave)
    }

    /// Convert to a Mel unit value.
    #[inline]
    pub fn mel(self) -> calc::Mel {
        mel_from_step(self.step())
    }

    /// Convert to a Mel struct.
    #[inline]
    pub fn to_mel(self) -> Mel {
        Mel(self.mel())
    }

    /// Convert to the unit value of the equivalent Perc.
    #[inline]
    pub fn perc(self) -> calc::Perc {
        perc_from_step(self.step())
    }

    /// Convert to a percentage of the human hearing range.
    #[inline]
    pub fn to_perc(self) -> Perc {
        Perc(self.perc())
    }

    /// Convert to a scaled percentage of the human hearing range with a given weight.
    #[inline]
    pub fn scaled_perc_with_weight(self, weight: ScaleWeight) -> calc::Perc {
        scaled_perc_from_step(self.step(), weight)
    }

    /// Convert to a scaled percentage of the human hearing range.
    #[inline]
    pub fn scaled_perc(self) -> calc::Perc {
        self.scaled_perc_with_weight(DEFAULT_SCALE_WEIGHT)
    }

    /// Convert to a scaled percentage of the human hearing range with a given weight.
    #[inline]
    pub fn to_scaled_perc_with_weight(self, weight: ScaleWeight) -> ScaledPerc {
        ScaledPerc(self.scaled_perc_with_weight(weight), weight)
    }

    /// Convert to a scaled percentage of the human hearing range.
    #[inline]
    pub fn to_scaled_perc(self) -> ScaledPerc {
        self.to_scaled_perc_with_weight(DEFAULT_SCALE_WEIGHT)
    }
}

impl Add for Step {
    type Output = Step;
    #[inline]
    fn add(self, rhs: Step) -> Step {
        Step(self.step() + rhs.step())
    }
}

impl Sub for Step {
    type Output = Step;
    #[inline]
    fn sub(self, rhs: Step) -> Step {
        Step(self.step() - rhs.step())
    }
}

impl Mul for Step {
    type Output = Step;
    #[inline]
    fn mul(self, rhs: Step) -> Step {
        Step(self.step() * rhs.step())
    }
}

impl Div for Step {
    type Output = Step;
    #[inline]
    fn div(self, rhs: Step) -> Step {
        Step(self.step() / rhs.step())
    }
}

impl Rem for Step {
    type Output = Step;
    #[inline]
    fn rem(self, rhs: Step) -> Step {
        Step(self.step() % rhs.step())
    }
}

impl Neg for Step {
    type Output = Step;
    #[inline]
    fn neg(self) -> Step {
        Step(-self.step())
    }
}

impl PartialEq for Step {
    #[inline]
    fn eq(&self, other: &Step) -> bool {
        self.step() == other.step()
    }
}

impl Eq for Step {}

impl PartialOrd for Step {
    #[inline]
    fn partial_cmp(&self, other: &Step) -> Option<Ordering> {
        self.step().partial_cmp(&other.step())
    }
}

impl Ord for Step {
    #[inline]
    fn cmp(&self, other: &Step) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

macro_rules! impl_from {
    ($TargetType:ty) => {
        impl From<$TargetType> for Step {
            fn from(val: $TargetType) -> Self {
                Step(val as f32)
            }
        }

        impl Into<$TargetType> for Step {
            fn into(self) -> $TargetType {
                self.0 as $TargetType
            }
        }
    };
}

macro_rules! impl_all_from {
    (
        $($x:ty),+
    ) => {
        $(
            impl_from!($x);
        )*
    };
}

impl_all_from!(u8, u16, u32, u64, i8, i16, i32, i64);

#[cfg(test)]
mod tests {
    use super::Step;

    #[test]
    fn test_from_integer() {
        assert_eq!(Step::from(60u8), Step(60.0));
        assert_eq!(Step::from(60u16), Step(60.0));
        assert_eq!(Step::from(60u32), Step(60.0));
        assert_eq!(Step::from(60u64), Step(60.0));
        assert_eq!(Step::from(60i8), Step(60.0));
        assert_eq!(Step::from(60i16), Step(60.0));
        assert_eq!(Step::from(60i32), Step(60.0));
        assert_eq!(Step::from(60i64), Step(60.0));
    }

    #[test]
    fn test_into_integer() {
        assert_eq!(60u8, Step(60.0).into());
        assert_eq!(60u16, Step(60.0).into());
        assert_eq!(60u32, Step(60.0).into());
        assert_eq!(60u64, Step(60.0).into());
        assert_eq!(60i8, Step(60.0).into());
        assert_eq!(60i16, Step(60.0).into());
        assert_eq!(60i32, Step(60.0).into());
        assert_eq!(60i64, Step(60.0).into());
    }
}
