#[doc = "Register `GMII_DATA` reader"]
pub type R = crate::R<GmiiDataSpec>;
#[doc = "Register `GMII_DATA` writer"]
pub type W = crate::W<GmiiDataSpec>;
#[doc = "Field `MD` reader - MII Data"]
pub type MdR = crate::FieldReader<u16>;
#[doc = "Field `MD` writer - MII Data"]
pub type MdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - MII Data"]
    #[inline(always)]
    pub fn md(&self) -> MdR {
        MdR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MII Data"]
    #[inline(always)]
    #[must_use]
    pub fn md(&mut self) -> MdW<GmiiDataSpec> {
        MdW::new(self, 0)
    }
}
#[doc = "MII Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmii_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmii_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmiiDataSpec;
impl crate::RegisterSpec for GmiiDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmii_data::R`](R) reader structure"]
impl crate::Readable for GmiiDataSpec {}
#[doc = "`write(|w| ..)` method takes [`gmii_data::W`](W) writer structure"]
impl crate::Writable for GmiiDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GMII_DATA to value 0"]
impl crate::Resettable for GmiiDataSpec {
    const RESET_VALUE: u32 = 0;
}
