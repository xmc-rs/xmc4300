#[doc = "Register `MPU_RASR_A2` reader"]
pub struct R(crate::R<MPU_RASR_A2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPU_RASR_A2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPU_RASR_A2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPU_RASR_A2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPU_RASR_A2` writer"]
pub struct W(crate::W<MPU_RASR_A2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPU_RASR_A2_SPEC>;
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
impl From<crate::W<MPU_RASR_A2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPU_RASR_A2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Region enable bit."]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Region enable bit."]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPU_RASR_A2_SPEC, bool, O>;
#[doc = "Field `SIZE` reader - MPU protection region size"]
pub type SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SIZE` writer - MPU protection region size"]
pub type SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPU_RASR_A2_SPEC, u8, u8, 5, O>;
#[doc = "Field `SRD` reader - Subregion disable bits"]
pub type SRD_R = crate::FieldReader<u8, SRD_A>;
#[doc = "Subregion disable bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SRD_A {
    #[doc = "0: corresponding sub-region is enabled"]
    VALUE1 = 0,
    #[doc = "1: corresponding sub-region is disabled"]
    VALUE2 = 1,
}
impl From<SRD_A> for u8 {
    #[inline(always)]
    fn from(variant: SRD_A) -> Self {
        variant as _
    }
}
impl SRD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SRD_A> {
        match self.bits {
            0 => Some(SRD_A::VALUE1),
            1 => Some(SRD_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SRD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SRD_A::VALUE2
    }
}
#[doc = "Field `SRD` writer - Subregion disable bits"]
pub type SRD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPU_RASR_A2_SPEC, u8, SRD_A, 8, O>;
impl<'a, const O: u8> SRD_W<'a, O> {
    #[doc = "corresponding sub-region is enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRD_A::VALUE1)
    }
    #[doc = "corresponding sub-region is disabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRD_A::VALUE2)
    }
}
#[doc = "Field `B` reader - Memory access attribute"]
pub type B_R = crate::BitReader<bool>;
#[doc = "Field `B` writer - Memory access attribute"]
pub type B_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPU_RASR_A2_SPEC, bool, O>;
#[doc = "Field `C` reader - Memory access attribute"]
pub type C_R = crate::BitReader<bool>;
#[doc = "Field `C` writer - Memory access attribute"]
pub type C_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPU_RASR_A2_SPEC, bool, O>;
#[doc = "Field `S` reader - Shareable bit"]
pub type S_R = crate::BitReader<bool>;
#[doc = "Field `S` writer - Shareable bit"]
pub type S_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPU_RASR_A2_SPEC, bool, O>;
#[doc = "Field `TEX` reader - Memory access attribute"]
pub type TEX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TEX` writer - Memory access attribute"]
pub type TEX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPU_RASR_A2_SPEC, u8, u8, 3, O>;
#[doc = "Field `AP` reader - Access permission field"]
pub type AP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AP` writer - Access permission field"]
pub type AP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPU_RASR_A2_SPEC, u8, u8, 3, O>;
#[doc = "Field `XN` reader - Instruction access disable bit"]
pub type XN_R = crate::BitReader<XN_A>;
#[doc = "Instruction access disable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XN_A {
    #[doc = "0: instruction fetches enabled"]
    VALUE1 = 0,
    #[doc = "1: instruction fetches disabled."]
    VALUE2 = 1,
}
impl From<XN_A> for bool {
    #[inline(always)]
    fn from(variant: XN_A) -> Self {
        variant as u8 != 0
    }
}
impl XN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XN_A {
        match self.bits {
            false => XN_A::VALUE1,
            true => XN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == XN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == XN_A::VALUE2
    }
}
#[doc = "Field `XN` writer - Instruction access disable bit"]
pub type XN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPU_RASR_A2_SPEC, XN_A, O>;
impl<'a, const O: u8> XN_W<'a, O> {
    #[doc = "instruction fetches enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(XN_A::VALUE1)
    }
    #[doc = "instruction fetches disabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(XN_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Region enable bit."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - MPU protection region size"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - Subregion disable bits"]
    #[inline(always)]
    pub fn srd(&self) -> SRD_R {
        SRD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Memory access attribute"]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Memory access attribute"]
    #[inline(always)]
    pub fn c(&self) -> C_R {
        C_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Shareable bit"]
    #[inline(always)]
    pub fn s(&self) -> S_R {
        S_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:21 - Memory access attribute"]
    #[inline(always)]
    pub fn tex(&self) -> TEX_R {
        TEX_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Access permission field"]
    #[inline(always)]
    pub fn ap(&self) -> AP_R {
        AP_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 28 - Instruction access disable bit"]
    #[inline(always)]
    pub fn xn(&self) -> XN_R {
        XN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Region enable bit."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 1:5 - MPU protection region size"]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SIZE_W<1> {
        SIZE_W::new(self)
    }
    #[doc = "Bits 8:15 - Subregion disable bits"]
    #[inline(always)]
    #[must_use]
    pub fn srd(&mut self) -> SRD_W<8> {
        SRD_W::new(self)
    }
    #[doc = "Bit 16 - Memory access attribute"]
    #[inline(always)]
    #[must_use]
    pub fn b(&mut self) -> B_W<16> {
        B_W::new(self)
    }
    #[doc = "Bit 17 - Memory access attribute"]
    #[inline(always)]
    #[must_use]
    pub fn c(&mut self) -> C_W<17> {
        C_W::new(self)
    }
    #[doc = "Bit 18 - Shareable bit"]
    #[inline(always)]
    #[must_use]
    pub fn s(&mut self) -> S_W<18> {
        S_W::new(self)
    }
    #[doc = "Bits 19:21 - Memory access attribute"]
    #[inline(always)]
    #[must_use]
    pub fn tex(&mut self) -> TEX_W<19> {
        TEX_W::new(self)
    }
    #[doc = "Bits 24:26 - Access permission field"]
    #[inline(always)]
    #[must_use]
    pub fn ap(&mut self) -> AP_W<24> {
        AP_W::new(self)
    }
    #[doc = "Bit 28 - Instruction access disable bit"]
    #[inline(always)]
    #[must_use]
    pub fn xn(&mut self) -> XN_W<28> {
        XN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPU Region Attribute and Size Register A2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpu_rasr_a2](index.html) module"]
pub struct MPU_RASR_A2_SPEC;
impl crate::RegisterSpec for MPU_RASR_A2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpu_rasr_a2::R](R) reader structure"]
impl crate::Readable for MPU_RASR_A2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpu_rasr_a2::W](W) writer structure"]
impl crate::Writable for MPU_RASR_A2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MPU_RASR_A2 to value 0"]
impl crate::Resettable for MPU_RASR_A2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
