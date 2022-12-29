#[doc = "Register `CTLL` reader"]
pub struct R(crate::R<CTLL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTLL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTLL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTLL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTLL` writer"]
pub struct W(crate::W<CTLL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTLL_SPEC>;
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
impl From<crate::W<CTLL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTLL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT_EN` reader - Interrupt Enable Bit"]
pub type INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `INT_EN` writer - Interrupt Enable Bit"]
pub type INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTLL_SPEC, bool, O>;
#[doc = "Field `DST_TR_WIDTH` reader - Destination Transfer Width"]
pub type DST_TR_WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DST_TR_WIDTH` writer - Destination Transfer Width"]
pub type DST_TR_WIDTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTLL_SPEC, u8, u8, 3, O>;
#[doc = "Field `SRC_TR_WIDTH` reader - Source Transfer Width"]
pub type SRC_TR_WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SRC_TR_WIDTH` writer - Source Transfer Width"]
pub type SRC_TR_WIDTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTLL_SPEC, u8, u8, 3, O>;
#[doc = "Field `DINC` reader - Destination Address Increment"]
pub type DINC_R = crate::FieldReader<u8, DINC_A>;
#[doc = "Destination Address Increment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DINC_A {
    #[doc = "0: Increment"]
    VALUE1 = 0,
    #[doc = "1: Decrement"]
    VALUE2 = 1,
    #[doc = "2: No change"]
    VALUE3 = 2,
}
impl From<DINC_A> for u8 {
    #[inline(always)]
    fn from(variant: DINC_A) -> Self {
        variant as _
    }
}
impl DINC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DINC_A> {
        match self.bits {
            0 => Some(DINC_A::VALUE1),
            1 => Some(DINC_A::VALUE2),
            2 => Some(DINC_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DINC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DINC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == DINC_A::VALUE3
    }
}
#[doc = "Field `DINC` writer - Destination Address Increment"]
pub type DINC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTLL_SPEC, u8, DINC_A, 2, O>;
impl<'a, const O: u8> DINC_W<'a, O> {
    #[doc = "Increment"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DINC_A::VALUE1)
    }
    #[doc = "Decrement"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DINC_A::VALUE2)
    }
    #[doc = "No change"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(DINC_A::VALUE3)
    }
}
#[doc = "Field `SINC` reader - Source Address Increment"]
pub type SINC_R = crate::FieldReader<u8, SINC_A>;
#[doc = "Source Address Increment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SINC_A {
    #[doc = "0: Increment"]
    VALUE1 = 0,
    #[doc = "1: Decrement"]
    VALUE2 = 1,
    #[doc = "2: No change"]
    VALUE3 = 2,
}
impl From<SINC_A> for u8 {
    #[inline(always)]
    fn from(variant: SINC_A) -> Self {
        variant as _
    }
}
impl SINC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SINC_A> {
        match self.bits {
            0 => Some(SINC_A::VALUE1),
            1 => Some(SINC_A::VALUE2),
            2 => Some(SINC_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SINC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SINC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SINC_A::VALUE3
    }
}
#[doc = "Field `SINC` writer - Source Address Increment"]
pub type SINC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTLL_SPEC, u8, SINC_A, 2, O>;
impl<'a, const O: u8> SINC_W<'a, O> {
    #[doc = "Increment"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SINC_A::VALUE1)
    }
    #[doc = "Decrement"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SINC_A::VALUE2)
    }
    #[doc = "No change"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SINC_A::VALUE3)
    }
}
#[doc = "Field `DEST_MSIZE` reader - Destination Burst Transaction Length"]
pub type DEST_MSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEST_MSIZE` writer - Destination Burst Transaction Length"]
pub type DEST_MSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTLL_SPEC, u8, u8, 3, O>;
#[doc = "Field `SRC_MSIZE` reader - Source Burst Transaction Length"]
pub type SRC_MSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SRC_MSIZE` writer - Source Burst Transaction Length"]
pub type SRC_MSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTLL_SPEC, u8, u8, 3, O>;
#[doc = "Field `SRC_GATHER_EN` reader - Source gather enable"]
pub type SRC_GATHER_EN_R = crate::BitReader<SRC_GATHER_EN_A>;
#[doc = "Source gather enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRC_GATHER_EN_A {
    #[doc = "0: Gather disabled"]
    VALUE1 = 0,
    #[doc = "1: Gather enabled"]
    VALUE2 = 1,
}
impl From<SRC_GATHER_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SRC_GATHER_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl SRC_GATHER_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_GATHER_EN_A {
        match self.bits {
            false => SRC_GATHER_EN_A::VALUE1,
            true => SRC_GATHER_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SRC_GATHER_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SRC_GATHER_EN_A::VALUE2
    }
}
#[doc = "Field `SRC_GATHER_EN` writer - Source gather enable"]
pub type SRC_GATHER_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTLL_SPEC, SRC_GATHER_EN_A, O>;
impl<'a, const O: u8> SRC_GATHER_EN_W<'a, O> {
    #[doc = "Gather disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRC_GATHER_EN_A::VALUE1)
    }
    #[doc = "Gather enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRC_GATHER_EN_A::VALUE2)
    }
}
#[doc = "Field `DST_SCATTER_EN` reader - Destination scatter enable"]
pub type DST_SCATTER_EN_R = crate::BitReader<DST_SCATTER_EN_A>;
#[doc = "Destination scatter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DST_SCATTER_EN_A {
    #[doc = "0: Scatter disabled"]
    VALUE1 = 0,
    #[doc = "1: Scatter enabled"]
    VALUE2 = 1,
}
impl From<DST_SCATTER_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DST_SCATTER_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DST_SCATTER_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DST_SCATTER_EN_A {
        match self.bits {
            false => DST_SCATTER_EN_A::VALUE1,
            true => DST_SCATTER_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DST_SCATTER_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DST_SCATTER_EN_A::VALUE2
    }
}
#[doc = "Field `DST_SCATTER_EN` writer - Destination scatter enable"]
pub type DST_SCATTER_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTLL_SPEC, DST_SCATTER_EN_A, O>;
impl<'a, const O: u8> DST_SCATTER_EN_W<'a, O> {
    #[doc = "Scatter disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DST_SCATTER_EN_A::VALUE1)
    }
    #[doc = "Scatter enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DST_SCATTER_EN_A::VALUE2)
    }
}
#[doc = "Field `TT_FC` reader - Transfer Type and Flow Control"]
pub type TT_FC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TT_FC` writer - Transfer Type and Flow Control"]
pub type TT_FC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTLL_SPEC, u8, u8, 3, O>;
#[doc = "Field `LLP_DST_EN` reader - Linked List Pointer for Destination Enable"]
pub type LLP_DST_EN_R = crate::BitReader<bool>;
#[doc = "Field `LLP_DST_EN` writer - Linked List Pointer for Destination Enable"]
pub type LLP_DST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTLL_SPEC, bool, O>;
#[doc = "Field `LLP_SRC_EN` reader - Linked List Pointer for Source Enable"]
pub type LLP_SRC_EN_R = crate::BitReader<bool>;
#[doc = "Field `LLP_SRC_EN` writer - Linked List Pointer for Source Enable"]
pub type LLP_SRC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTLL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Interrupt Enable Bit"]
    #[inline(always)]
    pub fn int_en(&self) -> INT_EN_R {
        INT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Destination Transfer Width"]
    #[inline(always)]
    pub fn dst_tr_width(&self) -> DST_TR_WIDTH_R {
        DST_TR_WIDTH_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:6 - Source Transfer Width"]
    #[inline(always)]
    pub fn src_tr_width(&self) -> SRC_TR_WIDTH_R {
        SRC_TR_WIDTH_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:8 - Destination Address Increment"]
    #[inline(always)]
    pub fn dinc(&self) -> DINC_R {
        DINC_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:10 - Source Address Increment"]
    #[inline(always)]
    pub fn sinc(&self) -> SINC_R {
        SINC_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:13 - Destination Burst Transaction Length"]
    #[inline(always)]
    pub fn dest_msize(&self) -> DEST_MSIZE_R {
        DEST_MSIZE_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:16 - Source Burst Transaction Length"]
    #[inline(always)]
    pub fn src_msize(&self) -> SRC_MSIZE_R {
        SRC_MSIZE_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 17 - Source gather enable"]
    #[inline(always)]
    pub fn src_gather_en(&self) -> SRC_GATHER_EN_R {
        SRC_GATHER_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Destination scatter enable"]
    #[inline(always)]
    pub fn dst_scatter_en(&self) -> DST_SCATTER_EN_R {
        DST_SCATTER_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:22 - Transfer Type and Flow Control"]
    #[inline(always)]
    pub fn tt_fc(&self) -> TT_FC_R {
        TT_FC_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 27 - Linked List Pointer for Destination Enable"]
    #[inline(always)]
    pub fn llp_dst_en(&self) -> LLP_DST_EN_R {
        LLP_DST_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Linked List Pointer for Source Enable"]
    #[inline(always)]
    pub fn llp_src_en(&self) -> LLP_SRC_EN_R {
        LLP_SRC_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn int_en(&mut self) -> INT_EN_W<0> {
        INT_EN_W::new(self)
    }
    #[doc = "Bits 1:3 - Destination Transfer Width"]
    #[inline(always)]
    #[must_use]
    pub fn dst_tr_width(&mut self) -> DST_TR_WIDTH_W<1> {
        DST_TR_WIDTH_W::new(self)
    }
    #[doc = "Bits 4:6 - Source Transfer Width"]
    #[inline(always)]
    #[must_use]
    pub fn src_tr_width(&mut self) -> SRC_TR_WIDTH_W<4> {
        SRC_TR_WIDTH_W::new(self)
    }
    #[doc = "Bits 7:8 - Destination Address Increment"]
    #[inline(always)]
    #[must_use]
    pub fn dinc(&mut self) -> DINC_W<7> {
        DINC_W::new(self)
    }
    #[doc = "Bits 9:10 - Source Address Increment"]
    #[inline(always)]
    #[must_use]
    pub fn sinc(&mut self) -> SINC_W<9> {
        SINC_W::new(self)
    }
    #[doc = "Bits 11:13 - Destination Burst Transaction Length"]
    #[inline(always)]
    #[must_use]
    pub fn dest_msize(&mut self) -> DEST_MSIZE_W<11> {
        DEST_MSIZE_W::new(self)
    }
    #[doc = "Bits 14:16 - Source Burst Transaction Length"]
    #[inline(always)]
    #[must_use]
    pub fn src_msize(&mut self) -> SRC_MSIZE_W<14> {
        SRC_MSIZE_W::new(self)
    }
    #[doc = "Bit 17 - Source gather enable"]
    #[inline(always)]
    #[must_use]
    pub fn src_gather_en(&mut self) -> SRC_GATHER_EN_W<17> {
        SRC_GATHER_EN_W::new(self)
    }
    #[doc = "Bit 18 - Destination scatter enable"]
    #[inline(always)]
    #[must_use]
    pub fn dst_scatter_en(&mut self) -> DST_SCATTER_EN_W<18> {
        DST_SCATTER_EN_W::new(self)
    }
    #[doc = "Bits 20:22 - Transfer Type and Flow Control"]
    #[inline(always)]
    #[must_use]
    pub fn tt_fc(&mut self) -> TT_FC_W<20> {
        TT_FC_W::new(self)
    }
    #[doc = "Bit 27 - Linked List Pointer for Destination Enable"]
    #[inline(always)]
    #[must_use]
    pub fn llp_dst_en(&mut self) -> LLP_DST_EN_W<27> {
        LLP_DST_EN_W::new(self)
    }
    #[doc = "Bit 28 - Linked List Pointer for Source Enable"]
    #[inline(always)]
    #[must_use]
    pub fn llp_src_en(&mut self) -> LLP_SRC_EN_W<28> {
        LLP_SRC_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctll](index.html) module"]
pub struct CTLL_SPEC;
impl crate::RegisterSpec for CTLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctll::R](R) reader structure"]
impl crate::Readable for CTLL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctll::W](W) writer structure"]
impl crate::Writable for CTLL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTLL to value 0x0030_4801"]
impl crate::Resettable for CTLL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0030_4801;
}
