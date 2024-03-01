#[doc = "Register `ATIM0` reader"]
pub type R = crate::R<Atim0Spec>;
#[doc = "Register `ATIM0` writer"]
pub type W = crate::W<Atim0Spec>;
#[doc = "Field `ASE` reader - Alarm Seconds Compare Value"]
pub type AseR = crate::FieldReader;
#[doc = "Field `ASE` writer - Alarm Seconds Compare Value"]
pub type AseW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `AMI` reader - Alarm Minutes Compare Value"]
pub type AmiR = crate::FieldReader;
#[doc = "Field `AMI` writer - Alarm Minutes Compare Value"]
pub type AmiW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `AHO` reader - Alarm Hours Compare Value"]
pub type AhoR = crate::FieldReader;
#[doc = "Field `AHO` writer - Alarm Hours Compare Value"]
pub type AhoW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADA` reader - Alarm Days Compare Value"]
pub type AdaR = crate::FieldReader;
#[doc = "Field `ADA` writer - Alarm Days Compare Value"]
pub type AdaW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:5 - Alarm Seconds Compare Value"]
    #[inline(always)]
    pub fn ase(&self) -> AseR {
        AseR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Alarm Minutes Compare Value"]
    #[inline(always)]
    pub fn ami(&self) -> AmiR {
        AmiR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - Alarm Hours Compare Value"]
    #[inline(always)]
    pub fn aho(&self) -> AhoR {
        AhoR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Alarm Days Compare Value"]
    #[inline(always)]
    pub fn ada(&self) -> AdaR {
        AdaR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Alarm Seconds Compare Value"]
    #[inline(always)]
    #[must_use]
    pub fn ase(&mut self) -> AseW<Atim0Spec> {
        AseW::new(self, 0)
    }
    #[doc = "Bits 8:13 - Alarm Minutes Compare Value"]
    #[inline(always)]
    #[must_use]
    pub fn ami(&mut self) -> AmiW<Atim0Spec> {
        AmiW::new(self, 8)
    }
    #[doc = "Bits 16:20 - Alarm Hours Compare Value"]
    #[inline(always)]
    #[must_use]
    pub fn aho(&mut self) -> AhoW<Atim0Spec> {
        AhoW::new(self, 16)
    }
    #[doc = "Bits 24:28 - Alarm Days Compare Value"]
    #[inline(always)]
    #[must_use]
    pub fn ada(&mut self) -> AdaW<Atim0Spec> {
        AdaW::new(self, 24)
    }
}
#[doc = "RTC Alarm Time Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atim0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atim0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Atim0Spec;
impl crate::RegisterSpec for Atim0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atim0::R`](R) reader structure"]
impl crate::Readable for Atim0Spec {}
#[doc = "`write(|w| ..)` method takes [`atim0::W`](W) writer structure"]
impl crate::Writable for Atim0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATIM0 to value 0"]
impl crate::Resettable for Atim0Spec {
    const RESET_VALUE: u32 = 0;
}
