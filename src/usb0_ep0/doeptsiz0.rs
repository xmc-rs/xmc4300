#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DOEPTSIZ0 {
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
pub struct XFERSIZER {
    bits: u8,
}
impl XFERSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PKTCNTR {
    bits: u8,
}
impl PKTCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SUPCnt`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUPCNTR {
    #[doc = "1 packet"]
    VALUE1,
    #[doc = "2 packets"]
    VALUE2,
    #[doc = "3 packets"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SUPCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SUPCNTR::VALUE1 => 1,
            SUPCNTR::VALUE2 => 2,
            SUPCNTR::VALUE3 => 3,
            SUPCNTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SUPCNTR {
        match value {
            1 => SUPCNTR::VALUE1,
            2 => SUPCNTR::VALUE2,
            3 => SUPCNTR::VALUE3,
            i => SUPCNTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SUPCNTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SUPCNTR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == SUPCNTR::VALUE3
    }
}
#[doc = r" Proxy"]
pub struct _XFERSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _XFERSIZEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PKTCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _PKTCNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SUPCnt`"]
pub enum SUPCNTW {
    #[doc = "1 packet"]
    VALUE1,
    #[doc = "2 packets"]
    VALUE2,
    #[doc = "3 packets"]
    VALUE3,
}
impl SUPCNTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SUPCNTW::VALUE1 => 1,
            SUPCNTW::VALUE2 => 2,
            SUPCNTW::VALUE3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SUPCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _SUPCNTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SUPCNTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 packet"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SUPCNTW::VALUE1)
    }
    #[doc = "2 packets"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SUPCNTW::VALUE2)
    }
    #[doc = "3 packets"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(SUPCNTW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 29;
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
    #[doc = "Bits 0:6 - Transfer Size"]
    #[inline]
    pub fn xfer_size(&self) -> XFERSIZER {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        XFERSIZER { bits }
    }
    #[doc = "Bits 19:20 - Packet Count"]
    #[inline]
    pub fn pkt_cnt(&self) -> PKTCNTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PKTCNTR { bits }
    }
    #[doc = "Bits 29:30 - SETUP Packet Count"]
    #[inline]
    pub fn supcnt(&self) -> SUPCNTR {
        SUPCNTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 29;
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
    #[doc = "Bits 0:6 - Transfer Size"]
    #[inline]
    pub fn xfer_size(&mut self) -> _XFERSIZEW {
        _XFERSIZEW { w: self }
    }
    #[doc = "Bits 19:20 - Packet Count"]
    #[inline]
    pub fn pkt_cnt(&mut self) -> _PKTCNTW {
        _PKTCNTW { w: self }
    }
    #[doc = "Bits 29:30 - SETUP Packet Count"]
    #[inline]
    pub fn supcnt(&mut self) -> _SUPCNTW {
        _SUPCNTW { w: self }
    }
}
