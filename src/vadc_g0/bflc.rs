#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BFLC {
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
#[doc = "Possible values of the field `BFM0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFM0R {
    #[doc = "Disable boundary flag, BFLy is not changed"]
    VALUE1,
    #[doc = "Always enable boundary flag (follow compare results)"]
    VALUE2,
    #[doc = "Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    VALUE3,
    #[doc = "Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BFM0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BFM0R::VALUE1 => 0,
            BFM0R::VALUE2 => 1,
            BFM0R::VALUE3 => 2,
            BFM0R::VALUE4 => 3,
            BFM0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BFM0R {
        match value {
            0 => BFM0R::VALUE1,
            1 => BFM0R::VALUE2,
            2 => BFM0R::VALUE3,
            3 => BFM0R::VALUE4,
            i => BFM0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BFM0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BFM0R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == BFM0R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == BFM0R::VALUE4
    }
}
#[doc = "Possible values of the field `BFM1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFM1R {
    #[doc = "Disable boundary flag, BFLy is not changed"]
    VALUE1,
    #[doc = "Always enable boundary flag (follow compare results)"]
    VALUE2,
    #[doc = "Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    VALUE3,
    #[doc = "Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BFM1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BFM1R::VALUE1 => 0,
            BFM1R::VALUE2 => 1,
            BFM1R::VALUE3 => 2,
            BFM1R::VALUE4 => 3,
            BFM1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BFM1R {
        match value {
            0 => BFM1R::VALUE1,
            1 => BFM1R::VALUE2,
            2 => BFM1R::VALUE3,
            3 => BFM1R::VALUE4,
            i => BFM1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BFM1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BFM1R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == BFM1R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == BFM1R::VALUE4
    }
}
#[doc = "Possible values of the field `BFM2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFM2R {
    #[doc = "Disable boundary flag, BFLy is not changed"]
    VALUE1,
    #[doc = "Always enable boundary flag (follow compare results)"]
    VALUE2,
    #[doc = "Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    VALUE3,
    #[doc = "Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BFM2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BFM2R::VALUE1 => 0,
            BFM2R::VALUE2 => 1,
            BFM2R::VALUE3 => 2,
            BFM2R::VALUE4 => 3,
            BFM2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BFM2R {
        match value {
            0 => BFM2R::VALUE1,
            1 => BFM2R::VALUE2,
            2 => BFM2R::VALUE3,
            3 => BFM2R::VALUE4,
            i => BFM2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BFM2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BFM2R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == BFM2R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == BFM2R::VALUE4
    }
}
#[doc = "Possible values of the field `BFM3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFM3R {
    #[doc = "Disable boundary flag, BFLy is not changed"]
    VALUE1,
    #[doc = "Always enable boundary flag (follow compare results)"]
    VALUE2,
    #[doc = "Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    VALUE3,
    #[doc = "Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BFM3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BFM3R::VALUE1 => 0,
            BFM3R::VALUE2 => 1,
            BFM3R::VALUE3 => 2,
            BFM3R::VALUE4 => 3,
            BFM3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BFM3R {
        match value {
            0 => BFM3R::VALUE1,
            1 => BFM3R::VALUE2,
            2 => BFM3R::VALUE3,
            3 => BFM3R::VALUE4,
            i => BFM3R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BFM3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BFM3R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == BFM3R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == BFM3R::VALUE4
    }
}
#[doc = "Values that can be written to the field `BFM0`"]
pub enum BFM0W {
    #[doc = "Disable boundary flag, BFLy is not changed"]
    VALUE1,
    #[doc = "Always enable boundary flag (follow compare results)"]
    VALUE2,
    #[doc = "Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    VALUE3,
    #[doc = "Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    VALUE4,
}
impl BFM0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BFM0W::VALUE1 => 0,
            BFM0W::VALUE2 => 1,
            BFM0W::VALUE3 => 2,
            BFM0W::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BFM0W<'a> {
    w: &'a mut W,
}
impl<'a> _BFM0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BFM0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disable boundary flag, BFLy is not changed"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFM0W::VALUE1)
    }
    #[doc = "Always enable boundary flag (follow compare results)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFM0W::VALUE2)
    }
    #[doc = "Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(BFM0W::VALUE3)
    }
    #[doc = "Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(BFM0W::VALUE4)
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
#[doc = "Values that can be written to the field `BFM1`"]
pub enum BFM1W {
    #[doc = "Disable boundary flag, BFLy is not changed"]
    VALUE1,
    #[doc = "Always enable boundary flag (follow compare results)"]
    VALUE2,
    #[doc = "Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    VALUE3,
    #[doc = "Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    VALUE4,
}
impl BFM1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BFM1W::VALUE1 => 0,
            BFM1W::VALUE2 => 1,
            BFM1W::VALUE3 => 2,
            BFM1W::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BFM1W<'a> {
    w: &'a mut W,
}
impl<'a> _BFM1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BFM1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disable boundary flag, BFLy is not changed"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFM1W::VALUE1)
    }
    #[doc = "Always enable boundary flag (follow compare results)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFM1W::VALUE2)
    }
    #[doc = "Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(BFM1W::VALUE3)
    }
    #[doc = "Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(BFM1W::VALUE4)
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
#[doc = "Values that can be written to the field `BFM2`"]
pub enum BFM2W {
    #[doc = "Disable boundary flag, BFLy is not changed"]
    VALUE1,
    #[doc = "Always enable boundary flag (follow compare results)"]
    VALUE2,
    #[doc = "Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    VALUE3,
    #[doc = "Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    VALUE4,
}
impl BFM2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BFM2W::VALUE1 => 0,
            BFM2W::VALUE2 => 1,
            BFM2W::VALUE3 => 2,
            BFM2W::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BFM2W<'a> {
    w: &'a mut W,
}
impl<'a> _BFM2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BFM2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disable boundary flag, BFLy is not changed"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFM2W::VALUE1)
    }
    #[doc = "Always enable boundary flag (follow compare results)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFM2W::VALUE2)
    }
    #[doc = "Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(BFM2W::VALUE3)
    }
    #[doc = "Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(BFM2W::VALUE4)
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
#[doc = "Values that can be written to the field `BFM3`"]
pub enum BFM3W {
    #[doc = "Disable boundary flag, BFLy is not changed"]
    VALUE1,
    #[doc = "Always enable boundary flag (follow compare results)"]
    VALUE2,
    #[doc = "Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    VALUE3,
    #[doc = "Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    VALUE4,
}
impl BFM3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BFM3W::VALUE1 => 0,
            BFM3W::VALUE2 => 1,
            BFM3W::VALUE3 => 2,
            BFM3W::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BFM3W<'a> {
    w: &'a mut W,
}
impl<'a> _BFM3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BFM3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disable boundary flag, BFLy is not changed"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFM3W::VALUE1)
    }
    #[doc = "Always enable boundary flag (follow compare results)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFM3W::VALUE2)
    }
    #[doc = "Enable boundary flag while gate of source 0 is active, clear BFLy while gate is inactive"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(BFM3W::VALUE3)
    }
    #[doc = "Enable boundary flag while gate of source 1 is active, clear BFLy while gate is inactive"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(BFM3W::VALUE4)
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
    #[doc = "Bits 0:3 - Boundary Flag y Mode Control"]
    #[inline]
    pub fn bfm0(&self) -> BFM0R {
        BFM0R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Boundary Flag y Mode Control"]
    #[inline]
    pub fn bfm1(&self) -> BFM1R {
        BFM1R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Boundary Flag y Mode Control"]
    #[inline]
    pub fn bfm2(&self) -> BFM2R {
        BFM2R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - Boundary Flag y Mode Control"]
    #[inline]
    pub fn bfm3(&self) -> BFM3R {
        BFM3R::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Boundary Flag y Mode Control"]
    #[inline]
    pub fn bfm0(&mut self) -> _BFM0W {
        _BFM0W { w: self }
    }
    #[doc = "Bits 4:7 - Boundary Flag y Mode Control"]
    #[inline]
    pub fn bfm1(&mut self) -> _BFM1W {
        _BFM1W { w: self }
    }
    #[doc = "Bits 8:11 - Boundary Flag y Mode Control"]
    #[inline]
    pub fn bfm2(&mut self) -> _BFM2W {
        _BFM2W { w: self }
    }
    #[doc = "Bits 12:15 - Boundary Flag y Mode Control"]
    #[inline]
    pub fn bfm3(&mut self) -> _BFM3W {
        _BFM3W { w: self }
    }
}
