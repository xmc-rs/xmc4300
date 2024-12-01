#[doc = "Register `DOEPMSK` reader"]
pub type R = crate::R<DOEPMSK_SPEC>;
#[doc = "Register `DOEPMSK` writer"]
pub type W = crate::W<DOEPMSK_SPEC>;
#[doc = "Field `XferComplMsk` reader - Transfer Completed Interrupt Mask"]
pub type XFER_COMPL_MSK_R = crate::BitReader;
#[doc = "Field `XferComplMsk` writer - Transfer Completed Interrupt Mask"]
pub type XFER_COMPL_MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDisbldMsk` reader - Endpoint Disabled Interrupt Mask"]
pub type EPDISBLD_MSK_R = crate::BitReader;
#[doc = "Field `EPDisbldMsk` writer - Endpoint Disabled Interrupt Mask"]
pub type EPDISBLD_MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBErrMsk` reader - AHB Error"]
pub type AHBERR_MSK_R = crate::BitReader;
#[doc = "Field `AHBErrMsk` writer - AHB Error"]
pub type AHBERR_MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SetUPMsk` reader - SETUP Phase Done Mask"]
pub type SET_UPMSK_R = crate::BitReader;
#[doc = "Field `SetUPMsk` writer - SETUP Phase Done Mask"]
pub type SET_UPMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTTknEPdisMsk` reader - OUT Token Received when Endpoint Disabled Mask"]
pub type OUTTKN_EPDIS_MSK_R = crate::BitReader;
#[doc = "Field `OUTTknEPdisMsk` writer - OUT Token Received when Endpoint Disabled Mask"]
pub type OUTTKN_EPDIS_MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Back2BackSETup` reader - Back-to-Back SETUP Packets Received Mask"]
pub type BACK2BACK_SETUP_R = crate::BitReader;
#[doc = "Field `Back2BackSETup` writer - Back-to-Back SETUP Packets Received Mask"]
pub type BACK2BACK_SETUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OutPktErrMsk` reader - OUT Packet Error Mask"]
pub type OUT_PKT_ERR_MSK_R = crate::BitReader;
#[doc = "Field `OutPktErrMsk` writer - OUT Packet Error Mask"]
pub type OUT_PKT_ERR_MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BnaOutIntrMsk` reader - BNA interrupt Mask"]
pub type BNA_OUT_INTR_MSK_R = crate::BitReader;
#[doc = "Field `BnaOutIntrMsk` writer - BNA interrupt Mask"]
pub type BNA_OUT_INTR_MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BbleErrMsk` reader - Babble Interrupt Mask"]
pub type BBLE_ERR_MSK_R = crate::BitReader;
#[doc = "Field `BbleErrMsk` writer - Babble Interrupt Mask"]
pub type BBLE_ERR_MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKMsk` reader - NAK Interrupt Mask"]
pub type NAKMSK_R = crate::BitReader;
#[doc = "Field `NAKMsk` writer - NAK Interrupt Mask"]
pub type NAKMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYETMsk` reader - NYET Interrupt Mask"]
pub type NYETMSK_R = crate::BitReader;
#[doc = "Field `NYETMsk` writer - NYET Interrupt Mask"]
pub type NYETMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn xfer_compl_msk(&mut self) -> XFER_COMPL_MSK_W<DOEPMSK_SPEC> {
        XFER_COMPL_MSK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt Mask"]
    #[inline(always)]
    pub fn epdisbld_msk(&mut self) -> EPDISBLD_MSK_W<DOEPMSK_SPEC> {
        EPDISBLD_MSK_W::new(self, 1)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    pub fn ahberr_msk(&mut self) -> AHBERR_MSK_W<DOEPMSK_SPEC> {
        AHBERR_MSK_W::new(self, 2)
    }
    #[doc = "Bit 3 - SETUP Phase Done Mask"]
    #[inline(always)]
    pub fn set_upmsk(&mut self) -> SET_UPMSK_W<DOEPMSK_SPEC> {
        SET_UPMSK_W::new(self, 3)
    }
    #[doc = "Bit 4 - OUT Token Received when Endpoint Disabled Mask"]
    #[inline(always)]
    pub fn outtkn_epdis_msk(&mut self) -> OUTTKN_EPDIS_MSK_W<DOEPMSK_SPEC> {
        OUTTKN_EPDIS_MSK_W::new(self, 4)
    }
    #[doc = "Bit 6 - Back-to-Back SETUP Packets Received Mask"]
    #[inline(always)]
    pub fn back2back_setup(&mut self) -> BACK2BACK_SETUP_W<DOEPMSK_SPEC> {
        BACK2BACK_SETUP_W::new(self, 6)
    }
    #[doc = "Bit 8 - OUT Packet Error Mask"]
    #[inline(always)]
    pub fn out_pkt_err_msk(&mut self) -> OUT_PKT_ERR_MSK_W<DOEPMSK_SPEC> {
        OUT_PKT_ERR_MSK_W::new(self, 8)
    }
    #[doc = "Bit 9 - BNA interrupt Mask"]
    #[inline(always)]
    pub fn bna_out_intr_msk(&mut self) -> BNA_OUT_INTR_MSK_W<DOEPMSK_SPEC> {
        BNA_OUT_INTR_MSK_W::new(self, 9)
    }
    #[doc = "Bit 12 - Babble Interrupt Mask"]
    #[inline(always)]
    pub fn bble_err_msk(&mut self) -> BBLE_ERR_MSK_W<DOEPMSK_SPEC> {
        BBLE_ERR_MSK_W::new(self, 12)
    }
    #[doc = "Bit 13 - NAK Interrupt Mask"]
    #[inline(always)]
    pub fn nakmsk(&mut self) -> NAKMSK_W<DOEPMSK_SPEC> {
        NAKMSK_W::new(self, 13)
    }
    #[doc = "Bit 14 - NYET Interrupt Mask"]
    #[inline(always)]
    pub fn nyetmsk(&mut self) -> NYETMSK_W<DOEPMSK_SPEC> {
        NYETMSK_W::new(self, 14)
    }
}
#[doc = "Device OUT Endpoint Common Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doepmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPMSK_SPEC;
impl crate::RegisterSpec for DOEPMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepmsk::R`](R) reader structure"]
impl crate::Readable for DOEPMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepmsk::W`](W) writer structure"]
impl crate::Writable for DOEPMSK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPMSK to value 0"]
impl crate::Resettable for DOEPMSK_SPEC {
    const RESET_VALUE: u32 = 0;
}
