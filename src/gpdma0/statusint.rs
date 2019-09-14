#[doc = "Reader of register STATUSINT"]
pub type R = crate::R<u32, super::STATUSINT>;
#[doc = "Reader of field `ERR`"]
pub type ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DSTT`"]
pub type DSTT_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRCT`"]
pub type SRCT_R = crate::R<bool, bool>;
#[doc = "Reader of field `BLOCK`"]
pub type BLOCK_R = crate::R<bool, bool>;
#[doc = "Reader of field `TFR`"]
pub type TFR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 4 - OR of the contents of STATUSERR register"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - OR of the contents of STATUSDSTTRAN register"]
    #[inline(always)]
    pub fn dstt(&self) -> DSTT_R {
        DSTT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - OR of the contents of STATUSSRCTRAN register"]
    #[inline(always)]
    pub fn srct(&self) -> SRCT_R {
        SRCT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - OR of the contents of STATUSBLOCK register"]
    #[inline(always)]
    pub fn block(&self) -> BLOCK_R {
        BLOCK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - OR of the contents of STATUSTFR register"]
    #[inline(always)]
    pub fn tfr(&self) -> TFR_R {
        TFR_R::new((self.bits & 0x01) != 0)
    }
}
