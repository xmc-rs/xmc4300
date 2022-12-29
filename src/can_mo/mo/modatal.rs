#[doc = "Register `MODATAL` reader"]
pub struct R(crate::R<MODATAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODATAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODATAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODATAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODATAL` writer"]
pub struct W(crate::W<MODATAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODATAL_SPEC>;
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
impl From<crate::W<MODATAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODATAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DB0` reader - Data Byte 0 of Message Object n"]
pub type DB0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DB0` writer - Data Byte 0 of Message Object n"]
pub type DB0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODATAL_SPEC, u8, u8, 8, O>;
#[doc = "Field `DB1` reader - Data Byte 1 of Message Object n"]
pub type DB1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DB1` writer - Data Byte 1 of Message Object n"]
pub type DB1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODATAL_SPEC, u8, u8, 8, O>;
#[doc = "Field `DB2` reader - Data Byte 2 of Message Object n"]
pub type DB2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DB2` writer - Data Byte 2 of Message Object n"]
pub type DB2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODATAL_SPEC, u8, u8, 8, O>;
#[doc = "Field `DB3` reader - Data Byte 3 of Message Object n"]
pub type DB3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DB3` writer - Data Byte 3 of Message Object n"]
pub type DB3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODATAL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Data Byte 0 of Message Object n"]
    #[inline(always)]
    pub fn db0(&self) -> DB0_R {
        DB0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data Byte 1 of Message Object n"]
    #[inline(always)]
    pub fn db1(&self) -> DB1_R {
        DB1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data Byte 2 of Message Object n"]
    #[inline(always)]
    pub fn db2(&self) -> DB2_R {
        DB2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data Byte 3 of Message Object n"]
    #[inline(always)]
    pub fn db3(&self) -> DB3_R {
        DB3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Byte 0 of Message Object n"]
    #[inline(always)]
    #[must_use]
    pub fn db0(&mut self) -> DB0_W<0> {
        DB0_W::new(self)
    }
    #[doc = "Bits 8:15 - Data Byte 1 of Message Object n"]
    #[inline(always)]
    #[must_use]
    pub fn db1(&mut self) -> DB1_W<8> {
        DB1_W::new(self)
    }
    #[doc = "Bits 16:23 - Data Byte 2 of Message Object n"]
    #[inline(always)]
    #[must_use]
    pub fn db2(&mut self) -> DB2_W<16> {
        DB2_W::new(self)
    }
    #[doc = "Bits 24:31 - Data Byte 3 of Message Object n"]
    #[inline(always)]
    #[must_use]
    pub fn db3(&mut self) -> DB3_W<24> {
        DB3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Message Object Data Register Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modatal](index.html) module"]
pub struct MODATAL_SPEC;
impl crate::RegisterSpec for MODATAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [modatal::R](R) reader structure"]
impl crate::Readable for MODATAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [modatal::W](W) writer structure"]
impl crate::Writable for MODATAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MODATAL to value 0"]
impl crate::Resettable for MODATAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
