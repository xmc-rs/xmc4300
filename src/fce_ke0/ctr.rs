#[doc = "Register `CTR` reader"]
pub type R = crate::R<CtrSpec>;
#[doc = "Register `CTR` writer"]
pub type W = crate::W<CtrSpec>;
#[doc = "Field `FCM` reader - Force CRC Mismatch"]
pub type FcmR = crate::BitReader;
#[doc = "Field `FCM` writer - Force CRC Mismatch"]
pub type FcmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRM_CFG` reader - Force CFG Register Mismatch"]
pub type FrmCfgR = crate::BitReader;
#[doc = "Field `FRM_CFG` writer - Force CFG Register Mismatch"]
pub type FrmCfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRM_CHECK` reader - Force Check Register Mismatch"]
pub type FrmCheckR = crate::BitReader;
#[doc = "Field `FRM_CHECK` writer - Force Check Register Mismatch"]
pub type FrmCheckW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Force CRC Mismatch"]
    #[inline(always)]
    pub fn fcm(&self) -> FcmR {
        FcmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force CFG Register Mismatch"]
    #[inline(always)]
    pub fn frm_cfg(&self) -> FrmCfgR {
        FrmCfgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Force Check Register Mismatch"]
    #[inline(always)]
    pub fn frm_check(&self) -> FrmCheckR {
        FrmCheckR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force CRC Mismatch"]
    #[inline(always)]
    #[must_use]
    pub fn fcm(&mut self) -> FcmW<CtrSpec> {
        FcmW::new(self, 0)
    }
    #[doc = "Bit 1 - Force CFG Register Mismatch"]
    #[inline(always)]
    #[must_use]
    pub fn frm_cfg(&mut self) -> FrmCfgW<CtrSpec> {
        FrmCfgW::new(self, 1)
    }
    #[doc = "Bit 2 - Force Check Register Mismatch"]
    #[inline(always)]
    #[must_use]
    pub fn frm_check(&mut self) -> FrmCheckW<CtrSpec> {
        FrmCheckW::new(self, 2)
    }
}
#[doc = "CRC Test Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrSpec;
impl crate::RegisterSpec for CtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctr::R`](R) reader structure"]
impl crate::Readable for CtrSpec {}
#[doc = "`write(|w| ..)` method takes [`ctr::W`](W) writer structure"]
impl crate::Writable for CtrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTR to value 0"]
impl crate::Resettable for CtrSpec {
    const RESET_VALUE: u32 = 0;
}
