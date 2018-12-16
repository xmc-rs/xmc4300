#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SRS {
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
#[doc = "Possible values of the field `POSR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POSRR {
    #[doc = "Forward to CC4ySR0"]
    VALUE1,
    #[doc = "Forward to CC4ySR1"]
    VALUE2,
    #[doc = "Forward to CC4ySR2"]
    VALUE3,
    #[doc = "Forward to CC4ySR3"]
    VALUE4,
}
impl POSRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            POSRR::VALUE1 => 0,
            POSRR::VALUE2 => 1,
            POSRR::VALUE3 => 2,
            POSRR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> POSRR {
        match value {
            0 => POSRR::VALUE1,
            1 => POSRR::VALUE2,
            2 => POSRR::VALUE3,
            3 => POSRR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == POSRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == POSRR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == POSRR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == POSRR::VALUE4
    }
}
#[doc = "Possible values of the field `CMSR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMSRR {
    #[doc = "Forward to CC4ySR0"]
    VALUE1,
    #[doc = "Forward to CC4ySR1"]
    VALUE2,
    #[doc = "Forward to CC4ySR2"]
    VALUE3,
    #[doc = "Forward to CC4ySR3"]
    VALUE4,
}
impl CMSRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMSRR::VALUE1 => 0,
            CMSRR::VALUE2 => 1,
            CMSRR::VALUE3 => 2,
            CMSRR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMSRR {
        match value {
            0 => CMSRR::VALUE1,
            1 => CMSRR::VALUE2,
            2 => CMSRR::VALUE3,
            3 => CMSRR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CMSRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CMSRR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CMSRR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CMSRR::VALUE4
    }
}
#[doc = "Possible values of the field `E0SR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E0SRR {
    #[doc = "Forward to CC4ySR0"]
    VALUE1,
    #[doc = "Forward to CC4ySR1"]
    VALUE2,
    #[doc = "Forward to CC4ySR2"]
    VALUE3,
    #[doc = "Forward to CC4ySR3"]
    VALUE4,
}
impl E0SRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            E0SRR::VALUE1 => 0,
            E0SRR::VALUE2 => 1,
            E0SRR::VALUE3 => 2,
            E0SRR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> E0SRR {
        match value {
            0 => E0SRR::VALUE1,
            1 => E0SRR::VALUE2,
            2 => E0SRR::VALUE3,
            3 => E0SRR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == E0SRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == E0SRR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == E0SRR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == E0SRR::VALUE4
    }
}
#[doc = "Possible values of the field `E1SR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E1SRR {
    #[doc = "Forward to CC4ySR0"]
    VALUE1,
    #[doc = "Forward to CC4ySR1"]
    VALUE2,
    #[doc = "Forward to CC4ySR2"]
    VALUE3,
    #[doc = "Forward to CC4ySR3"]
    VALUE4,
}
impl E1SRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            E1SRR::VALUE1 => 0,
            E1SRR::VALUE2 => 1,
            E1SRR::VALUE3 => 2,
            E1SRR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> E1SRR {
        match value {
            0 => E1SRR::VALUE1,
            1 => E1SRR::VALUE2,
            2 => E1SRR::VALUE3,
            3 => E1SRR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == E1SRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == E1SRR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == E1SRR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == E1SRR::VALUE4
    }
}
#[doc = "Possible values of the field `E2SR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E2SRR {
    #[doc = "Forward to CC4ySR0"]
    VALUE1,
    #[doc = "Forward to CC4ySR1"]
    VALUE2,
    #[doc = "Forward to CC4ySR2"]
    VALUE3,
    #[doc = "Forward to CC4ySR3"]
    VALUE4,
}
impl E2SRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            E2SRR::VALUE1 => 0,
            E2SRR::VALUE2 => 1,
            E2SRR::VALUE3 => 2,
            E2SRR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> E2SRR {
        match value {
            0 => E2SRR::VALUE1,
            1 => E2SRR::VALUE2,
            2 => E2SRR::VALUE3,
            3 => E2SRR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == E2SRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == E2SRR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == E2SRR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == E2SRR::VALUE4
    }
}
#[doc = "Values that can be written to the field `POSR`"]
pub enum POSRW {
    #[doc = "Forward to CC4ySR0"]
    VALUE1,
    #[doc = "Forward to CC4ySR1"]
    VALUE2,
    #[doc = "Forward to CC4ySR2"]
    VALUE3,
    #[doc = "Forward to CC4ySR3"]
    VALUE4,
}
impl POSRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            POSRW::VALUE1 => 0,
            POSRW::VALUE2 => 1,
            POSRW::VALUE3 => 2,
            POSRW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POSRW<'a> {
    w: &'a mut W,
}
impl<'a> _POSRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POSRW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Forward to CC4ySR0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(POSRW::VALUE1)
    }
    #[doc = "Forward to CC4ySR1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(POSRW::VALUE2)
    }
    #[doc = "Forward to CC4ySR2"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(POSRW::VALUE3)
    }
    #[doc = "Forward to CC4ySR3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(POSRW::VALUE4)
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
#[doc = "Values that can be written to the field `CMSR`"]
pub enum CMSRW {
    #[doc = "Forward to CC4ySR0"]
    VALUE1,
    #[doc = "Forward to CC4ySR1"]
    VALUE2,
    #[doc = "Forward to CC4ySR2"]
    VALUE3,
    #[doc = "Forward to CC4ySR3"]
    VALUE4,
}
impl CMSRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMSRW::VALUE1 => 0,
            CMSRW::VALUE2 => 1,
            CMSRW::VALUE3 => 2,
            CMSRW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMSRW<'a> {
    w: &'a mut W,
}
impl<'a> _CMSRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMSRW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Forward to CC4ySR0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMSRW::VALUE1)
    }
    #[doc = "Forward to CC4ySR1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMSRW::VALUE2)
    }
    #[doc = "Forward to CC4ySR2"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CMSRW::VALUE3)
    }
    #[doc = "Forward to CC4ySR3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CMSRW::VALUE4)
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
#[doc = "Values that can be written to the field `E0SR`"]
pub enum E0SRW {
    #[doc = "Forward to CC4ySR0"]
    VALUE1,
    #[doc = "Forward to CC4ySR1"]
    VALUE2,
    #[doc = "Forward to CC4ySR2"]
    VALUE3,
    #[doc = "Forward to CC4ySR3"]
    VALUE4,
}
impl E0SRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            E0SRW::VALUE1 => 0,
            E0SRW::VALUE2 => 1,
            E0SRW::VALUE3 => 2,
            E0SRW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _E0SRW<'a> {
    w: &'a mut W,
}
impl<'a> _E0SRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: E0SRW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Forward to CC4ySR0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(E0SRW::VALUE1)
    }
    #[doc = "Forward to CC4ySR1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(E0SRW::VALUE2)
    }
    #[doc = "Forward to CC4ySR2"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(E0SRW::VALUE3)
    }
    #[doc = "Forward to CC4ySR3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(E0SRW::VALUE4)
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
#[doc = "Values that can be written to the field `E1SR`"]
pub enum E1SRW {
    #[doc = "Forward to CC4ySR0"]
    VALUE1,
    #[doc = "Forward to CC4ySR1"]
    VALUE2,
    #[doc = "Forward to CC4ySR2"]
    VALUE3,
    #[doc = "Forward to CC4ySR3"]
    VALUE4,
}
impl E1SRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            E1SRW::VALUE1 => 0,
            E1SRW::VALUE2 => 1,
            E1SRW::VALUE3 => 2,
            E1SRW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _E1SRW<'a> {
    w: &'a mut W,
}
impl<'a> _E1SRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: E1SRW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Forward to CC4ySR0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(E1SRW::VALUE1)
    }
    #[doc = "Forward to CC4ySR1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(E1SRW::VALUE2)
    }
    #[doc = "Forward to CC4ySR2"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(E1SRW::VALUE3)
    }
    #[doc = "Forward to CC4ySR3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(E1SRW::VALUE4)
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
#[doc = "Values that can be written to the field `E2SR`"]
pub enum E2SRW {
    #[doc = "Forward to CC4ySR0"]
    VALUE1,
    #[doc = "Forward to CC4ySR1"]
    VALUE2,
    #[doc = "Forward to CC4ySR2"]
    VALUE3,
    #[doc = "Forward to CC4ySR3"]
    VALUE4,
}
impl E2SRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            E2SRW::VALUE1 => 0,
            E2SRW::VALUE2 => 1,
            E2SRW::VALUE3 => 2,
            E2SRW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _E2SRW<'a> {
    w: &'a mut W,
}
impl<'a> _E2SRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: E2SRW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Forward to CC4ySR0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(E2SRW::VALUE1)
    }
    #[doc = "Forward to CC4ySR1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(E2SRW::VALUE2)
    }
    #[doc = "Forward to CC4ySR2"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(E2SRW::VALUE3)
    }
    #[doc = "Forward to CC4ySR3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(E2SRW::VALUE4)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Period/One match Service request selector"]
    #[inline]
    pub fn posr(&self) -> POSRR {
        POSRR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Compare match Service request selector"]
    #[inline]
    pub fn cmsr(&self) -> CMSRR {
        CMSRR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Event 0 Service request selector"]
    #[inline]
    pub fn e0sr(&self) -> E0SRR {
        E0SRR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Event 1 Service request selector"]
    #[inline]
    pub fn e1sr(&self) -> E1SRR {
        E1SRR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Event 2 Service request selector"]
    #[inline]
    pub fn e2sr(&self) -> E2SRR {
        E2SRR::_from({
            const MASK: u8 = 3;
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
    #[doc = "Bits 0:1 - Period/One match Service request selector"]
    #[inline]
    pub fn posr(&mut self) -> _POSRW {
        _POSRW { w: self }
    }
    #[doc = "Bits 2:3 - Compare match Service request selector"]
    #[inline]
    pub fn cmsr(&mut self) -> _CMSRW {
        _CMSRW { w: self }
    }
    #[doc = "Bits 8:9 - Event 0 Service request selector"]
    #[inline]
    pub fn e0sr(&mut self) -> _E0SRW {
        _E0SRW { w: self }
    }
    #[doc = "Bits 10:11 - Event 1 Service request selector"]
    #[inline]
    pub fn e1sr(&mut self) -> _E1SRW {
        _E1SRW { w: self }
    }
    #[doc = "Bits 12:13 - Event 2 Service request selector"]
    #[inline]
    pub fn e2sr(&mut self) -> _E2SRW {
        _E2SRW { w: self }
    }
}
