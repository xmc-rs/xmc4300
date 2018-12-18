#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DTSCON {
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
#[doc = "Possible values of the field `PWD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWDR {
    #[doc = "The DTS is powered"]
    CONST_0,
    #[doc = "The DTS is not powered"]
    CONST_1,
}
impl PWDR {
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
            PWDR::CONST_0 => false,
            PWDR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWDR {
        match value {
            false => PWDR::CONST_0,
            true => PWDR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PWDR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PWDR::CONST_1
    }
}
#[doc = r" Value of the field"]
pub struct OFFSETR {
    bits: u8,
}
impl OFFSETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GAINR {
    bits: u8,
}
impl GAINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct REFTRIMR {
    bits: u8,
}
impl REFTRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BGTRIMR {
    bits: u8,
}
impl BGTRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `PWD`"]
pub enum PWDW {
    #[doc = "The DTS is powered"]
    CONST_0,
    #[doc = "The DTS is not powered"]
    CONST_1,
}
impl PWDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWDW::CONST_0 => false,
            PWDW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWDW<'a> {
    w: &'a mut W,
}
impl<'a> _PWDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DTS is powered"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PWDW::CONST_0)
    }
    #[doc = "The DTS is not powered"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PWDW::CONST_1)
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
#[doc = "Values that can be written to the field `START`"]
pub enum STARTW {
    #[doc = "No DTS measurement is started"]
    CONST_0,
    #[doc = "DTS measurement is started"]
    CONST_1,
}
impl STARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STARTW::CONST_0 => false,
            STARTW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STARTW<'a> {
    w: &'a mut W,
}
impl<'a> _STARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STARTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No DTS measurement is started"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(STARTW::CONST_0)
    }
    #[doc = "DTS measurement is started"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(STARTW::CONST_1)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OFFSETW<'a> {
    w: &'a mut W,
}
impl<'a> _OFFSETW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GAINW<'a> {
    w: &'a mut W,
}
impl<'a> _GAINW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _REFTRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _REFTRIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BGTRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _BGTRIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
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
    #[doc = "Bit 0 - Sensor Power Down"]
    #[inline]
    pub fn pwd(&self) -> PWDR {
        PWDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:10 - Offset Calibration Value"]
    #[inline]
    pub fn offset(&self) -> OFFSETR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OFFSETR { bits }
    }
    #[doc = "Bits 11:16 - Gain Calibration Value"]
    #[inline]
    pub fn gain(&self) -> GAINR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GAINR { bits }
    }
    #[doc = "Bits 17:19 - Reference Trim Calibration Value"]
    #[inline]
    pub fn reftrim(&self) -> REFTRIMR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        REFTRIMR { bits }
    }
    #[doc = "Bits 20:23 - Bandgap Trim Calibration Value"]
    #[inline]
    pub fn bgtrim(&self) -> BGTRIMR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BGTRIMR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Sensor Power Down"]
    #[inline]
    pub fn pwd(&mut self) -> _PWDW {
        _PWDW { w: self }
    }
    #[doc = "Bit 1 - Sensor Measurement Start"]
    #[inline]
    pub fn start(&mut self) -> _STARTW {
        _STARTW { w: self }
    }
    #[doc = "Bits 4:10 - Offset Calibration Value"]
    #[inline]
    pub fn offset(&mut self) -> _OFFSETW {
        _OFFSETW { w: self }
    }
    #[doc = "Bits 11:16 - Gain Calibration Value"]
    #[inline]
    pub fn gain(&mut self) -> _GAINW {
        _GAINW { w: self }
    }
    #[doc = "Bits 17:19 - Reference Trim Calibration Value"]
    #[inline]
    pub fn reftrim(&mut self) -> _REFTRIMW {
        _REFTRIMW { w: self }
    }
    #[doc = "Bits 20:23 - Bandgap Trim Calibration Value"]
    #[inline]
    pub fn bgtrim(&mut self) -> _BGTRIMW {
        _BGTRIMW { w: self }
    }
}
