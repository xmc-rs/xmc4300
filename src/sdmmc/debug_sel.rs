#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DEBUG_SEL {
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
#[doc = "Values that can be written to the field `DEBUG_SEL`"]
pub enum DEBUG_SELW {
    #[doc = "receiver module and fifo_ctrl module signals are probed out"]
    VALUE1,
    #[doc = "cmd register, Interrupt status, transmitter module and clk sdcard signals are probed out."]
    VALUE2,
}
impl DEBUG_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DEBUG_SELW::VALUE1 => false,
            DEBUG_SELW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DEBUG_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _DEBUG_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DEBUG_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "receiver module and fifo_ctrl module signals are probed out"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DEBUG_SELW::VALUE1)
    }
    #[doc = "cmd register, Interrupt status, transmitter module and clk sdcard signals are probed out."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DEBUG_SELW::VALUE2)
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
    #[doc = "Bit 0 - Debug_sel"]
    #[inline]
    pub fn debug_sel(&mut self) -> _DEBUG_SELW {
        _DEBUG_SELW { w: self }
    }
}
