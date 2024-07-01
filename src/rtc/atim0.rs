#[doc = "Register `ATIM0` reader"]
pub type R = crate::R<ATIM0_SPEC>;
#[doc = "Register `ATIM0` writer"]
pub type W = crate::W<ATIM0_SPEC>;
#[doc = "Field `ASE` reader - Alarm Seconds Compare Value"]
pub type ASE_R = crate::FieldReader;
#[doc = "Field `ASE` writer - Alarm Seconds Compare Value"]
pub type ASE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `AMI` reader - Alarm Minutes Compare Value"]
pub type AMI_R = crate::FieldReader;
#[doc = "Field `AMI` writer - Alarm Minutes Compare Value"]
pub type AMI_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `AHO` reader - Alarm Hours Compare Value"]
pub type AHO_R = crate::FieldReader;
#[doc = "Field `AHO` writer - Alarm Hours Compare Value"]
pub type AHO_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADA` reader - Alarm Days Compare Value"]
pub type ADA_R = crate::FieldReader;
#[doc = "Field `ADA` writer - Alarm Days Compare Value"]
pub type ADA_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:5 - Alarm Seconds Compare Value"]
    #[inline(always)]
    pub fn ase(&self) -> ASE_R {
        ASE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Alarm Minutes Compare Value"]
    #[inline(always)]
    pub fn ami(&self) -> AMI_R {
        AMI_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - Alarm Hours Compare Value"]
    #[inline(always)]
    pub fn aho(&self) -> AHO_R {
        AHO_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Alarm Days Compare Value"]
    #[inline(always)]
    pub fn ada(&self) -> ADA_R {
        ADA_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Alarm Seconds Compare Value"]
    #[inline(always)]
    #[must_use]
    pub fn ase(&mut self) -> ASE_W<ATIM0_SPEC> {
        ASE_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - Alarm Minutes Compare Value"]
    #[inline(always)]
    #[must_use]
    pub fn ami(&mut self) -> AMI_W<ATIM0_SPEC> {
        AMI_W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Alarm Hours Compare Value"]
    #[inline(always)]
    #[must_use]
    pub fn aho(&mut self) -> AHO_W<ATIM0_SPEC> {
        AHO_W::new(self, 16)
    }
    #[doc = "Bits 24:28 - Alarm Days Compare Value"]
    #[inline(always)]
    #[must_use]
    pub fn ada(&mut self) -> ADA_W<ATIM0_SPEC> {
        ADA_W::new(self, 24)
    }
}
#[doc = "RTC Alarm Time Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`atim0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atim0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ATIM0_SPEC;
impl crate::RegisterSpec for ATIM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atim0::R`](R) reader structure"]
impl crate::Readable for ATIM0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`atim0::W`](W) writer structure"]
impl crate::Writable for ATIM0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATIM0 to value 0"]
impl crate::Resettable for ATIM0_SPEC {
    const RESET_VALUE: u32 = 0;
}
