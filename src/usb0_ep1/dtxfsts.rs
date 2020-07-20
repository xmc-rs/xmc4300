#[doc = "Reader of register DTXFSTS"]
pub type R = crate::R<u32, super::DTXFSTS>;
#[doc = "IN Endpoint TxFIFO Space Avail\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum INEPTXFSPCAVAIL_A {
    #[doc = "0: Endpoint TxFIFO is full"]
    VALUE1 = 0,
    #[doc = "1: 1 word available"]
    VALUE2 = 1,
    #[doc = "2: 2 words available"]
    VALUE3 = 2,
}
impl From<INEPTXFSPCAVAIL_A> for u16 {
    #[inline(always)]
    fn from(variant: INEPTXFSPCAVAIL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `INEPTxFSpcAvail`"]
pub type INEPTXFSPCAVAIL_R = crate::R<u16, INEPTXFSPCAVAIL_A>;
impl INEPTXFSPCAVAIL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, INEPTXFSPCAVAIL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(INEPTXFSPCAVAIL_A::VALUE1),
            1 => Val(INEPTXFSPCAVAIL_A::VALUE2),
            2 => Val(INEPTXFSPCAVAIL_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == INEPTXFSPCAVAIL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == INEPTXFSPCAVAIL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == INEPTXFSPCAVAIL_A::VALUE3
    }
}
impl R {
    #[doc = "Bits 0:15 - IN Endpoint TxFIFO Space Avail"]
    #[inline(always)]
    pub fn ineptx_fspc_avail(&self) -> INEPTXFSPCAVAIL_R {
        INEPTXFSPCAVAIL_R::new((self.bits & 0xffff) as u16)
    }
}
