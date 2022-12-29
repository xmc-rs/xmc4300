#[doc = "Register `HCFG` reader"]
pub struct R(crate::R<HCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCFG` writer"]
pub struct W(crate::W<HCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCFG_SPEC>;
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
impl From<crate::W<HCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FSLSPclkSel` reader - FS PHY Clock Select"]
pub type FSLSPCLK_SEL_R = crate::FieldReader<u8, FSLSPCLK_SEL_A>;
#[doc = "FS PHY Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSLSPCLK_SEL_A {
    #[doc = "1: PHY clock is running at 48 MHz"]
    VALUE1 = 1,
}
impl From<FSLSPCLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FSLSPCLK_SEL_A) -> Self {
        variant as _
    }
}
impl FSLSPCLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FSLSPCLK_SEL_A> {
        match self.bits {
            1 => Some(FSLSPCLK_SEL_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FSLSPCLK_SEL_A::VALUE1
    }
}
#[doc = "Field `FSLSPclkSel` writer - FS PHY Clock Select"]
pub type FSLSPCLK_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCFG_SPEC, u8, FSLSPCLK_SEL_A, 2, O>;
impl<'a, const O: u8> FSLSPCLK_SEL_W<'a, O> {
    #[doc = "PHY clock is running at 48 MHz"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FSLSPCLK_SEL_A::VALUE1)
    }
}
#[doc = "Field `FSLSSupp` reader - FS-Only Support"]
pub type FSLSSUPP_R = crate::BitReader<FSLSSUPP_A>;
#[doc = "FS-Only Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSLSSUPP_A {
    #[doc = "0: FS-only, connected device can supports also only FS."]
    VALUE1 = 0,
    #[doc = "1: FS-only, even if the connected device can support HS"]
    VALUE2 = 1,
}
impl From<FSLSSUPP_A> for bool {
    #[inline(always)]
    fn from(variant: FSLSSUPP_A) -> Self {
        variant as u8 != 0
    }
}
impl FSLSSUPP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSLSSUPP_A {
        match self.bits {
            false => FSLSSUPP_A::VALUE1,
            true => FSLSSUPP_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FSLSSUPP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FSLSSUPP_A::VALUE2
    }
}
#[doc = "Field `FSLSSupp` writer - FS-Only Support"]
pub type FSLSSUPP_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCFG_SPEC, FSLSSUPP_A, O>;
impl<'a, const O: u8> FSLSSUPP_W<'a, O> {
    #[doc = "FS-only, connected device can supports also only FS."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FSLSSUPP_A::VALUE1)
    }
    #[doc = "FS-only, even if the connected device can support HS"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FSLSSUPP_A::VALUE2)
    }
}
#[doc = "Field `DescDMA` reader - Enable Scatter/gather DMA in Host mode"]
pub type DESC_DMA_R = crate::BitReader<bool>;
#[doc = "Field `DescDMA` writer - Enable Scatter/gather DMA in Host mode"]
pub type DESC_DMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCFG_SPEC, bool, O>;
#[doc = "Field `FrListEn` reader - Frame List Entries"]
pub type FR_LIST_EN_R = crate::FieldReader<u8, FR_LIST_EN_A>;
#[doc = "Frame List Entries\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FR_LIST_EN_A {
    #[doc = "0: 8 Entries"]
    VALUE1 = 0,
    #[doc = "1: 16 Entries"]
    VALUE2 = 1,
    #[doc = "2: 32 Entries"]
    VALUE3 = 2,
    #[doc = "3: 64 Entries"]
    VALUE4 = 3,
}
impl From<FR_LIST_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: FR_LIST_EN_A) -> Self {
        variant as _
    }
}
impl FR_LIST_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FR_LIST_EN_A {
        match self.bits {
            0 => FR_LIST_EN_A::VALUE1,
            1 => FR_LIST_EN_A::VALUE2,
            2 => FR_LIST_EN_A::VALUE3,
            3 => FR_LIST_EN_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FR_LIST_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FR_LIST_EN_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == FR_LIST_EN_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == FR_LIST_EN_A::VALUE4
    }
}
#[doc = "Field `FrListEn` writer - Frame List Entries"]
pub type FR_LIST_EN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, HCFG_SPEC, u8, FR_LIST_EN_A, 2, O>;
impl<'a, const O: u8> FR_LIST_EN_W<'a, O> {
    #[doc = "8 Entries"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FR_LIST_EN_A::VALUE1)
    }
    #[doc = "16 Entries"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FR_LIST_EN_A::VALUE2)
    }
    #[doc = "32 Entries"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(FR_LIST_EN_A::VALUE3)
    }
    #[doc = "64 Entries"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(FR_LIST_EN_A::VALUE4)
    }
}
#[doc = "Field `PerSchedEna` reader - Enable Periodic Scheduling"]
pub type PER_SCHED_ENA_R = crate::BitReader<bool>;
#[doc = "Field `PerSchedEna` writer - Enable Periodic Scheduling"]
pub type PER_SCHED_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - FS PHY Clock Select"]
    #[inline(always)]
    pub fn fslspclk_sel(&self) -> FSLSPCLK_SEL_R {
        FSLSPCLK_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - FS-Only Support"]
    #[inline(always)]
    pub fn fslssupp(&self) -> FSLSSUPP_R {
        FSLSSUPP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Scatter/gather DMA in Host mode"]
    #[inline(always)]
    pub fn desc_dma(&self) -> DESC_DMA_R {
        DESC_DMA_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Frame List Entries"]
    #[inline(always)]
    pub fn fr_list_en(&self) -> FR_LIST_EN_R {
        FR_LIST_EN_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Enable Periodic Scheduling"]
    #[inline(always)]
    pub fn per_sched_ena(&self) -> PER_SCHED_ENA_R {
        PER_SCHED_ENA_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - FS PHY Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn fslspclk_sel(&mut self) -> FSLSPCLK_SEL_W<0> {
        FSLSPCLK_SEL_W::new(self)
    }
    #[doc = "Bit 2 - FS-Only Support"]
    #[inline(always)]
    #[must_use]
    pub fn fslssupp(&mut self) -> FSLSSUPP_W<2> {
        FSLSSUPP_W::new(self)
    }
    #[doc = "Bit 23 - Enable Scatter/gather DMA in Host mode"]
    #[inline(always)]
    #[must_use]
    pub fn desc_dma(&mut self) -> DESC_DMA_W<23> {
        DESC_DMA_W::new(self)
    }
    #[doc = "Bits 24:25 - Frame List Entries"]
    #[inline(always)]
    #[must_use]
    pub fn fr_list_en(&mut self) -> FR_LIST_EN_W<24> {
        FR_LIST_EN_W::new(self)
    }
    #[doc = "Bit 26 - Enable Periodic Scheduling"]
    #[inline(always)]
    #[must_use]
    pub fn per_sched_ena(&mut self) -> PER_SCHED_ENA_W<26> {
        PER_SCHED_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcfg](index.html) module"]
pub struct HCFG_SPEC;
impl crate::RegisterSpec for HCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcfg::R](R) reader structure"]
impl crate::Readable for HCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcfg::W](W) writer structure"]
impl crate::Writable for HCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCFG to value 0x0200"]
impl crate::Resettable for HCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
