#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HFIR {
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
pub struct FRINTR {
    bits: u16,
}
impl FRINTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `HFIRRldCtrl`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFIRRLDCTRLR {
    #[doc = "HFIR cannot be reloaded dynamically"]
    VALUE1,
    #[doc = "HFIR can be dynamically reloaded during runtime"]
    VALUE2,
}
impl HFIRRLDCTRLR {
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
            HFIRRLDCTRLR::VALUE1 => false,
            HFIRRLDCTRLR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HFIRRLDCTRLR {
        match value {
            false => HFIRRLDCTRLR::VALUE1,
            true => HFIRRLDCTRLR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HFIRRLDCTRLR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HFIRRLDCTRLR::VALUE2
    }
}
#[doc = r" Proxy"]
pub struct _FRINTW<'a> {
    w: &'a mut W,
}
impl<'a> _FRINTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HFIRRldCtrl`"]
pub enum HFIRRLDCTRLW {
    #[doc = "HFIR cannot be reloaded dynamically"]
    VALUE1,
    #[doc = "HFIR can be dynamically reloaded during runtime"]
    VALUE2,
}
impl HFIRRLDCTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HFIRRLDCTRLW::VALUE1 => false,
            HFIRRLDCTRLW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HFIRRLDCTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _HFIRRLDCTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HFIRRLDCTRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HFIR cannot be reloaded dynamically"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HFIRRLDCTRLW::VALUE1)
    }
    #[doc = "HFIR can be dynamically reloaded during runtime"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HFIRRLDCTRLW::VALUE2)
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
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:15 - Frame Interval"]
    #[inline]
    pub fn fr_int(&self) -> FRINTR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        FRINTR { bits }
    }
    #[doc = "Bit 16 - Reload Control"]
    #[inline]
    pub fn hfirrld_ctrl(&self) -> HFIRRLDCTRLR {
        HFIRRLDCTRLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 60000 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:15 - Frame Interval"]
    #[inline]
    pub fn fr_int(&mut self) -> _FRINTW {
        _FRINTW { w: self }
    }
    #[doc = "Bit 16 - Reload Control"]
    #[inline]
    pub fn hfirrld_ctrl(&mut self) -> _HFIRRLDCTRLW {
        _HFIRRLDCTRLW { w: self }
    }
}
