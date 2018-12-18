#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::SM_PDI_CTR {
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
#[doc = "Possible values of the field `DEACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEACTR {
    #[doc = "Read 0 for Normal operation, SyncManager activated, write 0 for Activate SyncManager"]
    VALUE1,
    #[doc = "Read 1 for SyncManager deactivated and reset SyncManager locks access to Memory area, write 1 for Request SyncManager deactivation"]
    VALUE2,
}
impl DEACTR {
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
            DEACTR::VALUE1 => false,
            DEACTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DEACTR {
        match value {
            false => DEACTR::VALUE1,
            true => DEACTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DEACTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DEACTR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct REP_ACKR {
    bits: bool,
}
impl REP_ACKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Values that can be written to the field `DEACT`"]
pub enum DEACTW {
    #[doc = "Read 0 for Normal operation, SyncManager activated, write 0 for Activate SyncManager"]
    VALUE1,
    #[doc = "Read 1 for SyncManager deactivated and reset SyncManager locks access to Memory area, write 1 for Request SyncManager deactivation"]
    VALUE2,
}
impl DEACTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DEACTW::VALUE1 => false,
            DEACTW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DEACTW<'a> {
    w: &'a mut W,
}
impl<'a> _DEACTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DEACTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read 0 for Normal operation, SyncManager activated, write 0 for Activate SyncManager"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DEACTW::VALUE1)
    }
    #[doc = "Read 1 for SyncManager deactivated and reset SyncManager locks access to Memory area, write 1 for Request SyncManager deactivation"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DEACTW::VALUE2)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _REP_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _REP_ACKW<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Deactivate SyncManager"]
    #[inline]
    pub fn deact(&self) -> DEACTR {
        DEACTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Repeat Ack"]
    #[inline]
    pub fn rep_ack(&self) -> REP_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        REP_ACKR { bits }
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Deactivate SyncManager"]
    #[inline]
    pub fn deact(&mut self) -> _DEACTW {
        _DEACTW { w: self }
    }
    #[doc = "Bit 1 - Repeat Ack"]
    #[inline]
    pub fn rep_ack(&mut self) -> _REP_ACKW {
        _REP_ACKW { w: self }
    }
}
