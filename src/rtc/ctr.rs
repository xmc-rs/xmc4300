#[doc = "Register `CTR` reader"]
pub struct R(crate::R<CTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTR` writer"]
pub struct W(crate::W<CTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTR_SPEC>;
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
impl From<crate::W<CTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENB` reader - RTC Module Enable"]
pub type ENB_R = crate::BitReader<bool>;
#[doc = "Field `ENB` writer - RTC Module Enable"]
pub type ENB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTR_SPEC, bool, O>;
#[doc = "Field `TAE` reader - Timer Alarm Enable for Hibernation Wake-up"]
pub type TAE_R = crate::BitReader<bool>;
#[doc = "Field `TAE` writer - Timer Alarm Enable for Hibernation Wake-up"]
pub type TAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTR_SPEC, bool, O>;
#[doc = "Field `ESEC` reader - Enable Seconds Comparison for Hibernation Wake-up"]
pub type ESEC_R = crate::BitReader<bool>;
#[doc = "Field `ESEC` writer - Enable Seconds Comparison for Hibernation Wake-up"]
pub type ESEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTR_SPEC, bool, O>;
#[doc = "Field `EMIC` reader - Enable Minutes Comparison for Hibernation Wake-up"]
pub type EMIC_R = crate::BitReader<bool>;
#[doc = "Field `EMIC` writer - Enable Minutes Comparison for Hibernation Wake-up"]
pub type EMIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTR_SPEC, bool, O>;
#[doc = "Field `EHOC` reader - Enable Hours Comparison for Hibernation Wake-up"]
pub type EHOC_R = crate::BitReader<bool>;
#[doc = "Field `EHOC` writer - Enable Hours Comparison for Hibernation Wake-up"]
pub type EHOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTR_SPEC, bool, O>;
#[doc = "Field `EDAC` reader - Enable Days Comparison for Hibernation Wake-up"]
pub type EDAC_R = crate::BitReader<bool>;
#[doc = "Field `EDAC` writer - Enable Days Comparison for Hibernation Wake-up"]
pub type EDAC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTR_SPEC, bool, O>;
#[doc = "Field `EMOC` reader - Enable Months Comparison for Hibernation Wake-up"]
pub type EMOC_R = crate::BitReader<bool>;
#[doc = "Field `EMOC` writer - Enable Months Comparison for Hibernation Wake-up"]
pub type EMOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTR_SPEC, bool, O>;
#[doc = "Field `EYEC` reader - Enable Years Comparison for Hibernation Wake-up"]
pub type EYEC_R = crate::BitReader<bool>;
#[doc = "Field `EYEC` writer - Enable Years Comparison for Hibernation Wake-up"]
pub type EYEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTR_SPEC, bool, O>;
#[doc = "Field `DIV` reader - RTC Clock Divider Value"]
pub type DIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DIV` writer - RTC Clock Divider Value"]
pub type DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - RTC Module Enable"]
    #[inline(always)]
    pub fn enb(&self) -> ENB_R {
        ENB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Timer Alarm Enable for Hibernation Wake-up"]
    #[inline(always)]
    pub fn tae(&self) -> TAE_R {
        TAE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Seconds Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn esec(&self) -> ESEC_R {
        ESEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Minutes Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn emic(&self) -> EMIC_R {
        EMIC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Hours Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn ehoc(&self) -> EHOC_R {
        EHOC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Days Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn edac(&self) -> EDAC_R {
        EDAC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Months Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn emoc(&self) -> EMOC_R {
        EMOC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Years Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn eyec(&self) -> EYEC_R {
        EYEC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:31 - RTC Clock Divider Value"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Module Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enb(&mut self) -> ENB_W<0> {
        ENB_W::new(self)
    }
    #[doc = "Bit 2 - Timer Alarm Enable for Hibernation Wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn tae(&mut self) -> TAE_W<2> {
        TAE_W::new(self)
    }
    #[doc = "Bit 8 - Enable Seconds Comparison for Hibernation Wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn esec(&mut self) -> ESEC_W<8> {
        ESEC_W::new(self)
    }
    #[doc = "Bit 9 - Enable Minutes Comparison for Hibernation Wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn emic(&mut self) -> EMIC_W<9> {
        EMIC_W::new(self)
    }
    #[doc = "Bit 10 - Enable Hours Comparison for Hibernation Wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn ehoc(&mut self) -> EHOC_W<10> {
        EHOC_W::new(self)
    }
    #[doc = "Bit 11 - Enable Days Comparison for Hibernation Wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn edac(&mut self) -> EDAC_W<11> {
        EDAC_W::new(self)
    }
    #[doc = "Bit 13 - Enable Months Comparison for Hibernation Wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn emoc(&mut self) -> EMOC_W<13> {
        EMOC_W::new(self)
    }
    #[doc = "Bit 14 - Enable Years Comparison for Hibernation Wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn eyec(&mut self) -> EYEC_W<14> {
        EYEC_W::new(self)
    }
    #[doc = "Bits 16:31 - RTC Clock Divider Value"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<16> {
        DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctr](index.html) module"]
pub struct CTR_SPEC;
impl crate::RegisterSpec for CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctr::R](R) reader structure"]
impl crate::Readable for CTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctr::W](W) writer structure"]
impl crate::Writable for CTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTR to value 0x7fff_0000"]
impl crate::Resettable for CTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x7fff_0000;
}
