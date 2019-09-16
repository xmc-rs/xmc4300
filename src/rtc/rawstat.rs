#[doc = "Reader of register RAWSTAT"]
pub type R = crate::R<u32, super::RAWSTAT>;
#[doc = "Reader of field `RPSE`"]
pub type RPSE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RPMI`"]
pub type RPMI_R = crate::R<bool, bool>;
#[doc = "Reader of field `RPHO`"]
pub type RPHO_R = crate::R<bool, bool>;
#[doc = "Reader of field `RPDA`"]
pub type RPDA_R = crate::R<bool, bool>;
#[doc = "Reader of field `RPMO`"]
pub type RPMO_R = crate::R<bool, bool>;
#[doc = "Reader of field `RPYE`"]
pub type RPYE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RAI`"]
pub type RAI_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Raw Periodic Seconds Service Request"]
    #[inline(always)]
    pub fn rpse(&self) -> RPSE_R {
        RPSE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Raw Periodic Minutes Service Request"]
    #[inline(always)]
    pub fn rpmi(&self) -> RPMI_R {
        RPMI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Raw Periodic Hours Service Request"]
    #[inline(always)]
    pub fn rpho(&self) -> RPHO_R {
        RPHO_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Raw Periodic Days Service Request"]
    #[inline(always)]
    pub fn rpda(&self) -> RPDA_R {
        RPDA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Raw Periodic Months Service Request"]
    #[inline(always)]
    pub fn rpmo(&self) -> RPMO_R {
        RPMO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Raw Periodic Years Service Request"]
    #[inline(always)]
    pub fn rpye(&self) -> RPYE_R {
        RPYE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Raw Alarm Service Request"]
    #[inline(always)]
    pub fn rai(&self) -> RAI_R {
        RAI_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
