#[doc = "Reader of register HPTXSTS"]
pub type R = crate::R<u32, super::HPTXSTS>;
#[doc = "Writer for register HPTXSTS"]
pub type W = crate::W<u32, super::HPTXSTS>;
#[doc = "Register HPTXSTS `reset()`'s with value 0x0008_0100"]
impl crate::ResetValue for super::HPTXSTS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0008_0100
    }
}
#[doc = "Periodic Transmit Data FIFO Space Available\n\nValue on reset: 256"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTXFSPCAVAIL_A {
    #[doc = "0: Periodic TxFIFO is full"]
    VALUE1,
    #[doc = "1: 1 word available"]
    VALUE2,
    #[doc = "2: 2 words available"]
    VALUE3,
}
impl From<PTXFSPCAVAIL_A> for u16 {
    #[inline(always)]
    fn from(variant: PTXFSPCAVAIL_A) -> Self {
        match variant {
            PTXFSPCAVAIL_A::VALUE1 => 0,
            PTXFSPCAVAIL_A::VALUE2 => 1,
            PTXFSPCAVAIL_A::VALUE3 => 2,
        }
    }
}
#[doc = "Reader of field `PTxFSpcAvail`"]
pub type PTXFSPCAVAIL_R = crate::R<u16, PTXFSPCAVAIL_A>;
impl PTXFSPCAVAIL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, PTXFSPCAVAIL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PTXFSPCAVAIL_A::VALUE1),
            1 => Val(PTXFSPCAVAIL_A::VALUE2),
            2 => Val(PTXFSPCAVAIL_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PTXFSPCAVAIL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PTXFSPCAVAIL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PTXFSPCAVAIL_A::VALUE3
    }
}
#[doc = "Write proxy for field `PTxFSpcAvail`"]
pub struct PTXFSPCAVAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> PTXFSPCAVAIL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTXFSPCAVAIL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Periodic TxFIFO is full"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PTXFSPCAVAIL_A::VALUE1)
    }
    #[doc = "1 word available"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PTXFSPCAVAIL_A::VALUE2)
    }
    #[doc = "2 words available"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PTXFSPCAVAIL_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Periodic Transmit Request Queue Space Available\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTXQSPCAVAIL_A {
    #[doc = "0: Periodic Transmit Request Queue is full"]
    VALUE1,
    #[doc = "1: 1 location available"]
    VALUE2,
    #[doc = "2: 2 locations available"]
    VALUE3,
}
impl From<PTXQSPCAVAIL_A> for u8 {
    #[inline(always)]
    fn from(variant: PTXQSPCAVAIL_A) -> Self {
        match variant {
            PTXQSPCAVAIL_A::VALUE1 => 0,
            PTXQSPCAVAIL_A::VALUE2 => 1,
            PTXQSPCAVAIL_A::VALUE3 => 2,
        }
    }
}
#[doc = "Reader of field `PTxQSpcAvail`"]
pub type PTXQSPCAVAIL_R = crate::R<u8, PTXQSPCAVAIL_A>;
impl PTXQSPCAVAIL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PTXQSPCAVAIL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PTXQSPCAVAIL_A::VALUE1),
            1 => Val(PTXQSPCAVAIL_A::VALUE2),
            2 => Val(PTXQSPCAVAIL_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PTXQSPCAVAIL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PTXQSPCAVAIL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PTXQSPCAVAIL_A::VALUE3
    }
}
#[doc = "Reader of field `PTxQTop`"]
pub type PTXQTOP_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - Periodic Transmit Data FIFO Space Available"]
    #[inline(always)]
    pub fn ptx_fspc_avail(&self) -> PTXFSPCAVAIL_R {
        PTXFSPCAVAIL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Periodic Transmit Request Queue Space Available"]
    #[inline(always)]
    pub fn ptx_qspc_avail(&self) -> PTXQSPCAVAIL_R {
        PTXQSPCAVAIL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Top of the Periodic Transmit Request Queue"]
    #[inline(always)]
    pub fn ptx_qtop(&self) -> PTXQTOP_R {
        PTXQTOP_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Periodic Transmit Data FIFO Space Available"]
    #[inline(always)]
    pub fn ptx_fspc_avail(&mut self) -> PTXFSPCAVAIL_W {
        PTXFSPCAVAIL_W { w: self }
    }
}
