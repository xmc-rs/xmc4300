#[doc = "Register `DOEPMSK` reader"]
pub struct R(crate::R<DOEPMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPMSK` writer"]
pub struct W(crate::W<DOEPMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DOEPMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XferComplMsk` reader - Transfer Completed Interrupt Mask"]
pub type XFER_COMPL_MSK_R = crate::BitReader<bool>;
#[doc = "Field `XferComplMsk` writer - Transfer Completed Interrupt Mask"]
pub type XFER_COMPL_MSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPMSK_SPEC, bool, O>;
#[doc = "Field `EPDisbldMsk` reader - Endpoint Disabled Interrupt Mask"]
pub type EPDISBLD_MSK_R = crate::BitReader<bool>;
#[doc = "Field `EPDisbldMsk` writer - Endpoint Disabled Interrupt Mask"]
pub type EPDISBLD_MSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPMSK_SPEC, bool, O>;
#[doc = "Field `AHBErrMsk` reader - AHB Error"]
pub type AHBERR_MSK_R = crate::BitReader<bool>;
#[doc = "Field `AHBErrMsk` writer - AHB Error"]
pub type AHBERR_MSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPMSK_SPEC, bool, O>;
#[doc = "Field `SetUPMsk` reader - SETUP Phase Done Mask"]
pub type SET_UPMSK_R = crate::BitReader<bool>;
#[doc = "Field `SetUPMsk` writer - SETUP Phase Done Mask"]
pub type SET_UPMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPMSK_SPEC, bool, O>;
#[doc = "Field `OUTTknEPdisMsk` reader - OUT Token Received when Endpoint Disabled Mask"]
pub type OUTTKN_EPDIS_MSK_R = crate::BitReader<bool>;
#[doc = "Field `OUTTknEPdisMsk` writer - OUT Token Received when Endpoint Disabled Mask"]
pub type OUTTKN_EPDIS_MSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPMSK_SPEC, bool, O>;
#[doc = "Field `Back2BackSETup` reader - Back-to-Back SETUP Packets Received Mask"]
pub type BACK2BACK_SETUP_R = crate::BitReader<bool>;
#[doc = "Field `Back2BackSETup` writer - Back-to-Back SETUP Packets Received Mask"]
pub type BACK2BACK_SETUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPMSK_SPEC, bool, O>;
#[doc = "Field `OutPktErrMsk` reader - OUT Packet Error Mask"]
pub type OUT_PKT_ERR_MSK_R = crate::BitReader<bool>;
#[doc = "Field `OutPktErrMsk` writer - OUT Packet Error Mask"]
pub type OUT_PKT_ERR_MSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPMSK_SPEC, bool, O>;
#[doc = "Field `BnaOutIntrMsk` reader - BNA interrupt Mask"]
pub type BNA_OUT_INTR_MSK_R = crate::BitReader<bool>;
#[doc = "Field `BnaOutIntrMsk` writer - BNA interrupt Mask"]
pub type BNA_OUT_INTR_MSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPMSK_SPEC, bool, O>;
#[doc = "Field `BbleErrMsk` reader - Babble Interrupt Mask"]
pub type BBLE_ERR_MSK_R = crate::BitReader<bool>;
#[doc = "Field `BbleErrMsk` writer - Babble Interrupt Mask"]
pub type BBLE_ERR_MSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPMSK_SPEC, bool, O>;
#[doc = "Field `NAKMsk` reader - NAK Interrupt Mask"]
pub type NAKMSK_R = crate::BitReader<bool>;
#[doc = "Field `NAKMsk` writer - NAK Interrupt Mask"]
pub type NAKMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPMSK_SPEC, bool, O>;
#[doc = "Field `NYETMsk` reader - NYET Interrupt Mask"]
pub type NYETMSK_R = crate::BitReader<bool>;
#[doc = "Field `NYETMsk` writer - NYET Interrupt Mask"]
pub type NYETMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPMSK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transfer Completed Interrupt Mask"]
    #[inline(always)]
    pub fn xfer_compl_msk(&self) -> XFER_COMPL_MSK_R {
        XFER_COMPL_MSK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt Mask"]
    #[inline(always)]
    pub fn epdisbld_msk(&self) -> EPDISBLD_MSK_R {
        EPDISBLD_MSK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    pub fn ahberr_msk(&self) -> AHBERR_MSK_R {
        AHBERR_MSK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SETUP Phase Done Mask"]
    #[inline(always)]
    pub fn set_upmsk(&self) -> SET_UPMSK_R {
        SET_UPMSK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OUT Token Received when Endpoint Disabled Mask"]
    #[inline(always)]
    pub fn outtkn_epdis_msk(&self) -> OUTTKN_EPDIS_MSK_R {
        OUTTKN_EPDIS_MSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Back-to-Back SETUP Packets Received Mask"]
    #[inline(always)]
    pub fn back2back_setup(&self) -> BACK2BACK_SETUP_R {
        BACK2BACK_SETUP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - OUT Packet Error Mask"]
    #[inline(always)]
    pub fn out_pkt_err_msk(&self) -> OUT_PKT_ERR_MSK_R {
        OUT_PKT_ERR_MSK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BNA interrupt Mask"]
    #[inline(always)]
    pub fn bna_out_intr_msk(&self) -> BNA_OUT_INTR_MSK_R {
        BNA_OUT_INTR_MSK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Babble Interrupt Mask"]
    #[inline(always)]
    pub fn bble_err_msk(&self) -> BBLE_ERR_MSK_R {
        BBLE_ERR_MSK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NAK Interrupt Mask"]
    #[inline(always)]
    pub fn nakmsk(&self) -> NAKMSK_R {
        NAKMSK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NYET Interrupt Mask"]
    #[inline(always)]
    pub fn nyetmsk(&self) -> NYETMSK_R {
        NYETMSK_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn xfer_compl_msk(&mut self) -> XFER_COMPL_MSK_W<0> {
        XFER_COMPL_MSK_W::new(self)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn epdisbld_msk(&mut self) -> EPDISBLD_MSK_W<1> {
        EPDISBLD_MSK_W::new(self)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    #[must_use]
    pub fn ahberr_msk(&mut self) -> AHBERR_MSK_W<2> {
        AHBERR_MSK_W::new(self)
    }
    #[doc = "Bit 3 - SETUP Phase Done Mask"]
    #[inline(always)]
    #[must_use]
    pub fn set_upmsk(&mut self) -> SET_UPMSK_W<3> {
        SET_UPMSK_W::new(self)
    }
    #[doc = "Bit 4 - OUT Token Received when Endpoint Disabled Mask"]
    #[inline(always)]
    #[must_use]
    pub fn outtkn_epdis_msk(&mut self) -> OUTTKN_EPDIS_MSK_W<4> {
        OUTTKN_EPDIS_MSK_W::new(self)
    }
    #[doc = "Bit 6 - Back-to-Back SETUP Packets Received Mask"]
    #[inline(always)]
    #[must_use]
    pub fn back2back_setup(&mut self) -> BACK2BACK_SETUP_W<6> {
        BACK2BACK_SETUP_W::new(self)
    }
    #[doc = "Bit 8 - OUT Packet Error Mask"]
    #[inline(always)]
    #[must_use]
    pub fn out_pkt_err_msk(&mut self) -> OUT_PKT_ERR_MSK_W<8> {
        OUT_PKT_ERR_MSK_W::new(self)
    }
    #[doc = "Bit 9 - BNA interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn bna_out_intr_msk(&mut self) -> BNA_OUT_INTR_MSK_W<9> {
        BNA_OUT_INTR_MSK_W::new(self)
    }
    #[doc = "Bit 12 - Babble Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn bble_err_msk(&mut self) -> BBLE_ERR_MSK_W<12> {
        BBLE_ERR_MSK_W::new(self)
    }
    #[doc = "Bit 13 - NAK Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn nakmsk(&mut self) -> NAKMSK_W<13> {
        NAKMSK_W::new(self)
    }
    #[doc = "Bit 14 - NYET Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn nyetmsk(&mut self) -> NYETMSK_W<14> {
        NYETMSK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device OUT Endpoint Common Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepmsk](index.html) module"]
pub struct DOEPMSK_SPEC;
impl crate::RegisterSpec for DOEPMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doepmsk::R](R) reader structure"]
impl crate::Readable for DOEPMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doepmsk::W](W) writer structure"]
impl crate::Writable for DOEPMSK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPMSK to value 0"]
impl crate::Resettable for DOEPMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
