#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INS {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `EV0IS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EV0ISR {
    #[doc = "CCU8x.INyA"]
    VALUE1,
    #[doc = "CCU8x.INyB"]
    VALUE2,
    #[doc = "CCU8x.INyC"]
    VALUE3,
    #[doc = "CCU8x.INyD"]
    VALUE4,
    #[doc = "CCU8x.INyE"]
    VALUE5,
    #[doc = "CCU8x.INyF"]
    VALUE6,
    #[doc = "CCU8x.INyG"]
    VALUE7,
    #[doc = "CCU8x.INyH"]
    VALUE8,
    #[doc = "CCU8x.INyI"]
    VALUE9,
    #[doc = "CCU8x.INyJ"]
    VALUE10,
    #[doc = "CCU8x.INyK"]
    VALUE11,
    #[doc = "CCU8x.INyL"]
    VALUE12,
    #[doc = "CCU8x.INyM"]
    VALUE13,
    #[doc = "CCU8x.INyN"]
    VALUE14,
    #[doc = "CCU8x.INyO"]
    VALUE15,
    #[doc = "CCU8x.INyP"]
    VALUE16,
}
impl EV0ISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EV0ISR::VALUE1 => 0,
            EV0ISR::VALUE2 => 1,
            EV0ISR::VALUE3 => 2,
            EV0ISR::VALUE4 => 3,
            EV0ISR::VALUE5 => 4,
            EV0ISR::VALUE6 => 5,
            EV0ISR::VALUE7 => 6,
            EV0ISR::VALUE8 => 7,
            EV0ISR::VALUE9 => 8,
            EV0ISR::VALUE10 => 9,
            EV0ISR::VALUE11 => 10,
            EV0ISR::VALUE12 => 11,
            EV0ISR::VALUE13 => 12,
            EV0ISR::VALUE14 => 13,
            EV0ISR::VALUE15 => 14,
            EV0ISR::VALUE16 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EV0ISR {
        match value {
            0 => EV0ISR::VALUE1,
            1 => EV0ISR::VALUE2,
            2 => EV0ISR::VALUE3,
            3 => EV0ISR::VALUE4,
            4 => EV0ISR::VALUE5,
            5 => EV0ISR::VALUE6,
            6 => EV0ISR::VALUE7,
            7 => EV0ISR::VALUE8,
            8 => EV0ISR::VALUE9,
            9 => EV0ISR::VALUE10,
            10 => EV0ISR::VALUE11,
            11 => EV0ISR::VALUE12,
            12 => EV0ISR::VALUE13,
            13 => EV0ISR::VALUE14,
            14 => EV0ISR::VALUE15,
            15 => EV0ISR::VALUE16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EV0ISR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EV0ISR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == EV0ISR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == EV0ISR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == EV0ISR::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == EV0ISR::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == EV0ISR::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == EV0ISR::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline]
    pub fn is_value9(&self) -> bool {
        *self == EV0ISR::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline]
    pub fn is_value10(&self) -> bool {
        *self == EV0ISR::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline]
    pub fn is_value11(&self) -> bool {
        *self == EV0ISR::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline]
    pub fn is_value12(&self) -> bool {
        *self == EV0ISR::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline]
    pub fn is_value13(&self) -> bool {
        *self == EV0ISR::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline]
    pub fn is_value14(&self) -> bool {
        *self == EV0ISR::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline]
    pub fn is_value15(&self) -> bool {
        *self == EV0ISR::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline]
    pub fn is_value16(&self) -> bool {
        *self == EV0ISR::VALUE16
    }
}
#[doc = "Possible values of the field `EV1IS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EV1ISR {
    #[doc = "CCU8x.INyA"]
    VALUE1,
    #[doc = "CCU8x.INyB"]
    VALUE2,
    #[doc = "CCU8x.INyC"]
    VALUE3,
    #[doc = "CCU8x.INyD"]
    VALUE4,
    #[doc = "CCU8x.INyE"]
    VALUE5,
    #[doc = "CCU8x.INyF"]
    VALUE6,
    #[doc = "CCU8x.INyG"]
    VALUE7,
    #[doc = "CCU8x.INyH"]
    VALUE8,
    #[doc = "CCU8x.INyI"]
    VALUE9,
    #[doc = "CCU8x.INyJ"]
    VALUE10,
    #[doc = "CCU8x.INyK"]
    VALUE11,
    #[doc = "CCU8x.INyL"]
    VALUE12,
    #[doc = "CCU8x.INyM"]
    VALUE13,
    #[doc = "CCU8x.INyN"]
    VALUE14,
    #[doc = "CCU8x.INyO"]
    VALUE15,
    #[doc = "CCU8x.INyP"]
    VALUE16,
}
impl EV1ISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EV1ISR::VALUE1 => 0,
            EV1ISR::VALUE2 => 1,
            EV1ISR::VALUE3 => 2,
            EV1ISR::VALUE4 => 3,
            EV1ISR::VALUE5 => 4,
            EV1ISR::VALUE6 => 5,
            EV1ISR::VALUE7 => 6,
            EV1ISR::VALUE8 => 7,
            EV1ISR::VALUE9 => 8,
            EV1ISR::VALUE10 => 9,
            EV1ISR::VALUE11 => 10,
            EV1ISR::VALUE12 => 11,
            EV1ISR::VALUE13 => 12,
            EV1ISR::VALUE14 => 13,
            EV1ISR::VALUE15 => 14,
            EV1ISR::VALUE16 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EV1ISR {
        match value {
            0 => EV1ISR::VALUE1,
            1 => EV1ISR::VALUE2,
            2 => EV1ISR::VALUE3,
            3 => EV1ISR::VALUE4,
            4 => EV1ISR::VALUE5,
            5 => EV1ISR::VALUE6,
            6 => EV1ISR::VALUE7,
            7 => EV1ISR::VALUE8,
            8 => EV1ISR::VALUE9,
            9 => EV1ISR::VALUE10,
            10 => EV1ISR::VALUE11,
            11 => EV1ISR::VALUE12,
            12 => EV1ISR::VALUE13,
            13 => EV1ISR::VALUE14,
            14 => EV1ISR::VALUE15,
            15 => EV1ISR::VALUE16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EV1ISR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EV1ISR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == EV1ISR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == EV1ISR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == EV1ISR::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == EV1ISR::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == EV1ISR::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == EV1ISR::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline]
    pub fn is_value9(&self) -> bool {
        *self == EV1ISR::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline]
    pub fn is_value10(&self) -> bool {
        *self == EV1ISR::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline]
    pub fn is_value11(&self) -> bool {
        *self == EV1ISR::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline]
    pub fn is_value12(&self) -> bool {
        *self == EV1ISR::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline]
    pub fn is_value13(&self) -> bool {
        *self == EV1ISR::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline]
    pub fn is_value14(&self) -> bool {
        *self == EV1ISR::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline]
    pub fn is_value15(&self) -> bool {
        *self == EV1ISR::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline]
    pub fn is_value16(&self) -> bool {
        *self == EV1ISR::VALUE16
    }
}
#[doc = "Possible values of the field `EV2IS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EV2ISR {
    #[doc = "CCU8x.INyA"]
    VALUE1,
    #[doc = "CCU8x.INyB"]
    VALUE2,
    #[doc = "CCU8x.INyC"]
    VALUE3,
    #[doc = "CCU8x.INyD"]
    VALUE4,
    #[doc = "CCU8x.INyE"]
    VALUE5,
    #[doc = "CCU8x.INyF"]
    VALUE6,
    #[doc = "CCU8x.INyG"]
    VALUE7,
    #[doc = "CCU8x.INyH"]
    VALUE8,
    #[doc = "CCU8x.INyI"]
    VALUE9,
    #[doc = "CCU8x.INyJ"]
    VALUE10,
    #[doc = "CCU8x.INyK"]
    VALUE11,
    #[doc = "CCU8x.INyL"]
    VALUE12,
    #[doc = "CCU8x.INyM"]
    VALUE13,
    #[doc = "CCU8x.INyN"]
    VALUE14,
    #[doc = "CCU8x.INyO"]
    VALUE15,
    #[doc = "CCU8x.INyP"]
    VALUE16,
}
impl EV2ISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EV2ISR::VALUE1 => 0,
            EV2ISR::VALUE2 => 1,
            EV2ISR::VALUE3 => 2,
            EV2ISR::VALUE4 => 3,
            EV2ISR::VALUE5 => 4,
            EV2ISR::VALUE6 => 5,
            EV2ISR::VALUE7 => 6,
            EV2ISR::VALUE8 => 7,
            EV2ISR::VALUE9 => 8,
            EV2ISR::VALUE10 => 9,
            EV2ISR::VALUE11 => 10,
            EV2ISR::VALUE12 => 11,
            EV2ISR::VALUE13 => 12,
            EV2ISR::VALUE14 => 13,
            EV2ISR::VALUE15 => 14,
            EV2ISR::VALUE16 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EV2ISR {
        match value {
            0 => EV2ISR::VALUE1,
            1 => EV2ISR::VALUE2,
            2 => EV2ISR::VALUE3,
            3 => EV2ISR::VALUE4,
            4 => EV2ISR::VALUE5,
            5 => EV2ISR::VALUE6,
            6 => EV2ISR::VALUE7,
            7 => EV2ISR::VALUE8,
            8 => EV2ISR::VALUE9,
            9 => EV2ISR::VALUE10,
            10 => EV2ISR::VALUE11,
            11 => EV2ISR::VALUE12,
            12 => EV2ISR::VALUE13,
            13 => EV2ISR::VALUE14,
            14 => EV2ISR::VALUE15,
            15 => EV2ISR::VALUE16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EV2ISR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EV2ISR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == EV2ISR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == EV2ISR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == EV2ISR::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == EV2ISR::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == EV2ISR::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == EV2ISR::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline]
    pub fn is_value9(&self) -> bool {
        *self == EV2ISR::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline]
    pub fn is_value10(&self) -> bool {
        *self == EV2ISR::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline]
    pub fn is_value11(&self) -> bool {
        *self == EV2ISR::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline]
    pub fn is_value12(&self) -> bool {
        *self == EV2ISR::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline]
    pub fn is_value13(&self) -> bool {
        *self == EV2ISR::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline]
    pub fn is_value14(&self) -> bool {
        *self == EV2ISR::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline]
    pub fn is_value15(&self) -> bool {
        *self == EV2ISR::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline]
    pub fn is_value16(&self) -> bool {
        *self == EV2ISR::VALUE16
    }
}
#[doc = "Possible values of the field `EV0EM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EV0EMR {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Signal active on rising edge"]
    VALUE2,
    #[doc = "Signal active on falling edge"]
    VALUE3,
    #[doc = "Signal active on both edges"]
    VALUE4,
}
impl EV0EMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EV0EMR::VALUE1 => 0,
            EV0EMR::VALUE2 => 1,
            EV0EMR::VALUE3 => 2,
            EV0EMR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EV0EMR {
        match value {
            0 => EV0EMR::VALUE1,
            1 => EV0EMR::VALUE2,
            2 => EV0EMR::VALUE3,
            3 => EV0EMR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EV0EMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EV0EMR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == EV0EMR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == EV0EMR::VALUE4
    }
}
#[doc = "Possible values of the field `EV1EM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EV1EMR {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Signal active on rising edge"]
    VALUE2,
    #[doc = "Signal active on falling edge"]
    VALUE3,
    #[doc = "Signal active on both edges"]
    VALUE4,
}
impl EV1EMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EV1EMR::VALUE1 => 0,
            EV1EMR::VALUE2 => 1,
            EV1EMR::VALUE3 => 2,
            EV1EMR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EV1EMR {
        match value {
            0 => EV1EMR::VALUE1,
            1 => EV1EMR::VALUE2,
            2 => EV1EMR::VALUE3,
            3 => EV1EMR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EV1EMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EV1EMR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == EV1EMR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == EV1EMR::VALUE4
    }
}
#[doc = "Possible values of the field `EV2EM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EV2EMR {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Signal active on rising edge"]
    VALUE2,
    #[doc = "Signal active on falling edge"]
    VALUE3,
    #[doc = "Signal active on both edges"]
    VALUE4,
}
impl EV2EMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EV2EMR::VALUE1 => 0,
            EV2EMR::VALUE2 => 1,
            EV2EMR::VALUE3 => 2,
            EV2EMR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EV2EMR {
        match value {
            0 => EV2EMR::VALUE1,
            1 => EV2EMR::VALUE2,
            2 => EV2EMR::VALUE3,
            3 => EV2EMR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EV2EMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EV2EMR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == EV2EMR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == EV2EMR::VALUE4
    }
}
#[doc = "Possible values of the field `EV0LM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EV0LMR {
    #[doc = "Active on HIGH level"]
    VALUE1,
    #[doc = "Active on LOW level"]
    VALUE2,
}
impl EV0LMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            EV0LMR::VALUE1 => false,
            EV0LMR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EV0LMR {
        match value {
            false => EV0LMR::VALUE1,
            true => EV0LMR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EV0LMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EV0LMR::VALUE2
    }
}
#[doc = "Possible values of the field `EV1LM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EV1LMR {
    #[doc = "Active on HIGH level"]
    VALUE1,
    #[doc = "Active on LOW level"]
    VALUE2,
}
impl EV1LMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            EV1LMR::VALUE1 => false,
            EV1LMR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EV1LMR {
        match value {
            false => EV1LMR::VALUE1,
            true => EV1LMR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EV1LMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EV1LMR::VALUE2
    }
}
#[doc = "Possible values of the field `EV2LM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EV2LMR {
    #[doc = "Active on HIGH level"]
    VALUE1,
    #[doc = "Active on LOW level"]
    VALUE2,
}
impl EV2LMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            EV2LMR::VALUE1 => false,
            EV2LMR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EV2LMR {
        match value {
            false => EV2LMR::VALUE1,
            true => EV2LMR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EV2LMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EV2LMR::VALUE2
    }
}
#[doc = "Possible values of the field `LPF0M`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPF0MR {
    #[doc = "LPF is disabled"]
    VALUE1,
    #[doc = "3 clock cycles of fCCU8"]
    VALUE2,
    #[doc = "5 clock cycles of fCCU8"]
    VALUE3,
    #[doc = "7 clock cycles of fCCU8"]
    VALUE4,
}
impl LPF0MR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LPF0MR::VALUE1 => 0,
            LPF0MR::VALUE2 => 1,
            LPF0MR::VALUE3 => 2,
            LPF0MR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LPF0MR {
        match value {
            0 => LPF0MR::VALUE1,
            1 => LPF0MR::VALUE2,
            2 => LPF0MR::VALUE3,
            3 => LPF0MR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LPF0MR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LPF0MR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == LPF0MR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == LPF0MR::VALUE4
    }
}
#[doc = "Possible values of the field `LPF1M`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPF1MR {
    #[doc = "LPF is disabled"]
    VALUE1,
    #[doc = "3 clock cycles of fCCU8"]
    VALUE2,
    #[doc = "5 clock cycles of fCCU8"]
    VALUE3,
    #[doc = "7 clock cycles of fCCU8"]
    VALUE4,
}
impl LPF1MR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LPF1MR::VALUE1 => 0,
            LPF1MR::VALUE2 => 1,
            LPF1MR::VALUE3 => 2,
            LPF1MR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LPF1MR {
        match value {
            0 => LPF1MR::VALUE1,
            1 => LPF1MR::VALUE2,
            2 => LPF1MR::VALUE3,
            3 => LPF1MR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LPF1MR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LPF1MR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == LPF1MR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == LPF1MR::VALUE4
    }
}
#[doc = "Possible values of the field `LPF2M`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPF2MR {
    #[doc = "LPF is disabled"]
    VALUE1,
    #[doc = "3 clock cycles of fCCU8"]
    VALUE2,
    #[doc = "5 clock cycles of fCCU8"]
    VALUE3,
    #[doc = "7 clock cycles of fCCU8"]
    VALUE4,
}
impl LPF2MR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LPF2MR::VALUE1 => 0,
            LPF2MR::VALUE2 => 1,
            LPF2MR::VALUE3 => 2,
            LPF2MR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LPF2MR {
        match value {
            0 => LPF2MR::VALUE1,
            1 => LPF2MR::VALUE2,
            2 => LPF2MR::VALUE3,
            3 => LPF2MR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LPF2MR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LPF2MR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == LPF2MR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == LPF2MR::VALUE4
    }
}
#[doc = "Values that can be written to the field `EV0IS`"]
pub enum EV0ISW {
    #[doc = "CCU8x.INyA"]
    VALUE1,
    #[doc = "CCU8x.INyB"]
    VALUE2,
    #[doc = "CCU8x.INyC"]
    VALUE3,
    #[doc = "CCU8x.INyD"]
    VALUE4,
    #[doc = "CCU8x.INyE"]
    VALUE5,
    #[doc = "CCU8x.INyF"]
    VALUE6,
    #[doc = "CCU8x.INyG"]
    VALUE7,
    #[doc = "CCU8x.INyH"]
    VALUE8,
    #[doc = "CCU8x.INyI"]
    VALUE9,
    #[doc = "CCU8x.INyJ"]
    VALUE10,
    #[doc = "CCU8x.INyK"]
    VALUE11,
    #[doc = "CCU8x.INyL"]
    VALUE12,
    #[doc = "CCU8x.INyM"]
    VALUE13,
    #[doc = "CCU8x.INyN"]
    VALUE14,
    #[doc = "CCU8x.INyO"]
    VALUE15,
    #[doc = "CCU8x.INyP"]
    VALUE16,
}
impl EV0ISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EV0ISW::VALUE1 => 0,
            EV0ISW::VALUE2 => 1,
            EV0ISW::VALUE3 => 2,
            EV0ISW::VALUE4 => 3,
            EV0ISW::VALUE5 => 4,
            EV0ISW::VALUE6 => 5,
            EV0ISW::VALUE7 => 6,
            EV0ISW::VALUE8 => 7,
            EV0ISW::VALUE9 => 8,
            EV0ISW::VALUE10 => 9,
            EV0ISW::VALUE11 => 10,
            EV0ISW::VALUE12 => 11,
            EV0ISW::VALUE13 => 12,
            EV0ISW::VALUE14 => 13,
            EV0ISW::VALUE15 => 14,
            EV0ISW::VALUE16 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EV0ISW<'a> {
    w: &'a mut W,
}
impl<'a> _EV0ISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EV0ISW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCU8x.INyA"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EV0ISW::VALUE1)
    }
    #[doc = "CCU8x.INyB"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EV0ISW::VALUE2)
    }
    #[doc = "CCU8x.INyC"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(EV0ISW::VALUE3)
    }
    #[doc = "CCU8x.INyD"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(EV0ISW::VALUE4)
    }
    #[doc = "CCU8x.INyE"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(EV0ISW::VALUE5)
    }
    #[doc = "CCU8x.INyF"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(EV0ISW::VALUE6)
    }
    #[doc = "CCU8x.INyG"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(EV0ISW::VALUE7)
    }
    #[doc = "CCU8x.INyH"]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(EV0ISW::VALUE8)
    }
    #[doc = "CCU8x.INyI"]
    #[inline]
    pub fn value9(self) -> &'a mut W {
        self.variant(EV0ISW::VALUE9)
    }
    #[doc = "CCU8x.INyJ"]
    #[inline]
    pub fn value10(self) -> &'a mut W {
        self.variant(EV0ISW::VALUE10)
    }
    #[doc = "CCU8x.INyK"]
    #[inline]
    pub fn value11(self) -> &'a mut W {
        self.variant(EV0ISW::VALUE11)
    }
    #[doc = "CCU8x.INyL"]
    #[inline]
    pub fn value12(self) -> &'a mut W {
        self.variant(EV0ISW::VALUE12)
    }
    #[doc = "CCU8x.INyM"]
    #[inline]
    pub fn value13(self) -> &'a mut W {
        self.variant(EV0ISW::VALUE13)
    }
    #[doc = "CCU8x.INyN"]
    #[inline]
    pub fn value14(self) -> &'a mut W {
        self.variant(EV0ISW::VALUE14)
    }
    #[doc = "CCU8x.INyO"]
    #[inline]
    pub fn value15(self) -> &'a mut W {
        self.variant(EV0ISW::VALUE15)
    }
    #[doc = "CCU8x.INyP"]
    #[inline]
    pub fn value16(self) -> &'a mut W {
        self.variant(EV0ISW::VALUE16)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EV1IS`"]
pub enum EV1ISW {
    #[doc = "CCU8x.INyA"]
    VALUE1,
    #[doc = "CCU8x.INyB"]
    VALUE2,
    #[doc = "CCU8x.INyC"]
    VALUE3,
    #[doc = "CCU8x.INyD"]
    VALUE4,
    #[doc = "CCU8x.INyE"]
    VALUE5,
    #[doc = "CCU8x.INyF"]
    VALUE6,
    #[doc = "CCU8x.INyG"]
    VALUE7,
    #[doc = "CCU8x.INyH"]
    VALUE8,
    #[doc = "CCU8x.INyI"]
    VALUE9,
    #[doc = "CCU8x.INyJ"]
    VALUE10,
    #[doc = "CCU8x.INyK"]
    VALUE11,
    #[doc = "CCU8x.INyL"]
    VALUE12,
    #[doc = "CCU8x.INyM"]
    VALUE13,
    #[doc = "CCU8x.INyN"]
    VALUE14,
    #[doc = "CCU8x.INyO"]
    VALUE15,
    #[doc = "CCU8x.INyP"]
    VALUE16,
}
impl EV1ISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EV1ISW::VALUE1 => 0,
            EV1ISW::VALUE2 => 1,
            EV1ISW::VALUE3 => 2,
            EV1ISW::VALUE4 => 3,
            EV1ISW::VALUE5 => 4,
            EV1ISW::VALUE6 => 5,
            EV1ISW::VALUE7 => 6,
            EV1ISW::VALUE8 => 7,
            EV1ISW::VALUE9 => 8,
            EV1ISW::VALUE10 => 9,
            EV1ISW::VALUE11 => 10,
            EV1ISW::VALUE12 => 11,
            EV1ISW::VALUE13 => 12,
            EV1ISW::VALUE14 => 13,
            EV1ISW::VALUE15 => 14,
            EV1ISW::VALUE16 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EV1ISW<'a> {
    w: &'a mut W,
}
impl<'a> _EV1ISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EV1ISW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCU8x.INyA"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EV1ISW::VALUE1)
    }
    #[doc = "CCU8x.INyB"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EV1ISW::VALUE2)
    }
    #[doc = "CCU8x.INyC"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(EV1ISW::VALUE3)
    }
    #[doc = "CCU8x.INyD"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(EV1ISW::VALUE4)
    }
    #[doc = "CCU8x.INyE"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(EV1ISW::VALUE5)
    }
    #[doc = "CCU8x.INyF"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(EV1ISW::VALUE6)
    }
    #[doc = "CCU8x.INyG"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(EV1ISW::VALUE7)
    }
    #[doc = "CCU8x.INyH"]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(EV1ISW::VALUE8)
    }
    #[doc = "CCU8x.INyI"]
    #[inline]
    pub fn value9(self) -> &'a mut W {
        self.variant(EV1ISW::VALUE9)
    }
    #[doc = "CCU8x.INyJ"]
    #[inline]
    pub fn value10(self) -> &'a mut W {
        self.variant(EV1ISW::VALUE10)
    }
    #[doc = "CCU8x.INyK"]
    #[inline]
    pub fn value11(self) -> &'a mut W {
        self.variant(EV1ISW::VALUE11)
    }
    #[doc = "CCU8x.INyL"]
    #[inline]
    pub fn value12(self) -> &'a mut W {
        self.variant(EV1ISW::VALUE12)
    }
    #[doc = "CCU8x.INyM"]
    #[inline]
    pub fn value13(self) -> &'a mut W {
        self.variant(EV1ISW::VALUE13)
    }
    #[doc = "CCU8x.INyN"]
    #[inline]
    pub fn value14(self) -> &'a mut W {
        self.variant(EV1ISW::VALUE14)
    }
    #[doc = "CCU8x.INyO"]
    #[inline]
    pub fn value15(self) -> &'a mut W {
        self.variant(EV1ISW::VALUE15)
    }
    #[doc = "CCU8x.INyP"]
    #[inline]
    pub fn value16(self) -> &'a mut W {
        self.variant(EV1ISW::VALUE16)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EV2IS`"]
pub enum EV2ISW {
    #[doc = "CCU8x.INyA"]
    VALUE1,
    #[doc = "CCU8x.INyB"]
    VALUE2,
    #[doc = "CCU8x.INyC"]
    VALUE3,
    #[doc = "CCU8x.INyD"]
    VALUE4,
    #[doc = "CCU8x.INyE"]
    VALUE5,
    #[doc = "CCU8x.INyF"]
    VALUE6,
    #[doc = "CCU8x.INyG"]
    VALUE7,
    #[doc = "CCU8x.INyH"]
    VALUE8,
    #[doc = "CCU8x.INyI"]
    VALUE9,
    #[doc = "CCU8x.INyJ"]
    VALUE10,
    #[doc = "CCU8x.INyK"]
    VALUE11,
    #[doc = "CCU8x.INyL"]
    VALUE12,
    #[doc = "CCU8x.INyM"]
    VALUE13,
    #[doc = "CCU8x.INyN"]
    VALUE14,
    #[doc = "CCU8x.INyO"]
    VALUE15,
    #[doc = "CCU8x.INyP"]
    VALUE16,
}
impl EV2ISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EV2ISW::VALUE1 => 0,
            EV2ISW::VALUE2 => 1,
            EV2ISW::VALUE3 => 2,
            EV2ISW::VALUE4 => 3,
            EV2ISW::VALUE5 => 4,
            EV2ISW::VALUE6 => 5,
            EV2ISW::VALUE7 => 6,
            EV2ISW::VALUE8 => 7,
            EV2ISW::VALUE9 => 8,
            EV2ISW::VALUE10 => 9,
            EV2ISW::VALUE11 => 10,
            EV2ISW::VALUE12 => 11,
            EV2ISW::VALUE13 => 12,
            EV2ISW::VALUE14 => 13,
            EV2ISW::VALUE15 => 14,
            EV2ISW::VALUE16 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EV2ISW<'a> {
    w: &'a mut W,
}
impl<'a> _EV2ISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EV2ISW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCU8x.INyA"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EV2ISW::VALUE1)
    }
    #[doc = "CCU8x.INyB"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EV2ISW::VALUE2)
    }
    #[doc = "CCU8x.INyC"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(EV2ISW::VALUE3)
    }
    #[doc = "CCU8x.INyD"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(EV2ISW::VALUE4)
    }
    #[doc = "CCU8x.INyE"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(EV2ISW::VALUE5)
    }
    #[doc = "CCU8x.INyF"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(EV2ISW::VALUE6)
    }
    #[doc = "CCU8x.INyG"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(EV2ISW::VALUE7)
    }
    #[doc = "CCU8x.INyH"]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(EV2ISW::VALUE8)
    }
    #[doc = "CCU8x.INyI"]
    #[inline]
    pub fn value9(self) -> &'a mut W {
        self.variant(EV2ISW::VALUE9)
    }
    #[doc = "CCU8x.INyJ"]
    #[inline]
    pub fn value10(self) -> &'a mut W {
        self.variant(EV2ISW::VALUE10)
    }
    #[doc = "CCU8x.INyK"]
    #[inline]
    pub fn value11(self) -> &'a mut W {
        self.variant(EV2ISW::VALUE11)
    }
    #[doc = "CCU8x.INyL"]
    #[inline]
    pub fn value12(self) -> &'a mut W {
        self.variant(EV2ISW::VALUE12)
    }
    #[doc = "CCU8x.INyM"]
    #[inline]
    pub fn value13(self) -> &'a mut W {
        self.variant(EV2ISW::VALUE13)
    }
    #[doc = "CCU8x.INyN"]
    #[inline]
    pub fn value14(self) -> &'a mut W {
        self.variant(EV2ISW::VALUE14)
    }
    #[doc = "CCU8x.INyO"]
    #[inline]
    pub fn value15(self) -> &'a mut W {
        self.variant(EV2ISW::VALUE15)
    }
    #[doc = "CCU8x.INyP"]
    #[inline]
    pub fn value16(self) -> &'a mut W {
        self.variant(EV2ISW::VALUE16)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EV0EM`"]
pub enum EV0EMW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Signal active on rising edge"]
    VALUE2,
    #[doc = "Signal active on falling edge"]
    VALUE3,
    #[doc = "Signal active on both edges"]
    VALUE4,
}
impl EV0EMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EV0EMW::VALUE1 => 0,
            EV0EMW::VALUE2 => 1,
            EV0EMW::VALUE3 => 2,
            EV0EMW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EV0EMW<'a> {
    w: &'a mut W,
}
impl<'a> _EV0EMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EV0EMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EV0EMW::VALUE1)
    }
    #[doc = "Signal active on rising edge"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EV0EMW::VALUE2)
    }
    #[doc = "Signal active on falling edge"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(EV0EMW::VALUE3)
    }
    #[doc = "Signal active on both edges"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(EV0EMW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EV1EM`"]
pub enum EV1EMW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Signal active on rising edge"]
    VALUE2,
    #[doc = "Signal active on falling edge"]
    VALUE3,
    #[doc = "Signal active on both edges"]
    VALUE4,
}
impl EV1EMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EV1EMW::VALUE1 => 0,
            EV1EMW::VALUE2 => 1,
            EV1EMW::VALUE3 => 2,
            EV1EMW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EV1EMW<'a> {
    w: &'a mut W,
}
impl<'a> _EV1EMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EV1EMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EV1EMW::VALUE1)
    }
    #[doc = "Signal active on rising edge"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EV1EMW::VALUE2)
    }
    #[doc = "Signal active on falling edge"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(EV1EMW::VALUE3)
    }
    #[doc = "Signal active on both edges"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(EV1EMW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EV2EM`"]
pub enum EV2EMW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Signal active on rising edge"]
    VALUE2,
    #[doc = "Signal active on falling edge"]
    VALUE3,
    #[doc = "Signal active on both edges"]
    VALUE4,
}
impl EV2EMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EV2EMW::VALUE1 => 0,
            EV2EMW::VALUE2 => 1,
            EV2EMW::VALUE3 => 2,
            EV2EMW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EV2EMW<'a> {
    w: &'a mut W,
}
impl<'a> _EV2EMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EV2EMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EV2EMW::VALUE1)
    }
    #[doc = "Signal active on rising edge"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EV2EMW::VALUE2)
    }
    #[doc = "Signal active on falling edge"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(EV2EMW::VALUE3)
    }
    #[doc = "Signal active on both edges"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(EV2EMW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EV0LM`"]
pub enum EV0LMW {
    #[doc = "Active on HIGH level"]
    VALUE1,
    #[doc = "Active on LOW level"]
    VALUE2,
}
impl EV0LMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EV0LMW::VALUE1 => false,
            EV0LMW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EV0LMW<'a> {
    w: &'a mut W,
}
impl<'a> _EV0LMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EV0LMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Active on HIGH level"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EV0LMW::VALUE1)
    }
    #[doc = "Active on LOW level"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EV0LMW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EV1LM`"]
pub enum EV1LMW {
    #[doc = "Active on HIGH level"]
    VALUE1,
    #[doc = "Active on LOW level"]
    VALUE2,
}
impl EV1LMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EV1LMW::VALUE1 => false,
            EV1LMW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EV1LMW<'a> {
    w: &'a mut W,
}
impl<'a> _EV1LMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EV1LMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Active on HIGH level"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EV1LMW::VALUE1)
    }
    #[doc = "Active on LOW level"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EV1LMW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EV2LM`"]
pub enum EV2LMW {
    #[doc = "Active on HIGH level"]
    VALUE1,
    #[doc = "Active on LOW level"]
    VALUE2,
}
impl EV2LMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EV2LMW::VALUE1 => false,
            EV2LMW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EV2LMW<'a> {
    w: &'a mut W,
}
impl<'a> _EV2LMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EV2LMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Active on HIGH level"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EV2LMW::VALUE1)
    }
    #[doc = "Active on LOW level"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EV2LMW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPF0M`"]
pub enum LPF0MW {
    #[doc = "LPF is disabled"]
    VALUE1,
    #[doc = "3 clock cycles of fCCU8"]
    VALUE2,
    #[doc = "5 clock cycles of fCCU8"]
    VALUE3,
    #[doc = "7 clock cycles of fCCU8"]
    VALUE4,
}
impl LPF0MW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LPF0MW::VALUE1 => 0,
            LPF0MW::VALUE2 => 1,
            LPF0MW::VALUE3 => 2,
            LPF0MW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPF0MW<'a> {
    w: &'a mut W,
}
impl<'a> _LPF0MW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPF0MW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "LPF is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LPF0MW::VALUE1)
    }
    #[doc = "3 clock cycles of fCCU8"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LPF0MW::VALUE2)
    }
    #[doc = "5 clock cycles of fCCU8"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(LPF0MW::VALUE3)
    }
    #[doc = "7 clock cycles of fCCU8"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(LPF0MW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPF1M`"]
pub enum LPF1MW {
    #[doc = "LPF is disabled"]
    VALUE1,
    #[doc = "3 clock cycles of fCCU8"]
    VALUE2,
    #[doc = "5 clock cycles of fCCU8"]
    VALUE3,
    #[doc = "7 clock cycles of fCCU8"]
    VALUE4,
}
impl LPF1MW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LPF1MW::VALUE1 => 0,
            LPF1MW::VALUE2 => 1,
            LPF1MW::VALUE3 => 2,
            LPF1MW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPF1MW<'a> {
    w: &'a mut W,
}
impl<'a> _LPF1MW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPF1MW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "LPF is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LPF1MW::VALUE1)
    }
    #[doc = "3 clock cycles of fCCU8"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LPF1MW::VALUE2)
    }
    #[doc = "5 clock cycles of fCCU8"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(LPF1MW::VALUE3)
    }
    #[doc = "7 clock cycles of fCCU8"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(LPF1MW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPF2M`"]
pub enum LPF2MW {
    #[doc = "LPF is disabled"]
    VALUE1,
    #[doc = "3 clock cycles of fCCU8"]
    VALUE2,
    #[doc = "5 clock cycles of fCCU8"]
    VALUE3,
    #[doc = "7 clock cycles of fCCU8"]
    VALUE4,
}
impl LPF2MW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LPF2MW::VALUE1 => 0,
            LPF2MW::VALUE2 => 1,
            LPF2MW::VALUE3 => 2,
            LPF2MW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPF2MW<'a> {
    w: &'a mut W,
}
impl<'a> _LPF2MW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPF2MW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "LPF is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LPF2MW::VALUE1)
    }
    #[doc = "3 clock cycles of fCCU8"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LPF2MW::VALUE2)
    }
    #[doc = "5 clock cycles of fCCU8"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(LPF2MW::VALUE3)
    }
    #[doc = "7 clock cycles of fCCU8"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(LPF2MW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Event 0 signal selection"]
    #[inline]
    pub fn ev0is(&self) -> EV0ISR {
        EV0ISR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Event 1 signal selection"]
    #[inline]
    pub fn ev1is(&self) -> EV1ISR {
        EV1ISR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Event 2 signal selection"]
    #[inline]
    pub fn ev2is(&self) -> EV2ISR {
        EV2ISR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Event 0 Edge Selection"]
    #[inline]
    pub fn ev0em(&self) -> EV0EMR {
        EV0EMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - Event 1 Edge Selection"]
    #[inline]
    pub fn ev1em(&self) -> EV1EMR {
        EV1EMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Event 2 Edge Selection"]
    #[inline]
    pub fn ev2em(&self) -> EV2EMR {
        EV2EMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 22 - Event 0 Level Selection"]
    #[inline]
    pub fn ev0lm(&self) -> EV0LMR {
        EV0LMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Event 1 Level Selection"]
    #[inline]
    pub fn ev1lm(&self) -> EV1LMR {
        EV1LMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Event 2 Level Selection"]
    #[inline]
    pub fn ev2lm(&self) -> EV2LMR {
        EV2LMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 25:26 - Event 0 Low Pass Filter Configuration"]
    #[inline]
    pub fn lpf0m(&self) -> LPF0MR {
        LPF0MR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 27:28 - Event 1 Low Pass Filter Configuration"]
    #[inline]
    pub fn lpf1m(&self) -> LPF1MR {
        LPF1MR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 29:30 - Event 2 Low Pass Filter Configuration"]
    #[inline]
    pub fn lpf2m(&self) -> LPF2MR {
        LPF2MR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Event 0 signal selection"]
    #[inline]
    pub fn ev0is(&mut self) -> _EV0ISW {
        _EV0ISW { w: self }
    }
    #[doc = "Bits 4:7 - Event 1 signal selection"]
    #[inline]
    pub fn ev1is(&mut self) -> _EV1ISW {
        _EV1ISW { w: self }
    }
    #[doc = "Bits 8:11 - Event 2 signal selection"]
    #[inline]
    pub fn ev2is(&mut self) -> _EV2ISW {
        _EV2ISW { w: self }
    }
    #[doc = "Bits 16:17 - Event 0 Edge Selection"]
    #[inline]
    pub fn ev0em(&mut self) -> _EV0EMW {
        _EV0EMW { w: self }
    }
    #[doc = "Bits 18:19 - Event 1 Edge Selection"]
    #[inline]
    pub fn ev1em(&mut self) -> _EV1EMW {
        _EV1EMW { w: self }
    }
    #[doc = "Bits 20:21 - Event 2 Edge Selection"]
    #[inline]
    pub fn ev2em(&mut self) -> _EV2EMW {
        _EV2EMW { w: self }
    }
    #[doc = "Bit 22 - Event 0 Level Selection"]
    #[inline]
    pub fn ev0lm(&mut self) -> _EV0LMW {
        _EV0LMW { w: self }
    }
    #[doc = "Bit 23 - Event 1 Level Selection"]
    #[inline]
    pub fn ev1lm(&mut self) -> _EV1LMW {
        _EV1LMW { w: self }
    }
    #[doc = "Bit 24 - Event 2 Level Selection"]
    #[inline]
    pub fn ev2lm(&mut self) -> _EV2LMW {
        _EV2LMW { w: self }
    }
    #[doc = "Bits 25:26 - Event 0 Low Pass Filter Configuration"]
    #[inline]
    pub fn lpf0m(&mut self) -> _LPF0MW {
        _LPF0MW { w: self }
    }
    #[doc = "Bits 27:28 - Event 1 Low Pass Filter Configuration"]
    #[inline]
    pub fn lpf1m(&mut self) -> _LPF1MW {
        _LPF1MW { w: self }
    }
    #[doc = "Bits 29:30 - Event 2 Low Pass Filter Configuration"]
    #[inline]
    pub fn lpf2m(&mut self) -> _LPF2MW {
        _LPF2MW { w: self }
    }
}
