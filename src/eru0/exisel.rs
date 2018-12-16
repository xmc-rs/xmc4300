#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EXISEL {
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
#[doc = "Possible values of the field `EXS0A`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXS0AR {
    #[doc = "Input ERU_0A0 is selected"]
    VALUE1,
    #[doc = "Input ERU_0A1 is selected"]
    VALUE2,
    #[doc = "Input ERU_0A2 is selected"]
    VALUE3,
    #[doc = "Input ERU_0A3 is selected"]
    VALUE4,
}
impl EXS0AR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXS0AR::VALUE1 => 0,
            EXS0AR::VALUE2 => 1,
            EXS0AR::VALUE3 => 2,
            EXS0AR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXS0AR {
        match value {
            0 => EXS0AR::VALUE1,
            1 => EXS0AR::VALUE2,
            2 => EXS0AR::VALUE3,
            3 => EXS0AR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EXS0AR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EXS0AR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == EXS0AR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == EXS0AR::VALUE4
    }
}
#[doc = "Possible values of the field `EXS0B`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXS0BR {
    #[doc = "Input ERU_0B0 is selected"]
    VALUE1,
    #[doc = "Input ERU_0B1 is selected"]
    VALUE2,
    #[doc = "Input ERU_0B2 is selected"]
    VALUE3,
    #[doc = "Input ERU_0B3 is selected"]
    VALUE4,
}
impl EXS0BR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXS0BR::VALUE1 => 0,
            EXS0BR::VALUE2 => 1,
            EXS0BR::VALUE3 => 2,
            EXS0BR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXS0BR {
        match value {
            0 => EXS0BR::VALUE1,
            1 => EXS0BR::VALUE2,
            2 => EXS0BR::VALUE3,
            3 => EXS0BR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EXS0BR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EXS0BR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == EXS0BR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == EXS0BR::VALUE4
    }
}
#[doc = "Possible values of the field `EXS1A`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXS1AR {
    #[doc = "Input ERU_1A0 is selected"]
    VALUE1,
    #[doc = "Input ERU_1A1 is selected"]
    VALUE2,
    #[doc = "Input ERU_1A2 is selected"]
    VALUE3,
    #[doc = "Input ERU_1A3 is selected"]
    VALUE4,
}
impl EXS1AR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXS1AR::VALUE1 => 0,
            EXS1AR::VALUE2 => 1,
            EXS1AR::VALUE3 => 2,
            EXS1AR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXS1AR {
        match value {
            0 => EXS1AR::VALUE1,
            1 => EXS1AR::VALUE2,
            2 => EXS1AR::VALUE3,
            3 => EXS1AR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EXS1AR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EXS1AR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == EXS1AR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == EXS1AR::VALUE4
    }
}
#[doc = "Possible values of the field `EXS1B`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXS1BR {
    #[doc = "Input ERU_1B0 is selected"]
    VALUE1,
    #[doc = "Input ERU_1B1 is selected"]
    VALUE2,
    #[doc = "Input ERU_1B2 is selected"]
    VALUE3,
    #[doc = "Input ERU_1B3 is selected"]
    VALUE4,
}
impl EXS1BR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXS1BR::VALUE1 => 0,
            EXS1BR::VALUE2 => 1,
            EXS1BR::VALUE3 => 2,
            EXS1BR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXS1BR {
        match value {
            0 => EXS1BR::VALUE1,
            1 => EXS1BR::VALUE2,
            2 => EXS1BR::VALUE3,
            3 => EXS1BR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EXS1BR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EXS1BR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == EXS1BR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == EXS1BR::VALUE4
    }
}
#[doc = "Possible values of the field `EXS2A`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXS2AR {
    #[doc = "Input ERU_2A0 is selected"]
    VALUE1,
    #[doc = "Input ERU_2A1 is selected"]
    VALUE2,
    #[doc = "Input ERU_2A2 is selected"]
    VALUE3,
    #[doc = "Input ERU_2A3 is selected"]
    VALUE4,
}
impl EXS2AR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXS2AR::VALUE1 => 0,
            EXS2AR::VALUE2 => 1,
            EXS2AR::VALUE3 => 2,
            EXS2AR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXS2AR {
        match value {
            0 => EXS2AR::VALUE1,
            1 => EXS2AR::VALUE2,
            2 => EXS2AR::VALUE3,
            3 => EXS2AR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EXS2AR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EXS2AR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == EXS2AR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == EXS2AR::VALUE4
    }
}
#[doc = "Possible values of the field `EXS2B`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXS2BR {
    #[doc = "Input ERU_2B0 is selected"]
    VALUE1,
    #[doc = "Input ERU_2B1 is selected"]
    VALUE2,
    #[doc = "Input ERU_2B2 is selected"]
    VALUE3,
    #[doc = "Input ERU_2B3 is selected"]
    VALUE4,
}
impl EXS2BR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXS2BR::VALUE1 => 0,
            EXS2BR::VALUE2 => 1,
            EXS2BR::VALUE3 => 2,
            EXS2BR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXS2BR {
        match value {
            0 => EXS2BR::VALUE1,
            1 => EXS2BR::VALUE2,
            2 => EXS2BR::VALUE3,
            3 => EXS2BR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EXS2BR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EXS2BR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == EXS2BR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == EXS2BR::VALUE4
    }
}
#[doc = "Possible values of the field `EXS3A`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXS3AR {
    #[doc = "Input ERU_3A0 is selected"]
    VALUE1,
    #[doc = "Input ERU_3A1 is selected"]
    VALUE2,
    #[doc = "Input ERU_3A2 is selected"]
    VALUE3,
    #[doc = "Input ERU_3A3 is selected"]
    VALUE4,
}
impl EXS3AR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXS3AR::VALUE1 => 0,
            EXS3AR::VALUE2 => 1,
            EXS3AR::VALUE3 => 2,
            EXS3AR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXS3AR {
        match value {
            0 => EXS3AR::VALUE1,
            1 => EXS3AR::VALUE2,
            2 => EXS3AR::VALUE3,
            3 => EXS3AR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EXS3AR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EXS3AR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == EXS3AR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == EXS3AR::VALUE4
    }
}
#[doc = "Possible values of the field `EXS3B`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXS3BR {
    #[doc = "Input ERU_3B0 is selected"]
    VALUE1,
    #[doc = "Input ERU_3B1 is selected"]
    VALUE2,
    #[doc = "Input ERU_3B2 is selected"]
    VALUE3,
    #[doc = "Input ERU_3B3 is selected"]
    VALUE4,
}
impl EXS3BR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXS3BR::VALUE1 => 0,
            EXS3BR::VALUE2 => 1,
            EXS3BR::VALUE3 => 2,
            EXS3BR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXS3BR {
        match value {
            0 => EXS3BR::VALUE1,
            1 => EXS3BR::VALUE2,
            2 => EXS3BR::VALUE3,
            3 => EXS3BR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EXS3BR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EXS3BR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == EXS3BR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == EXS3BR::VALUE4
    }
}
#[doc = "Values that can be written to the field `EXS0A`"]
pub enum EXS0AW {
    #[doc = "Input ERU_0A0 is selected"]
    VALUE1,
    #[doc = "Input ERU_0A1 is selected"]
    VALUE2,
    #[doc = "Input ERU_0A2 is selected"]
    VALUE3,
    #[doc = "Input ERU_0A3 is selected"]
    VALUE4,
}
impl EXS0AW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXS0AW::VALUE1 => 0,
            EXS0AW::VALUE2 => 1,
            EXS0AW::VALUE3 => 2,
            EXS0AW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXS0AW<'a> {
    w: &'a mut W,
}
impl<'a> _EXS0AW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXS0AW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input ERU_0A0 is selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EXS0AW::VALUE1)
    }
    #[doc = "Input ERU_0A1 is selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EXS0AW::VALUE2)
    }
    #[doc = "Input ERU_0A2 is selected"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(EXS0AW::VALUE3)
    }
    #[doc = "Input ERU_0A3 is selected"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(EXS0AW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXS0B`"]
pub enum EXS0BW {
    #[doc = "Input ERU_0B0 is selected"]
    VALUE1,
    #[doc = "Input ERU_0B1 is selected"]
    VALUE2,
    #[doc = "Input ERU_0B2 is selected"]
    VALUE3,
    #[doc = "Input ERU_0B3 is selected"]
    VALUE4,
}
impl EXS0BW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXS0BW::VALUE1 => 0,
            EXS0BW::VALUE2 => 1,
            EXS0BW::VALUE3 => 2,
            EXS0BW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXS0BW<'a> {
    w: &'a mut W,
}
impl<'a> _EXS0BW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXS0BW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input ERU_0B0 is selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EXS0BW::VALUE1)
    }
    #[doc = "Input ERU_0B1 is selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EXS0BW::VALUE2)
    }
    #[doc = "Input ERU_0B2 is selected"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(EXS0BW::VALUE3)
    }
    #[doc = "Input ERU_0B3 is selected"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(EXS0BW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXS1A`"]
pub enum EXS1AW {
    #[doc = "Input ERU_1A0 is selected"]
    VALUE1,
    #[doc = "Input ERU_1A1 is selected"]
    VALUE2,
    #[doc = "Input ERU_1A2 is selected"]
    VALUE3,
    #[doc = "Input ERU_1A3 is selected"]
    VALUE4,
}
impl EXS1AW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXS1AW::VALUE1 => 0,
            EXS1AW::VALUE2 => 1,
            EXS1AW::VALUE3 => 2,
            EXS1AW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXS1AW<'a> {
    w: &'a mut W,
}
impl<'a> _EXS1AW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXS1AW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input ERU_1A0 is selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EXS1AW::VALUE1)
    }
    #[doc = "Input ERU_1A1 is selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EXS1AW::VALUE2)
    }
    #[doc = "Input ERU_1A2 is selected"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(EXS1AW::VALUE3)
    }
    #[doc = "Input ERU_1A3 is selected"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(EXS1AW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXS1B`"]
pub enum EXS1BW {
    #[doc = "Input ERU_1B0 is selected"]
    VALUE1,
    #[doc = "Input ERU_1B1 is selected"]
    VALUE2,
    #[doc = "Input ERU_1B2 is selected"]
    VALUE3,
    #[doc = "Input ERU_1B3 is selected"]
    VALUE4,
}
impl EXS1BW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXS1BW::VALUE1 => 0,
            EXS1BW::VALUE2 => 1,
            EXS1BW::VALUE3 => 2,
            EXS1BW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXS1BW<'a> {
    w: &'a mut W,
}
impl<'a> _EXS1BW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXS1BW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input ERU_1B0 is selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EXS1BW::VALUE1)
    }
    #[doc = "Input ERU_1B1 is selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EXS1BW::VALUE2)
    }
    #[doc = "Input ERU_1B2 is selected"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(EXS1BW::VALUE3)
    }
    #[doc = "Input ERU_1B3 is selected"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(EXS1BW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXS2A`"]
pub enum EXS2AW {
    #[doc = "Input ERU_2A0 is selected"]
    VALUE1,
    #[doc = "Input ERU_2A1 is selected"]
    VALUE2,
    #[doc = "Input ERU_2A2 is selected"]
    VALUE3,
    #[doc = "Input ERU_2A3 is selected"]
    VALUE4,
}
impl EXS2AW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXS2AW::VALUE1 => 0,
            EXS2AW::VALUE2 => 1,
            EXS2AW::VALUE3 => 2,
            EXS2AW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXS2AW<'a> {
    w: &'a mut W,
}
impl<'a> _EXS2AW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXS2AW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input ERU_2A0 is selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EXS2AW::VALUE1)
    }
    #[doc = "Input ERU_2A1 is selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EXS2AW::VALUE2)
    }
    #[doc = "Input ERU_2A2 is selected"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(EXS2AW::VALUE3)
    }
    #[doc = "Input ERU_2A3 is selected"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(EXS2AW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXS2B`"]
pub enum EXS2BW {
    #[doc = "Input ERU_2B0 is selected"]
    VALUE1,
    #[doc = "Input ERU_2B1 is selected"]
    VALUE2,
    #[doc = "Input ERU_2B2 is selected"]
    VALUE3,
    #[doc = "Input ERU_2B3 is selected"]
    VALUE4,
}
impl EXS2BW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXS2BW::VALUE1 => 0,
            EXS2BW::VALUE2 => 1,
            EXS2BW::VALUE3 => 2,
            EXS2BW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXS2BW<'a> {
    w: &'a mut W,
}
impl<'a> _EXS2BW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXS2BW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input ERU_2B0 is selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EXS2BW::VALUE1)
    }
    #[doc = "Input ERU_2B1 is selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EXS2BW::VALUE2)
    }
    #[doc = "Input ERU_2B2 is selected"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(EXS2BW::VALUE3)
    }
    #[doc = "Input ERU_2B3 is selected"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(EXS2BW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXS3A`"]
pub enum EXS3AW {
    #[doc = "Input ERU_3A0 is selected"]
    VALUE1,
    #[doc = "Input ERU_3A1 is selected"]
    VALUE2,
    #[doc = "Input ERU_3A2 is selected"]
    VALUE3,
    #[doc = "Input ERU_3A3 is selected"]
    VALUE4,
}
impl EXS3AW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXS3AW::VALUE1 => 0,
            EXS3AW::VALUE2 => 1,
            EXS3AW::VALUE3 => 2,
            EXS3AW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXS3AW<'a> {
    w: &'a mut W,
}
impl<'a> _EXS3AW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXS3AW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input ERU_3A0 is selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EXS3AW::VALUE1)
    }
    #[doc = "Input ERU_3A1 is selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EXS3AW::VALUE2)
    }
    #[doc = "Input ERU_3A2 is selected"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(EXS3AW::VALUE3)
    }
    #[doc = "Input ERU_3A3 is selected"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(EXS3AW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXS3B`"]
pub enum EXS3BW {
    #[doc = "Input ERU_3B0 is selected"]
    VALUE1,
    #[doc = "Input ERU_3B1 is selected"]
    VALUE2,
    #[doc = "Input ERU_3B2 is selected"]
    VALUE3,
    #[doc = "Input ERU_3B3 is selected"]
    VALUE4,
}
impl EXS3BW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXS3BW::VALUE1 => 0,
            EXS3BW::VALUE2 => 1,
            EXS3BW::VALUE3 => 2,
            EXS3BW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXS3BW<'a> {
    w: &'a mut W,
}
impl<'a> _EXS3BW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXS3BW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input ERU_3B0 is selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EXS3BW::VALUE1)
    }
    #[doc = "Input ERU_3B1 is selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EXS3BW::VALUE2)
    }
    #[doc = "Input ERU_3B2 is selected"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(EXS3BW::VALUE3)
    }
    #[doc = "Input ERU_3B3 is selected"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(EXS3BW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
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
    #[doc = "Bits 0:1 - Event Source Select for A0 (ERS0)"]
    #[inline]
    pub fn exs0a(&self) -> EXS0AR {
        EXS0AR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Event Source Select for B0 (ERS0)"]
    #[inline]
    pub fn exs0b(&self) -> EXS0BR {
        EXS0BR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Event Source Select for A1 (ERS1)"]
    #[inline]
    pub fn exs1a(&self) -> EXS1AR {
        EXS1AR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Event Source Select for B1 (ERS1)"]
    #[inline]
    pub fn exs1b(&self) -> EXS1BR {
        EXS1BR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Event Source Select for A2 (ERS2)"]
    #[inline]
    pub fn exs2a(&self) -> EXS2AR {
        EXS2AR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Event Source Select for B2 (ERS2)"]
    #[inline]
    pub fn exs2b(&self) -> EXS2BR {
        EXS2BR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Event Source Select for A3 (ERS3)"]
    #[inline]
    pub fn exs3a(&self) -> EXS3AR {
        EXS3AR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Event Source Select for B3 (ERS3)"]
    #[inline]
    pub fn exs3b(&self) -> EXS3BR {
        EXS3BR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
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
    #[doc = "Bits 0:1 - Event Source Select for A0 (ERS0)"]
    #[inline]
    pub fn exs0a(&mut self) -> _EXS0AW {
        _EXS0AW { w: self }
    }
    #[doc = "Bits 2:3 - Event Source Select for B0 (ERS0)"]
    #[inline]
    pub fn exs0b(&mut self) -> _EXS0BW {
        _EXS0BW { w: self }
    }
    #[doc = "Bits 4:5 - Event Source Select for A1 (ERS1)"]
    #[inline]
    pub fn exs1a(&mut self) -> _EXS1AW {
        _EXS1AW { w: self }
    }
    #[doc = "Bits 6:7 - Event Source Select for B1 (ERS1)"]
    #[inline]
    pub fn exs1b(&mut self) -> _EXS1BW {
        _EXS1BW { w: self }
    }
    #[doc = "Bits 8:9 - Event Source Select for A2 (ERS2)"]
    #[inline]
    pub fn exs2a(&mut self) -> _EXS2AW {
        _EXS2AW { w: self }
    }
    #[doc = "Bits 10:11 - Event Source Select for B2 (ERS2)"]
    #[inline]
    pub fn exs2b(&mut self) -> _EXS2BW {
        _EXS2BW { w: self }
    }
    #[doc = "Bits 12:13 - Event Source Select for A3 (ERS3)"]
    #[inline]
    pub fn exs3a(&mut self) -> _EXS3AW {
        _EXS3AW { w: self }
    }
    #[doc = "Bits 14:15 - Event Source Select for B3 (ERS3)"]
    #[inline]
    pub fn exs3b(&mut self) -> _EXS3BW {
        _EXS3BW { w: self }
    }
}
