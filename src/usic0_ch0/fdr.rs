#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FDR {
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
#[doc = r" Value of the field"]
pub struct STEPR {
    bits: u16,
}
impl STEPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `DM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMR {
    #[doc = "The divider is switched off, fFD = 0."]
    VALUE1,
    #[doc = "Normal divider mode selected."]
    VALUE2,
    #[doc = "Fractional divider mode selected."]
    VALUE3,
    #[doc = "The divider is switched off, fFD = 0."]
    VALUE4,
}
impl DMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DMR::VALUE1 => 0,
            DMR::VALUE2 => 1,
            DMR::VALUE3 => 2,
            DMR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DMR {
        match value {
            0 => DMR::VALUE1,
            1 => DMR::VALUE2,
            2 => DMR::VALUE3,
            3 => DMR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DMR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == DMR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == DMR::VALUE4
    }
}
#[doc = r" Value of the field"]
pub struct RESULTR {
    bits: u16,
}
impl RESULTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _STEPW<'a> {
    w: &'a mut W,
}
impl<'a> _STEPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DM`"]
pub enum DMW {
    #[doc = "The divider is switched off, fFD = 0."]
    VALUE1,
    #[doc = "Normal divider mode selected."]
    VALUE2,
    #[doc = "Fractional divider mode selected."]
    VALUE3,
    #[doc = "The divider is switched off, fFD = 0."]
    VALUE4,
}
impl DMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DMW::VALUE1 => 0,
            DMW::VALUE2 => 1,
            DMW::VALUE3 => 2,
            DMW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMW<'a> {
    w: &'a mut W,
}
impl<'a> _DMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The divider is switched off, fFD = 0."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DMW::VALUE1)
    }
    #[doc = "Normal divider mode selected."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DMW::VALUE2)
    }
    #[doc = "Fractional divider mode selected."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(DMW::VALUE3)
    }
    #[doc = "The divider is switched off, fFD = 0."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(DMW::VALUE4)
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
    #[doc = "Bits 0:9 - Step Value"]
    #[inline]
    pub fn step(&self) -> STEPR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        STEPR { bits }
    }
    #[doc = "Bits 14:15 - Divider Mode"]
    #[inline]
    pub fn dm(&self) -> DMR {
        DMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:25 - Result Value"]
    #[inline]
    pub fn result(&self) -> RESULTR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RESULTR { bits }
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
    #[doc = "Bits 0:9 - Step Value"]
    #[inline]
    pub fn step(&mut self) -> _STEPW {
        _STEPW { w: self }
    }
    #[doc = "Bits 14:15 - Divider Mode"]
    #[inline]
    pub fn dm(&mut self) -> _DMW {
        _DMW { w: self }
    }
}
