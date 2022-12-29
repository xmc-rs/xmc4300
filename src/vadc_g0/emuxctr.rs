#[doc = "Register `EMUXCTR` reader"]
pub struct R(crate::R<EMUXCTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMUXCTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMUXCTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMUXCTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMUXCTR` writer"]
pub struct W(crate::W<EMUXCTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMUXCTR_SPEC>;
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
impl From<crate::W<EMUXCTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMUXCTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMUXSET` reader - External Multiplexer Start Selection"]
pub type EMUXSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EMUXSET` writer - External Multiplexer Start Selection"]
pub type EMUXSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EMUXCTR_SPEC, u8, u8, 3, O>;
#[doc = "Field `EMUXACT` reader - External Multiplexer Actual Selection"]
pub type EMUXACT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EMUXCH` reader - External Multiplexer Channel Select"]
pub type EMUXCH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EMUXCH` writer - External Multiplexer Channel Select"]
pub type EMUXCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EMUXCTR_SPEC, u16, u16, 10, O>;
#[doc = "Field `EMUXMODE` reader - External Multiplexer Mode"]
pub type EMUXMODE_R = crate::FieldReader<u8, EMUXMODE_A>;
#[doc = "External Multiplexer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EMUXMODE_A {
    #[doc = "0: Software control (no hardware action)"]
    VALUE1 = 0,
    #[doc = "1: Steady mode (use EMUXSET value)"]
    VALUE2 = 1,
    #[doc = "2: Single-step mode"]
    VALUE3 = 2,
    #[doc = "3: Sequence mode"]
    VALUE4 = 3,
}
impl From<EMUXMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: EMUXMODE_A) -> Self {
        variant as _
    }
}
impl EMUXMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMUXMODE_A {
        match self.bits {
            0 => EMUXMODE_A::VALUE1,
            1 => EMUXMODE_A::VALUE2,
            2 => EMUXMODE_A::VALUE3,
            3 => EMUXMODE_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EMUXMODE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EMUXMODE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == EMUXMODE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == EMUXMODE_A::VALUE4
    }
}
#[doc = "Field `EMUXMODE` writer - External Multiplexer Mode"]
pub type EMUXMODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, EMUXCTR_SPEC, u8, EMUXMODE_A, 2, O>;
impl<'a, const O: u8> EMUXMODE_W<'a, O> {
    #[doc = "Software control (no hardware action)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EMUXMODE_A::VALUE1)
    }
    #[doc = "Steady mode (use EMUXSET value)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EMUXMODE_A::VALUE2)
    }
    #[doc = "Single-step mode"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(EMUXMODE_A::VALUE3)
    }
    #[doc = "Sequence mode"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(EMUXMODE_A::VALUE4)
    }
}
#[doc = "Field `EMXCOD` reader - External Multiplexer Coding Scheme"]
pub type EMXCOD_R = crate::BitReader<EMXCOD_A>;
#[doc = "External Multiplexer Coding Scheme\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EMXCOD_A {
    #[doc = "0: Output the channel number in binary code"]
    VALUE1 = 0,
    #[doc = "1: Output the channel number in Gray code"]
    VALUE2 = 1,
}
impl From<EMXCOD_A> for bool {
    #[inline(always)]
    fn from(variant: EMXCOD_A) -> Self {
        variant as u8 != 0
    }
}
impl EMXCOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMXCOD_A {
        match self.bits {
            false => EMXCOD_A::VALUE1,
            true => EMXCOD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EMXCOD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EMXCOD_A::VALUE2
    }
}
#[doc = "Field `EMXCOD` writer - External Multiplexer Coding Scheme"]
pub type EMXCOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMUXCTR_SPEC, EMXCOD_A, O>;
impl<'a, const O: u8> EMXCOD_W<'a, O> {
    #[doc = "Output the channel number in binary code"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EMXCOD_A::VALUE1)
    }
    #[doc = "Output the channel number in Gray code"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EMXCOD_A::VALUE2)
    }
}
#[doc = "Field `EMXST` reader - External Multiplexer Sample Time Control"]
pub type EMXST_R = crate::BitReader<EMXST_A>;
#[doc = "External Multiplexer Sample Time Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EMXST_A {
    #[doc = "0: Use STCE whenever the setting changes"]
    VALUE1 = 0,
    #[doc = "1: Use STCE for each conversion of an external channel"]
    VALUE2 = 1,
}
impl From<EMXST_A> for bool {
    #[inline(always)]
    fn from(variant: EMXST_A) -> Self {
        variant as u8 != 0
    }
}
impl EMXST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMXST_A {
        match self.bits {
            false => EMXST_A::VALUE1,
            true => EMXST_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EMXST_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EMXST_A::VALUE2
    }
}
#[doc = "Field `EMXST` writer - External Multiplexer Sample Time Control"]
pub type EMXST_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMUXCTR_SPEC, EMXST_A, O>;
impl<'a, const O: u8> EMXST_W<'a, O> {
    #[doc = "Use STCE whenever the setting changes"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EMXST_A::VALUE1)
    }
    #[doc = "Use STCE for each conversion of an external channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EMXST_A::VALUE2)
    }
}
#[doc = "Field `EMXCSS` reader - External Multiplexer Channel Selection Style"]
pub type EMXCSS_R = crate::BitReader<EMXCSS_A>;
#[doc = "External Multiplexer Channel Selection Style\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EMXCSS_A {
    #[doc = "0: Channel number: Bitfield EMUXCH selects an arbitrary channel"]
    VALUE1 = 0,
    #[doc = "1: Channel enable: Each bit of bitfield EMUXCH selects the associated channel for EMUX control"]
    VALUE2 = 1,
}
impl From<EMXCSS_A> for bool {
    #[inline(always)]
    fn from(variant: EMXCSS_A) -> Self {
        variant as u8 != 0
    }
}
impl EMXCSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMXCSS_A {
        match self.bits {
            false => EMXCSS_A::VALUE1,
            true => EMXCSS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EMXCSS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EMXCSS_A::VALUE2
    }
}
#[doc = "Write Control for EMUX Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EMXWC_AW {
    #[doc = "0: No write access to EMUX cfg."]
    VALUE1 = 0,
    #[doc = "1: Bitfields EMXMODE, EMXCOD, EMXST, EMXCSS can be written"]
    VALUE2 = 1,
}
impl From<EMXWC_AW> for bool {
    #[inline(always)]
    fn from(variant: EMXWC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EMXWC` writer - Write Control for EMUX Configuration"]
pub type EMXWC_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMUXCTR_SPEC, EMXWC_AW, O>;
impl<'a, const O: u8> EMXWC_W<'a, O> {
    #[doc = "No write access to EMUX cfg."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EMXWC_AW::VALUE1)
    }
    #[doc = "Bitfields EMXMODE, EMXCOD, EMXST, EMXCSS can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EMXWC_AW::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:2 - External Multiplexer Start Selection"]
    #[inline(always)]
    pub fn emuxset(&self) -> EMUXSET_R {
        EMUXSET_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - External Multiplexer Actual Selection"]
    #[inline(always)]
    pub fn emuxact(&self) -> EMUXACT_R {
        EMUXACT_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:25 - External Multiplexer Channel Select"]
    #[inline(always)]
    pub fn emuxch(&self) -> EMUXCH_R {
        EMUXCH_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 26:27 - External Multiplexer Mode"]
    #[inline(always)]
    pub fn emuxmode(&self) -> EMUXMODE_R {
        EMUXMODE_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - External Multiplexer Coding Scheme"]
    #[inline(always)]
    pub fn emxcod(&self) -> EMXCOD_R {
        EMXCOD_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - External Multiplexer Sample Time Control"]
    #[inline(always)]
    pub fn emxst(&self) -> EMXST_R {
        EMXST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - External Multiplexer Channel Selection Style"]
    #[inline(always)]
    pub fn emxcss(&self) -> EMXCSS_R {
        EMXCSS_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - External Multiplexer Start Selection"]
    #[inline(always)]
    #[must_use]
    pub fn emuxset(&mut self) -> EMUXSET_W<0> {
        EMUXSET_W::new(self)
    }
    #[doc = "Bits 16:25 - External Multiplexer Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn emuxch(&mut self) -> EMUXCH_W<16> {
        EMUXCH_W::new(self)
    }
    #[doc = "Bits 26:27 - External Multiplexer Mode"]
    #[inline(always)]
    #[must_use]
    pub fn emuxmode(&mut self) -> EMUXMODE_W<26> {
        EMUXMODE_W::new(self)
    }
    #[doc = "Bit 28 - External Multiplexer Coding Scheme"]
    #[inline(always)]
    #[must_use]
    pub fn emxcod(&mut self) -> EMXCOD_W<28> {
        EMXCOD_W::new(self)
    }
    #[doc = "Bit 29 - External Multiplexer Sample Time Control"]
    #[inline(always)]
    #[must_use]
    pub fn emxst(&mut self) -> EMXST_W<29> {
        EMXST_W::new(self)
    }
    #[doc = "Bit 31 - Write Control for EMUX Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn emxwc(&mut self) -> EMXWC_W<31> {
        EMXWC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "E0ternal Multiplexer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emuxctr](index.html) module"]
pub struct EMUXCTR_SPEC;
impl crate::RegisterSpec for EMUXCTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emuxctr::R](R) reader structure"]
impl crate::Readable for EMUXCTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emuxctr::W](W) writer structure"]
impl crate::Writable for EMUXCTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EMUXCTR to value 0"]
impl crate::Resettable for EMUXCTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
