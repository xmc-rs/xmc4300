#[doc = "Reader of register GNPTXSTS"]
pub type R = crate::R<u32, super::GNPTXSTS>;
#[doc = "Non-periodic TxFIFO Space Avail\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NPTXFSPCAVAIL_A {
    #[doc = "0: Non-periodic TxFIFO is full"]
    VALUE1,
    #[doc = "1: 1 word available"]
    VALUE2,
    #[doc = "2: 2 words available"]
    VALUE3,
}
impl From<NPTXFSPCAVAIL_A> for u16 {
    #[inline(always)]
    fn from(variant: NPTXFSPCAVAIL_A) -> Self {
        match variant {
            NPTXFSPCAVAIL_A::VALUE1 => 0,
            NPTXFSPCAVAIL_A::VALUE2 => 1,
            NPTXFSPCAVAIL_A::VALUE3 => 2,
        }
    }
}
#[doc = "Reader of field `NPTxFSpcAvail`"]
pub type NPTXFSPCAVAIL_R = crate::R<u16, NPTXFSPCAVAIL_A>;
impl NPTXFSPCAVAIL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, NPTXFSPCAVAIL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(NPTXFSPCAVAIL_A::VALUE1),
            1 => Val(NPTXFSPCAVAIL_A::VALUE2),
            2 => Val(NPTXFSPCAVAIL_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NPTXFSPCAVAIL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NPTXFSPCAVAIL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == NPTXFSPCAVAIL_A::VALUE3
    }
}
#[doc = "Non-periodic Transmit Request Queue Space Available\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NPTXQSPCAVAIL_A {
    #[doc = "0: Non-periodic Transmit Request Queue is full"]
    VALUE1,
    #[doc = "1: 1 location available"]
    VALUE2,
    #[doc = "2: 2 locations available"]
    VALUE3,
}
impl From<NPTXQSPCAVAIL_A> for u8 {
    #[inline(always)]
    fn from(variant: NPTXQSPCAVAIL_A) -> Self {
        match variant {
            NPTXQSPCAVAIL_A::VALUE1 => 0,
            NPTXQSPCAVAIL_A::VALUE2 => 1,
            NPTXQSPCAVAIL_A::VALUE3 => 2,
        }
    }
}
#[doc = "Reader of field `NPTxQSpcAvail`"]
pub type NPTXQSPCAVAIL_R = crate::R<u8, NPTXQSPCAVAIL_A>;
impl NPTXQSPCAVAIL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, NPTXQSPCAVAIL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(NPTXQSPCAVAIL_A::VALUE1),
            1 => Val(NPTXQSPCAVAIL_A::VALUE2),
            2 => Val(NPTXQSPCAVAIL_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NPTXQSPCAVAIL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NPTXQSPCAVAIL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == NPTXQSPCAVAIL_A::VALUE3
    }
}
#[doc = "Top of the Non-periodic Transmit Request Queue\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NPTXQTOP_A {
    #[doc = "0: IN/OUT token"]
    VALUE1,
    #[doc = "1: Zero-length transmit packet (device IN/host OUT)"]
    VALUE2,
    #[doc = "3: Channel halt command"]
    VALUE4,
}
impl From<NPTXQTOP_A> for u8 {
    #[inline(always)]
    fn from(variant: NPTXQTOP_A) -> Self {
        match variant {
            NPTXQTOP_A::VALUE1 => 0,
            NPTXQTOP_A::VALUE2 => 1,
            NPTXQTOP_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `NPTxQTop`"]
pub type NPTXQTOP_R = crate::R<u8, NPTXQTOP_A>;
impl NPTXQTOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, NPTXQTOP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(NPTXQTOP_A::VALUE1),
            1 => Val(NPTXQTOP_A::VALUE2),
            3 => Val(NPTXQTOP_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NPTXQTOP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NPTXQTOP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == NPTXQTOP_A::VALUE4
    }
}
impl R {
    #[doc = "Bits 0:15 - Non-periodic TxFIFO Space Avail"]
    #[inline(always)]
    pub fn nptx_fspc_avail(&self) -> NPTXFSPCAVAIL_R {
        NPTXFSPCAVAIL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Non-periodic Transmit Request Queue Space Available"]
    #[inline(always)]
    pub fn nptx_qspc_avail(&self) -> NPTXQSPCAVAIL_R {
        NPTXQSPCAVAIL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:30 - Top of the Non-periodic Transmit Request Queue"]
    #[inline(always)]
    pub fn nptx_qtop(&self) -> NPTXQTOP_R {
        NPTXQTOP_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
