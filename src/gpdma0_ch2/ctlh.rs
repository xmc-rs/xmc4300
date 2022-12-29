#[doc = "Register `CTLH` reader"]
pub struct R(crate::R<CTLH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTLH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTLH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTLH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTLH` writer"]
pub struct W(crate::W<CTLH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTLH_SPEC>;
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
impl From<crate::W<CTLH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTLH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLOCK_TS` reader - Block Transfer Size"]
pub type BLOCK_TS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BLOCK_TS` writer - Block Transfer Size"]
pub type BLOCK_TS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTLH_SPEC, u16, u16, 12, O>;
#[doc = "Field `DONE` reader - Done bit"]
pub type DONE_R = crate::BitReader<bool>;
#[doc = "Field `DONE` writer - Done bit"]
pub type DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTLH_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:11 - Block Transfer Size"]
    #[inline(always)]
    pub fn block_ts(&self) -> BLOCK_TS_R {
        BLOCK_TS_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - Done bit"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Block Transfer Size"]
    #[inline(always)]
    #[must_use]
    pub fn block_ts(&mut self) -> BLOCK_TS_W<0> {
        BLOCK_TS_W::new(self)
    }
    #[doc = "Bit 12 - Done bit"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DONE_W<12> {
        DONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register High\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctlh](index.html) module"]
pub struct CTLH_SPEC;
impl crate::RegisterSpec for CTLH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctlh::R](R) reader structure"]
impl crate::Readable for CTLH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctlh::W](W) writer structure"]
impl crate::Writable for CTLH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTLH to value 0x02"]
impl crate::Resettable for CTLH_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
