#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TRAPSET {
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
}
#[doc = "Values that can be written to the field `SOSCWDGT`"]
pub enum SOSCWDGTW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Set trap request"]
    CONST_1,
}
impl SOSCWDGTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SOSCWDGTW::CONST_0 => false,
            SOSCWDGTW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SOSCWDGTW<'a> {
    w: &'a mut W,
}
impl<'a> _SOSCWDGTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SOSCWDGTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(SOSCWDGTW::CONST_0)
    }
    #[doc = "Set trap request"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(SOSCWDGTW::CONST_1)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SVCOLCKT`"]
pub enum SVCOLCKTW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Set trap request"]
    CONST_1,
}
impl SVCOLCKTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SVCOLCKTW::CONST_0 => false,
            SVCOLCKTW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SVCOLCKTW<'a> {
    w: &'a mut W,
}
impl<'a> _SVCOLCKTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SVCOLCKTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(SVCOLCKTW::CONST_0)
    }
    #[doc = "Set trap request"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(SVCOLCKTW::CONST_1)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UVCOLCKT`"]
pub enum UVCOLCKTW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Set trap request"]
    CONST_1,
}
impl UVCOLCKTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UVCOLCKTW::CONST_0 => false,
            UVCOLCKTW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UVCOLCKTW<'a> {
    w: &'a mut W,
}
impl<'a> _UVCOLCKTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UVCOLCKTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(UVCOLCKTW::CONST_0)
    }
    #[doc = "Set trap request"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(UVCOLCKTW::CONST_1)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PET`"]
pub enum PETW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Set trap request"]
    CONST_1,
}
impl PETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PETW::CONST_0 => false,
            PETW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PETW<'a> {
    w: &'a mut W,
}
impl<'a> _PETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PETW::CONST_0)
    }
    #[doc = "Set trap request"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PETW::CONST_1)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BRWNT`"]
pub enum BRWNTW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Set trap request"]
    CONST_1,
}
impl BRWNTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BRWNTW::CONST_0 => false,
            BRWNTW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BRWNTW<'a> {
    w: &'a mut W,
}
impl<'a> _BRWNTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BRWNTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(BRWNTW::CONST_0)
    }
    #[doc = "Set trap request"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(BRWNTW::CONST_1)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ULPWDT`"]
pub enum ULPWDTW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Set trap request"]
    CONST_1,
}
impl ULPWDTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ULPWDTW::CONST_0 => false,
            ULPWDTW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ULPWDTW<'a> {
    w: &'a mut W,
}
impl<'a> _ULPWDTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ULPWDTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ULPWDTW::CONST_0)
    }
    #[doc = "Set trap request"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ULPWDTW::CONST_1)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BWERR0T`"]
pub enum BWERR0TW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Set trap request"]
    CONST_1,
}
impl BWERR0TW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BWERR0TW::CONST_0 => false,
            BWERR0TW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BWERR0TW<'a> {
    w: &'a mut W,
}
impl<'a> _BWERR0TW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BWERR0TW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(BWERR0TW::CONST_0)
    }
    #[doc = "Set trap request"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(BWERR0TW::CONST_1)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BWERR1T`"]
pub enum BWERR1TW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Set trap request"]
    CONST_1,
}
impl BWERR1TW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BWERR1TW::CONST_0 => false,
            BWERR1TW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BWERR1TW<'a> {
    w: &'a mut W,
}
impl<'a> _BWERR1TW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BWERR1TW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(BWERR1TW::CONST_0)
    }
    #[doc = "Set trap request"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(BWERR1TW::CONST_1)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ECAT0RST`"]
pub enum ECAT0RSTW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Set trap request"]
    CONST_1,
}
impl ECAT0RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ECAT0RSTW::CONST_0 => false,
            ECAT0RSTW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ECAT0RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _ECAT0RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ECAT0RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ECAT0RSTW::CONST_0)
    }
    #[doc = "Set trap request"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ECAT0RSTW::CONST_1)
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
    #[doc = "Bit 0 - OSC_HP Oscillator Watchdog Trap Set"]
    #[inline]
    pub fn soscwdgt(&mut self) -> _SOSCWDGTW {
        _SOSCWDGTW { w: self }
    }
    #[doc = "Bit 2 - System VCO Lock Trap Set"]
    #[inline]
    pub fn svcolckt(&mut self) -> _SVCOLCKTW {
        _SVCOLCKTW { w: self }
    }
    #[doc = "Bit 3 - USB VCO Lock Trap Set"]
    #[inline]
    pub fn uvcolckt(&mut self) -> _UVCOLCKTW {
        _UVCOLCKTW { w: self }
    }
    #[doc = "Bit 4 - Parity Error Trap Set"]
    #[inline]
    pub fn pet(&mut self) -> _PETW {
        _PETW { w: self }
    }
    #[doc = "Bit 5 - Brown Out Trap Set"]
    #[inline]
    pub fn brwnt(&mut self) -> _BRWNTW {
        _BRWNTW { w: self }
    }
    #[doc = "Bit 6 - OSC_ULP Oscillator Watchdog Trap Set"]
    #[inline]
    pub fn ulpwdt(&mut self) -> _ULPWDTW {
        _ULPWDTW { w: self }
    }
    #[doc = "Bit 7 - Peripheral Bridge 0 Trap Set"]
    #[inline]
    pub fn bwerr0t(&mut self) -> _BWERR0TW {
        _BWERR0TW { w: self }
    }
    #[doc = "Bit 8 - Peripheral Bridge 1 Trap Set"]
    #[inline]
    pub fn bwerr1t(&mut self) -> _BWERR1TW {
        _BWERR1TW { w: self }
    }
    #[doc = "Bit 16 - EtherCat Reset 0 Trap Set"]
    #[inline]
    pub fn ecat0rst(&mut self) -> _ECAT0RSTW {
        _ECAT0RSTW { w: self }
    }
}
