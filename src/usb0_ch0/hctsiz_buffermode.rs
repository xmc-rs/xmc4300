#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HCTSIZ_BUFFERMODE {
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
    bits: u32,
}
impl XFERSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PKTCNTR {
    bits: u16,
}
impl PKTCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `Pid`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIDR {
    #[doc = "DATA0"]
    VALUE1,
    #[doc = "DATA2"]
    VALUE2,
    #[doc = "DATA1"]
    VALUE3,
    #[doc = "MDATA (non-control)/SETUP (control)"]
    VALUE4,
}
impl PIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PIDR::VALUE1 => 0,
            PIDR::VALUE2 => 1,
            PIDR::VALUE3 => 2,
            PIDR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PIDR {
        match value {
            0 => PIDR::VALUE1,
            1 => PIDR::VALUE2,
            2 => PIDR::VALUE3,
            3 => PIDR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PIDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PIDR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PIDR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PIDR::VALUE4
    }
}
#[doc = r" Proxy"]
pub struct _XFERSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _XFERSIZEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 524287;
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
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `Pid`"]
pub enum PIDW {
    #[doc = "DATA0"]
    VALUE1,
    #[doc = "DATA2"]
    VALUE2,
    #[doc = "DATA1"]
    VALUE3,
    #[doc = "MDATA (non-control)/SETUP (control)"]
    VALUE4,
}
impl PIDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PIDW::VALUE1 => 0,
            PIDW::VALUE2 => 1,
            PIDW::VALUE3 => 2,
            PIDW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIDW<'a> {
    w: &'a mut W,
}
impl<'a> _PIDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "DATA0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PIDW::VALUE1)
    }
    #[doc = "DATA2"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PIDW::VALUE2)
    }
    #[doc = "DATA1"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PIDW::VALUE3)
    }
    #[doc = "MDATA (non-control)/SETUP (control)"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(PIDW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline]
    pub fn xfer_size(&self) -> XFERSIZER {
        let bits = {
            const MASK: u32 = 524287;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        XFERSIZER { bits }
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline]
    pub fn pkt_cnt(&self) -> PKTCNTR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        PKTCNTR { bits }
    }
    #[doc = "Bits 29:30 - PID"]
    #[inline]
    pub fn pid(&self) -> PIDR {
        PIDR::_from({
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
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline]
    pub fn xfer_size(&mut self) -> _XFERSIZEW {
        _XFERSIZEW { w: self }
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline]
    pub fn pkt_cnt(&mut self) -> _PKTCNTW {
        _PKTCNTW { w: self }
    }
    #[doc = "Bits 29:30 - PID"]
    #[inline]
    pub fn pid(&mut self) -> _PIDW {
        _PIDW { w: self }
    }
}
