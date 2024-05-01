#[doc = "Register `ESC_WR_PROTECT` reader"]
pub type R = crate::R<EscWrProtectSpec>;
#[doc = "Write protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EscWrProt {
    #[doc = "0: Protection disabled"]
    Value1 = 0,
    #[doc = "1: Protection enabled"]
    Value2 = 1,
}
impl From<EscWrProt> for bool {
    #[inline(always)]
    fn from(variant: EscWrProt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ESC_WR_PROT` reader - Write protect"]
pub type EscWrProtR = crate::BitReader<EscWrProt>;
impl EscWrProtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EscWrProt {
        match self.bits {
            false => EscWrProt::Value1,
            true => EscWrProt::Value2,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EscWrProt::Value1
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EscWrProt::Value2
    }
}
impl R {
    #[doc = "Bit 0 - Write protect"]
    #[inline(always)]
    pub fn esc_wr_prot(&self) -> EscWrProtR {
        EscWrProtR::new((self.bits & 1) != 0)
    }
}
#[doc = "ESC Write Protection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`esc_wr_protect::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EscWrProtectSpec;
impl crate::RegisterSpec for EscWrProtectSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`esc_wr_protect::R`](R) reader structure"]
impl crate::Readable for EscWrProtectSpec {}
#[doc = "`reset()` method sets ESC_WR_PROTECT to value 0"]
impl crate::Resettable for EscWrProtectSpec {
    const RESET_VALUE: u8 = 0;
}
