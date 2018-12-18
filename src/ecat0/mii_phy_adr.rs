#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::MII_PHY_ADR {
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
pub struct PHY_ADDRR {
    bits: u8,
}
impl PHY_ADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PHY_CADDR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHY_CADDRR {
    #[doc = "Show address of port 0 (offset)"]
    VALUE1,
    #[doc = "Show individual address of port x"]
    VALUE2,
}
impl PHY_CADDRR {
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
            PHY_CADDRR::VALUE1 => false,
            PHY_CADDRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PHY_CADDRR {
        match value {
            false => PHY_CADDRR::VALUE1,
            true => PHY_CADDRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PHY_CADDRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PHY_CADDRR::VALUE2
    }
}
#[doc = r" Proxy"]
pub struct _PHY_ADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _PHY_ADDRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PHY_CADDR`"]
pub enum PHY_CADDRW {
    #[doc = "Show address of port 0 (offset)"]
    VALUE1,
    #[doc = "Show individual address of port x"]
    VALUE2,
}
impl PHY_CADDRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PHY_CADDRW::VALUE1 => false,
            PHY_CADDRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PHY_CADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _PHY_CADDRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PHY_CADDRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Show address of port 0 (offset)"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PHY_CADDRW::VALUE1)
    }
    #[doc = "Show individual address of port x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PHY_CADDRW::VALUE2)
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
    #[doc = "Bits 0:4 - PHY Address"]
    #[inline]
    pub fn phy_addr(&self) -> PHY_ADDRR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        PHY_ADDRR { bits }
    }
    #[doc = "Bit 7 - Show configured PHY address of port 0-3 in registerECAT0_MII_CONT_STAT\\[7:3\\]. Select port x with bits \\[4:0\\] of this register (valid values are 0-3)"]
    #[inline]
    pub fn phy_caddr(&self) -> PHY_CADDRR {
        PHY_CADDRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - PHY Address"]
    #[inline]
    pub fn phy_addr(&mut self) -> _PHY_ADDRW {
        _PHY_ADDRW { w: self }
    }
    #[doc = "Bit 7 - Show configured PHY address of port 0-3 in registerECAT0_MII_CONT_STAT\\[7:3\\]. Select port x with bits \\[4:0\\] of this register (valid values are 0-3)"]
    #[inline]
    pub fn phy_caddr(&mut self) -> _PHY_CADDRW {
        _PHY_CADDRW { w: self }
    }
}
