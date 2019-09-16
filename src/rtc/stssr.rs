#[doc = "Reader of register STSSR"]
pub type R = crate::R<u32, super::STSSR>;
#[doc = "Reader of field `SPSE`"]
pub type SPSE_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPMI`"]
pub type SPMI_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPHO`"]
pub type SPHO_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPDA`"]
pub type SPDA_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPMO`"]
pub type SPMO_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPYE`"]
pub type SPYE_R = crate::R<bool, bool>;
#[doc = "Reader of field `SAI`"]
pub type SAI_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Periodic Seconds Service Request Status after Masking"]
    #[inline(always)]
    pub fn spse(&self) -> SPSE_R {
        SPSE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Periodic Minutes Service Request Status after Masking"]
    #[inline(always)]
    pub fn spmi(&self) -> SPMI_R {
        SPMI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Periodic Hours Service Request Status after Masking"]
    #[inline(always)]
    pub fn spho(&self) -> SPHO_R {
        SPHO_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Periodic Days Service Request Status after Masking"]
    #[inline(always)]
    pub fn spda(&self) -> SPDA_R {
        SPDA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Periodic Months Service Request Status after Masking"]
    #[inline(always)]
    pub fn spmo(&self) -> SPMO_R {
        SPMO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Periodic Years Service Request Status after Masking"]
    #[inline(always)]
    pub fn spye(&self) -> SPYE_R {
        SPYE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Alarm Service Request Status after Masking"]
    #[inline(always)]
    pub fn sai(&self) -> SAI_R {
        SAI_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
