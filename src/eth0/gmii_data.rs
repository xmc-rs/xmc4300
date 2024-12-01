#[doc = "Register `GMII_DATA` reader"]
pub type R = crate::R<GMII_DATA_SPEC>;
#[doc = "Register `GMII_DATA` writer"]
pub type W = crate::W<GMII_DATA_SPEC>;
#[doc = "Field `MD` reader - MII Data"]
pub type MD_R = crate::FieldReader<u16>;
#[doc = "Field `MD` writer - MII Data"]
pub type MD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - MII Data"]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MII Data"]
    #[inline(always)]
    pub fn md(&mut self) -> MD_W<GMII_DATA_SPEC> {
        MD_W::new(self, 0)
    }
}
#[doc = "MII Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gmii_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmii_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GMII_DATA_SPEC;
impl crate::RegisterSpec for GMII_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmii_data::R`](R) reader structure"]
impl crate::Readable for GMII_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gmii_data::W`](W) writer structure"]
impl crate::Writable for GMII_DATA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GMII_DATA to value 0"]
impl crate::Resettable for GMII_DATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
