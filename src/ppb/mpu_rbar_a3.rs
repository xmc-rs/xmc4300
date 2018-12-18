#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MPU_RBAR_A3 {
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
pub struct REGIONR {
    bits: u8,
}
impl REGIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `VALID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VALIDR {
    #[doc = "MPU_RNR not changed, and the processor: - updates the base address for the region specified in the MPU_RNR - ignores the value of the REGION field"]
    VALUE1,
    #[doc = "the processor: - updates the value of the MPU_RNR to the value of the REGION field - updates the base address for the region specified in the REGION field."]
    VALUE2,
}
impl VALIDR {
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
            VALIDR::VALUE1 => false,
            VALIDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VALIDR {
        match value {
            false => VALIDR::VALUE1,
            true => VALIDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VALIDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VALIDR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct ADDRR {
    bits: u32,
}
impl ADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _REGIONW<'a> {
    w: &'a mut W,
}
impl<'a> _REGIONW<'a> {
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
#[doc = "Values that can be written to the field `VALID`"]
pub enum VALIDW {
    #[doc = "MPU_RNR not changed, and the processor: - updates the base address for the region specified in the MPU_RNR - ignores the value of the REGION field"]
    VALUE1,
    #[doc = "the processor: - updates the value of the MPU_RNR to the value of the REGION field - updates the base address for the region specified in the REGION field."]
    VALUE2,
}
impl VALIDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VALIDW::VALUE1 => false,
            VALIDW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VALIDW<'a> {
    w: &'a mut W,
}
impl<'a> _VALIDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VALIDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MPU_RNR not changed, and the processor: - updates the base address for the region specified in the MPU_RNR - ignores the value of the REGION field"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(VALIDW::VALUE1)
    }
    #[doc = "the processor: - updates the value of the MPU_RNR to the value of the REGION field - updates the base address for the region specified in the REGION field."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(VALIDW::VALUE2)
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
#[doc = r" Proxy"]
pub struct _ADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 8388607;
        const OFFSET: u8 = 9;
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
    #[doc = "Bits 0:3 - MPU region field"]
    #[inline]
    pub fn region(&self) -> REGIONR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        REGIONR { bits }
    }
    #[doc = "Bit 4 - MPU Region Number valid bit"]
    #[inline]
    pub fn valid(&self) -> VALIDR {
        VALIDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 9:31 - Region base address field"]
    #[inline]
    pub fn addr(&self) -> ADDRR {
        let bits = {
            const MASK: u32 = 8388607;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        ADDRR { bits }
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
    #[doc = "Bits 0:3 - MPU region field"]
    #[inline]
    pub fn region(&mut self) -> _REGIONW {
        _REGIONW { w: self }
    }
    #[doc = "Bit 4 - MPU Region Number valid bit"]
    #[inline]
    pub fn valid(&mut self) -> _VALIDW {
        _VALIDW { w: self }
    }
    #[doc = "Bits 9:31 - Region base address field"]
    #[inline]
    pub fn addr(&mut self) -> _ADDRW {
        _ADDRW { w: self }
    }
}
