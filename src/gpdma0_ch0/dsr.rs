#[doc = "Register `DSR` reader"]
pub type R = crate::R<DsrSpec>;
#[doc = "Register `DSR` writer"]
pub type W = crate::W<DsrSpec>;
#[doc = "Field `DSI` reader - Destination scatter interval"]
pub type DsiR = crate::FieldReader<u32>;
#[doc = "Field `DSI` writer - Destination scatter interval"]
pub type DsiW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `DSC` reader - Destination scatter count"]
pub type DscR = crate::FieldReader<u16>;
#[doc = "Field `DSC` writer - Destination scatter count"]
pub type DscW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:19 - Destination scatter interval"]
    #[inline(always)]
    pub fn dsi(&self) -> DsiR {
        DsiR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31 - Destination scatter count"]
    #[inline(always)]
    pub fn dsc(&self) -> DscR {
        DscR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:19 - Destination scatter interval"]
    #[inline(always)]
    #[must_use]
    pub fn dsi(&mut self) -> DsiW<DsrSpec> {
        DsiW::new(self, 0)
    }
    #[doc = "Bits 20:31 - Destination scatter count"]
    #[inline(always)]
    #[must_use]
    pub fn dsc(&mut self) -> DscW<DsrSpec> {
        DscW::new(self, 20)
    }
}
#[doc = "Destination Scatter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsrSpec;
impl crate::RegisterSpec for DsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsr::R`](R) reader structure"]
impl crate::Readable for DsrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsr::W`](W) writer structure"]
impl crate::Writable for DsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSR to value 0"]
impl crate::Resettable for DsrSpec {
    const RESET_VALUE: u32 = 0;
}
