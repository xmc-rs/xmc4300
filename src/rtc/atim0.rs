#[doc = "Register `ATIM0` reader"]
pub struct R(crate::R<ATIM0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ATIM0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ATIM0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ATIM0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ATIM0` writer"]
pub struct W(crate::W<ATIM0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ATIM0_SPEC>;
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
impl From<crate::W<ATIM0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ATIM0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ASE` reader - Alarm Seconds Compare Value"]
pub type ASE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ASE` writer - Alarm Seconds Compare Value"]
pub type ASE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATIM0_SPEC, u8, u8, 6, O>;
#[doc = "Field `AMI` reader - Alarm Minutes Compare Value"]
pub type AMI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AMI` writer - Alarm Minutes Compare Value"]
pub type AMI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATIM0_SPEC, u8, u8, 6, O>;
#[doc = "Field `AHO` reader - Alarm Hours Compare Value"]
pub type AHO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AHO` writer - Alarm Hours Compare Value"]
pub type AHO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATIM0_SPEC, u8, u8, 5, O>;
#[doc = "Field `ADA` reader - Alarm Days Compare Value"]
pub type ADA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADA` writer - Alarm Days Compare Value"]
pub type ADA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATIM0_SPEC, u8, u8, 5, O>;
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
    pub fn ase(&mut self) -> ASE_W<0> {
        ASE_W::new(self)
    }
    #[doc = "Bits 8:13 - Alarm Minutes Compare Value"]
    #[inline(always)]
    #[must_use]
    pub fn ami(&mut self) -> AMI_W<8> {
        AMI_W::new(self)
    }
    #[doc = "Bits 16:20 - Alarm Hours Compare Value"]
    #[inline(always)]
    #[must_use]
    pub fn aho(&mut self) -> AHO_W<16> {
        AHO_W::new(self)
    }
    #[doc = "Bits 24:28 - Alarm Days Compare Value"]
    #[inline(always)]
    #[must_use]
    pub fn ada(&mut self) -> ADA_W<24> {
        ADA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Alarm Time Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atim0](index.html) module"]
pub struct ATIM0_SPEC;
impl crate::RegisterSpec for ATIM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [atim0::R](R) reader structure"]
impl crate::Readable for ATIM0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [atim0::W](W) writer structure"]
impl crate::Writable for ATIM0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ATIM0 to value 0"]
impl crate::Resettable for ATIM0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
