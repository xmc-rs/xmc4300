#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BFLNP {
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
#[doc = "Possible values of the field `BFL0NP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFL0NPR {
    #[doc = "Select common bondary flag output 0"]
    VALUE1,
    #[doc = "Select common bondary flag output 3"]
    VALUE2,
    #[doc = "Select shared service request line 0"]
    VALUE3,
    #[doc = "Select shared service request line 3"]
    VALUE4,
    #[doc = "Disabled, no common output signal"]
    VALUE5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BFL0NPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BFL0NPR::VALUE1 => 0,
            BFL0NPR::VALUE2 => 3,
            BFL0NPR::VALUE3 => 4,
            BFL0NPR::VALUE4 => 7,
            BFL0NPR::VALUE5 => 15,
            BFL0NPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BFL0NPR {
        match value {
            0 => BFL0NPR::VALUE1,
            3 => BFL0NPR::VALUE2,
            4 => BFL0NPR::VALUE3,
            7 => BFL0NPR::VALUE4,
            15 => BFL0NPR::VALUE5,
            i => BFL0NPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BFL0NPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BFL0NPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == BFL0NPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == BFL0NPR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == BFL0NPR::VALUE5
    }
}
#[doc = "Possible values of the field `BFL1NP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFL1NPR {
    #[doc = "Select common bondary flag output 0"]
    VALUE1,
    #[doc = "Select common bondary flag output 3"]
    VALUE2,
    #[doc = "Select shared service request line 0"]
    VALUE3,
    #[doc = "Select shared service request line 3"]
    VALUE4,
    #[doc = "Disabled, no common output signal"]
    VALUE5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BFL1NPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BFL1NPR::VALUE1 => 0,
            BFL1NPR::VALUE2 => 3,
            BFL1NPR::VALUE3 => 4,
            BFL1NPR::VALUE4 => 7,
            BFL1NPR::VALUE5 => 15,
            BFL1NPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BFL1NPR {
        match value {
            0 => BFL1NPR::VALUE1,
            3 => BFL1NPR::VALUE2,
            4 => BFL1NPR::VALUE3,
            7 => BFL1NPR::VALUE4,
            15 => BFL1NPR::VALUE5,
            i => BFL1NPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BFL1NPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BFL1NPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == BFL1NPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == BFL1NPR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == BFL1NPR::VALUE5
    }
}
#[doc = "Possible values of the field `BFL2NP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFL2NPR {
    #[doc = "Select common bondary flag output 0"]
    VALUE1,
    #[doc = "Select common bondary flag output 3"]
    VALUE2,
    #[doc = "Select shared service request line 0"]
    VALUE3,
    #[doc = "Select shared service request line 3"]
    VALUE4,
    #[doc = "Disabled, no common output signal"]
    VALUE5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BFL2NPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BFL2NPR::VALUE1 => 0,
            BFL2NPR::VALUE2 => 3,
            BFL2NPR::VALUE3 => 4,
            BFL2NPR::VALUE4 => 7,
            BFL2NPR::VALUE5 => 15,
            BFL2NPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BFL2NPR {
        match value {
            0 => BFL2NPR::VALUE1,
            3 => BFL2NPR::VALUE2,
            4 => BFL2NPR::VALUE3,
            7 => BFL2NPR::VALUE4,
            15 => BFL2NPR::VALUE5,
            i => BFL2NPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BFL2NPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BFL2NPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == BFL2NPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == BFL2NPR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == BFL2NPR::VALUE5
    }
}
#[doc = "Possible values of the field `BFL3NP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFL3NPR {
    #[doc = "Select common bondary flag output 0"]
    VALUE1,
    #[doc = "Select common bondary flag output 3"]
    VALUE2,
    #[doc = "Select shared service request line 0"]
    VALUE3,
    #[doc = "Select shared service request line 3"]
    VALUE4,
    #[doc = "Disabled, no common output signal"]
    VALUE5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BFL3NPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BFL3NPR::VALUE1 => 0,
            BFL3NPR::VALUE2 => 3,
            BFL3NPR::VALUE3 => 4,
            BFL3NPR::VALUE4 => 7,
            BFL3NPR::VALUE5 => 15,
            BFL3NPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BFL3NPR {
        match value {
            0 => BFL3NPR::VALUE1,
            3 => BFL3NPR::VALUE2,
            4 => BFL3NPR::VALUE3,
            7 => BFL3NPR::VALUE4,
            15 => BFL3NPR::VALUE5,
            i => BFL3NPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BFL3NPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BFL3NPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == BFL3NPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == BFL3NPR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == BFL3NPR::VALUE5
    }
}
#[doc = "Values that can be written to the field `BFL0NP`"]
pub enum BFL0NPW {
    #[doc = "Select common bondary flag output 0"]
    VALUE1,
    #[doc = "Select common bondary flag output 3"]
    VALUE2,
    #[doc = "Select shared service request line 0"]
    VALUE3,
    #[doc = "Select shared service request line 3"]
    VALUE4,
    #[doc = "Disabled, no common output signal"]
    VALUE5,
}
impl BFL0NPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BFL0NPW::VALUE1 => 0,
            BFL0NPW::VALUE2 => 3,
            BFL0NPW::VALUE3 => 4,
            BFL0NPW::VALUE4 => 7,
            BFL0NPW::VALUE5 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BFL0NPW<'a> {
    w: &'a mut W,
}
impl<'a> _BFL0NPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BFL0NPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select common bondary flag output 0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFL0NPW::VALUE1)
    }
    #[doc = "Select common bondary flag output 3"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFL0NPW::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(BFL0NPW::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(BFL0NPW::VALUE4)
    }
    #[doc = "Disabled, no common output signal"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(BFL0NPW::VALUE5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BFL1NP`"]
pub enum BFL1NPW {
    #[doc = "Select common bondary flag output 0"]
    VALUE1,
    #[doc = "Select common bondary flag output 3"]
    VALUE2,
    #[doc = "Select shared service request line 0"]
    VALUE3,
    #[doc = "Select shared service request line 3"]
    VALUE4,
    #[doc = "Disabled, no common output signal"]
    VALUE5,
}
impl BFL1NPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BFL1NPW::VALUE1 => 0,
            BFL1NPW::VALUE2 => 3,
            BFL1NPW::VALUE3 => 4,
            BFL1NPW::VALUE4 => 7,
            BFL1NPW::VALUE5 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BFL1NPW<'a> {
    w: &'a mut W,
}
impl<'a> _BFL1NPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BFL1NPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select common bondary flag output 0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFL1NPW::VALUE1)
    }
    #[doc = "Select common bondary flag output 3"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFL1NPW::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(BFL1NPW::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(BFL1NPW::VALUE4)
    }
    #[doc = "Disabled, no common output signal"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(BFL1NPW::VALUE5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BFL2NP`"]
pub enum BFL2NPW {
    #[doc = "Select common bondary flag output 0"]
    VALUE1,
    #[doc = "Select common bondary flag output 3"]
    VALUE2,
    #[doc = "Select shared service request line 0"]
    VALUE3,
    #[doc = "Select shared service request line 3"]
    VALUE4,
    #[doc = "Disabled, no common output signal"]
    VALUE5,
}
impl BFL2NPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BFL2NPW::VALUE1 => 0,
            BFL2NPW::VALUE2 => 3,
            BFL2NPW::VALUE3 => 4,
            BFL2NPW::VALUE4 => 7,
            BFL2NPW::VALUE5 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BFL2NPW<'a> {
    w: &'a mut W,
}
impl<'a> _BFL2NPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BFL2NPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select common bondary flag output 0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFL2NPW::VALUE1)
    }
    #[doc = "Select common bondary flag output 3"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFL2NPW::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(BFL2NPW::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(BFL2NPW::VALUE4)
    }
    #[doc = "Disabled, no common output signal"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(BFL2NPW::VALUE5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BFL3NP`"]
pub enum BFL3NPW {
    #[doc = "Select common bondary flag output 0"]
    VALUE1,
    #[doc = "Select common bondary flag output 3"]
    VALUE2,
    #[doc = "Select shared service request line 0"]
    VALUE3,
    #[doc = "Select shared service request line 3"]
    VALUE4,
    #[doc = "Disabled, no common output signal"]
    VALUE5,
}
impl BFL3NPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BFL3NPW::VALUE1 => 0,
            BFL3NPW::VALUE2 => 3,
            BFL3NPW::VALUE3 => 4,
            BFL3NPW::VALUE4 => 7,
            BFL3NPW::VALUE5 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BFL3NPW<'a> {
    w: &'a mut W,
}
impl<'a> _BFL3NPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BFL3NPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select common bondary flag output 0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFL3NPW::VALUE1)
    }
    #[doc = "Select common bondary flag output 3"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFL3NPW::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(BFL3NPW::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(BFL3NPW::VALUE4)
    }
    #[doc = "Disabled, no common output signal"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(BFL3NPW::VALUE5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
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
    #[doc = "Bits 0:3 - Boundary Flag y Node Pointer"]
    #[inline]
    pub fn bfl0np(&self) -> BFL0NPR {
        BFL0NPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Boundary Flag y Node Pointer"]
    #[inline]
    pub fn bfl1np(&self) -> BFL1NPR {
        BFL1NPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Boundary Flag y Node Pointer"]
    #[inline]
    pub fn bfl2np(&self) -> BFL2NPR {
        BFL2NPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - Boundary Flag y Node Pointer"]
    #[inline]
    pub fn bfl3np(&self) -> BFL3NPR {
        BFL3NPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 65535 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Boundary Flag y Node Pointer"]
    #[inline]
    pub fn bfl0np(&mut self) -> _BFL0NPW {
        _BFL0NPW { w: self }
    }
    #[doc = "Bits 4:7 - Boundary Flag y Node Pointer"]
    #[inline]
    pub fn bfl1np(&mut self) -> _BFL1NPW {
        _BFL1NPW { w: self }
    }
    #[doc = "Bits 8:11 - Boundary Flag y Node Pointer"]
    #[inline]
    pub fn bfl2np(&mut self) -> _BFL2NPW {
        _BFL2NPW { w: self }
    }
    #[doc = "Bits 12:15 - Boundary Flag y Node Pointer"]
    #[inline]
    pub fn bfl3np(&mut self) -> _BFL3NPW {
        _BFL3NPW { w: self }
    }
}
