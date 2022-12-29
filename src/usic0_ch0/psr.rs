#[doc = "Register `PSR` reader"]
pub struct R(crate::R<PSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSR` writer"]
pub struct W(crate::W<PSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSR_SPEC>;
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
impl From<crate::W<PSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ST0` reader - Protocol Status Flag 0"]
pub type ST0_R = crate::BitReader<bool>;
#[doc = "Field `ST0` writer - Protocol Status Flag 0"]
pub type ST0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
#[doc = "Field `ST1` reader - Protocol Status Flag 1"]
pub type ST1_R = crate::BitReader<bool>;
#[doc = "Field `ST1` writer - Protocol Status Flag 1"]
pub type ST1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
#[doc = "Field `ST2` reader - Protocol Status Flag 2"]
pub type ST2_R = crate::BitReader<bool>;
#[doc = "Field `ST2` writer - Protocol Status Flag 2"]
pub type ST2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
#[doc = "Field `ST3` reader - Protocol Status Flag 3"]
pub type ST3_R = crate::BitReader<bool>;
#[doc = "Field `ST3` writer - Protocol Status Flag 3"]
pub type ST3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
#[doc = "Field `ST4` reader - Protocol Status Flag 4"]
pub type ST4_R = crate::BitReader<bool>;
#[doc = "Field `ST4` writer - Protocol Status Flag 4"]
pub type ST4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
#[doc = "Field `ST5` reader - Protocol Status Flag 5"]
pub type ST5_R = crate::BitReader<bool>;
#[doc = "Field `ST5` writer - Protocol Status Flag 5"]
pub type ST5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
#[doc = "Field `ST6` reader - Protocol Status Flag 6"]
pub type ST6_R = crate::BitReader<bool>;
#[doc = "Field `ST6` writer - Protocol Status Flag 6"]
pub type ST6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
#[doc = "Field `ST7` reader - Protocol Status Flag 7"]
pub type ST7_R = crate::BitReader<bool>;
#[doc = "Field `ST7` writer - Protocol Status Flag 7"]
pub type ST7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
#[doc = "Field `ST8` reader - Protocol Status Flag 8"]
pub type ST8_R = crate::BitReader<bool>;
#[doc = "Field `ST8` writer - Protocol Status Flag 8"]
pub type ST8_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
#[doc = "Field `ST9` reader - Protocol Status Flag 9"]
pub type ST9_R = crate::BitReader<bool>;
#[doc = "Field `ST9` writer - Protocol Status Flag 9"]
pub type ST9_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
#[doc = "Field `RSIF` reader - Receiver Start Indication Flag"]
pub type RSIF_R = crate::BitReader<RSIF_A>;
#[doc = "Receiver Start Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSIF_A {
    #[doc = "0: A receiver start event has not occurred."]
    VALUE1 = 0,
    #[doc = "1: A receiver start event has occurred."]
    VALUE2 = 1,
}
impl From<RSIF_A> for bool {
    #[inline(always)]
    fn from(variant: RSIF_A) -> Self {
        variant as u8 != 0
    }
}
impl RSIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSIF_A {
        match self.bits {
            false => RSIF_A::VALUE1,
            true => RSIF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RSIF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RSIF_A::VALUE2
    }
}
#[doc = "Field `RSIF` writer - Receiver Start Indication Flag"]
pub type RSIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, RSIF_A, O>;
impl<'a, const O: u8> RSIF_W<'a, O> {
    #[doc = "A receiver start event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RSIF_A::VALUE1)
    }
    #[doc = "A receiver start event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RSIF_A::VALUE2)
    }
}
#[doc = "Field `DLIF` reader - Data Lost Indication Flag"]
pub type DLIF_R = crate::BitReader<DLIF_A>;
#[doc = "Data Lost Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLIF_A {
    #[doc = "0: A data lost event has not occurred."]
    VALUE1 = 0,
    #[doc = "1: A data lost event has occurred."]
    VALUE2 = 1,
}
impl From<DLIF_A> for bool {
    #[inline(always)]
    fn from(variant: DLIF_A) -> Self {
        variant as u8 != 0
    }
}
impl DLIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLIF_A {
        match self.bits {
            false => DLIF_A::VALUE1,
            true => DLIF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DLIF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DLIF_A::VALUE2
    }
}
#[doc = "Field `DLIF` writer - Data Lost Indication Flag"]
pub type DLIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, DLIF_A, O>;
impl<'a, const O: u8> DLIF_W<'a, O> {
    #[doc = "A data lost event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DLIF_A::VALUE1)
    }
    #[doc = "A data lost event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DLIF_A::VALUE2)
    }
}
#[doc = "Field `TSIF` reader - Transmit Shift Indication Flag"]
pub type TSIF_R = crate::BitReader<TSIF_A>;
#[doc = "Transmit Shift Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSIF_A {
    #[doc = "0: A transmit shift event has not occurred."]
    VALUE1 = 0,
    #[doc = "1: A transmit shift event has occurred."]
    VALUE2 = 1,
}
impl From<TSIF_A> for bool {
    #[inline(always)]
    fn from(variant: TSIF_A) -> Self {
        variant as u8 != 0
    }
}
impl TSIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSIF_A {
        match self.bits {
            false => TSIF_A::VALUE1,
            true => TSIF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TSIF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TSIF_A::VALUE2
    }
}
#[doc = "Field `TSIF` writer - Transmit Shift Indication Flag"]
pub type TSIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, TSIF_A, O>;
impl<'a, const O: u8> TSIF_W<'a, O> {
    #[doc = "A transmit shift event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TSIF_A::VALUE1)
    }
    #[doc = "A transmit shift event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TSIF_A::VALUE2)
    }
}
#[doc = "Field `TBIF` reader - Transmit Buffer Indication Flag"]
pub type TBIF_R = crate::BitReader<TBIF_A>;
#[doc = "Transmit Buffer Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBIF_A {
    #[doc = "0: A transmit buffer event has not occurred."]
    VALUE1 = 0,
    #[doc = "1: A transmit buffer event has occurred."]
    VALUE2 = 1,
}
impl From<TBIF_A> for bool {
    #[inline(always)]
    fn from(variant: TBIF_A) -> Self {
        variant as u8 != 0
    }
}
impl TBIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBIF_A {
        match self.bits {
            false => TBIF_A::VALUE1,
            true => TBIF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TBIF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TBIF_A::VALUE2
    }
}
#[doc = "Field `TBIF` writer - Transmit Buffer Indication Flag"]
pub type TBIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, TBIF_A, O>;
impl<'a, const O: u8> TBIF_W<'a, O> {
    #[doc = "A transmit buffer event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TBIF_A::VALUE1)
    }
    #[doc = "A transmit buffer event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TBIF_A::VALUE2)
    }
}
#[doc = "Field `RIF` reader - Receive Indication Flag"]
pub type RIF_R = crate::BitReader<RIF_A>;
#[doc = "Receive Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RIF_A {
    #[doc = "0: A receive event has not occurred."]
    VALUE1 = 0,
    #[doc = "1: A receive event has occurred."]
    VALUE2 = 1,
}
impl From<RIF_A> for bool {
    #[inline(always)]
    fn from(variant: RIF_A) -> Self {
        variant as u8 != 0
    }
}
impl RIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RIF_A {
        match self.bits {
            false => RIF_A::VALUE1,
            true => RIF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RIF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RIF_A::VALUE2
    }
}
#[doc = "Field `RIF` writer - Receive Indication Flag"]
pub type RIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, RIF_A, O>;
impl<'a, const O: u8> RIF_W<'a, O> {
    #[doc = "A receive event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RIF_A::VALUE1)
    }
    #[doc = "A receive event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RIF_A::VALUE2)
    }
}
#[doc = "Field `AIF` reader - Alternative Receive Indication Flag"]
pub type AIF_R = crate::BitReader<AIF_A>;
#[doc = "Alternative Receive Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AIF_A {
    #[doc = "0: An alternative receive event has not occurred."]
    VALUE1 = 0,
    #[doc = "1: An alternative receive event has occurred."]
    VALUE2 = 1,
}
impl From<AIF_A> for bool {
    #[inline(always)]
    fn from(variant: AIF_A) -> Self {
        variant as u8 != 0
    }
}
impl AIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AIF_A {
        match self.bits {
            false => AIF_A::VALUE1,
            true => AIF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AIF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AIF_A::VALUE2
    }
}
#[doc = "Field `AIF` writer - Alternative Receive Indication Flag"]
pub type AIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, AIF_A, O>;
impl<'a, const O: u8> AIF_W<'a, O> {
    #[doc = "An alternative receive event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AIF_A::VALUE1)
    }
    #[doc = "An alternative receive event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(AIF_A::VALUE2)
    }
}
#[doc = "Field `BRGIF` reader - Baud Rate Generator Indication Flag"]
pub type BRGIF_R = crate::BitReader<BRGIF_A>;
#[doc = "Baud Rate Generator Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRGIF_A {
    #[doc = "0: A baud rate generator event has not occurred."]
    VALUE1 = 0,
    #[doc = "1: A baud rate generator event has occurred."]
    VALUE2 = 1,
}
impl From<BRGIF_A> for bool {
    #[inline(always)]
    fn from(variant: BRGIF_A) -> Self {
        variant as u8 != 0
    }
}
impl BRGIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRGIF_A {
        match self.bits {
            false => BRGIF_A::VALUE1,
            true => BRGIF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BRGIF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BRGIF_A::VALUE2
    }
}
#[doc = "Field `BRGIF` writer - Baud Rate Generator Indication Flag"]
pub type BRGIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, BRGIF_A, O>;
impl<'a, const O: u8> BRGIF_W<'a, O> {
    #[doc = "A baud rate generator event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BRGIF_A::VALUE1)
    }
    #[doc = "A baud rate generator event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BRGIF_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Protocol Status Flag 0"]
    #[inline(always)]
    pub fn st0(&self) -> ST0_R {
        ST0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Protocol Status Flag 1"]
    #[inline(always)]
    pub fn st1(&self) -> ST1_R {
        ST1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Protocol Status Flag 2"]
    #[inline(always)]
    pub fn st2(&self) -> ST2_R {
        ST2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Protocol Status Flag 3"]
    #[inline(always)]
    pub fn st3(&self) -> ST3_R {
        ST3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Protocol Status Flag 4"]
    #[inline(always)]
    pub fn st4(&self) -> ST4_R {
        ST4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Protocol Status Flag 5"]
    #[inline(always)]
    pub fn st5(&self) -> ST5_R {
        ST5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Protocol Status Flag 6"]
    #[inline(always)]
    pub fn st6(&self) -> ST6_R {
        ST6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Protocol Status Flag 7"]
    #[inline(always)]
    pub fn st7(&self) -> ST7_R {
        ST7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Protocol Status Flag 8"]
    #[inline(always)]
    pub fn st8(&self) -> ST8_R {
        ST8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Protocol Status Flag 9"]
    #[inline(always)]
    pub fn st9(&self) -> ST9_R {
        ST9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Receiver Start Indication Flag"]
    #[inline(always)]
    pub fn rsif(&self) -> RSIF_R {
        RSIF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Data Lost Indication Flag"]
    #[inline(always)]
    pub fn dlif(&self) -> DLIF_R {
        DLIF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit Shift Indication Flag"]
    #[inline(always)]
    pub fn tsif(&self) -> TSIF_R {
        TSIF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmit Buffer Indication Flag"]
    #[inline(always)]
    pub fn tbif(&self) -> TBIF_R {
        TBIF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Receive Indication Flag"]
    #[inline(always)]
    pub fn rif(&self) -> RIF_R {
        RIF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Alternative Receive Indication Flag"]
    #[inline(always)]
    pub fn aif(&self) -> AIF_R {
        AIF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Baud Rate Generator Indication Flag"]
    #[inline(always)]
    pub fn brgif(&self) -> BRGIF_R {
        BRGIF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Protocol Status Flag 0"]
    #[inline(always)]
    #[must_use]
    pub fn st0(&mut self) -> ST0_W<0> {
        ST0_W::new(self)
    }
    #[doc = "Bit 1 - Protocol Status Flag 1"]
    #[inline(always)]
    #[must_use]
    pub fn st1(&mut self) -> ST1_W<1> {
        ST1_W::new(self)
    }
    #[doc = "Bit 2 - Protocol Status Flag 2"]
    #[inline(always)]
    #[must_use]
    pub fn st2(&mut self) -> ST2_W<2> {
        ST2_W::new(self)
    }
    #[doc = "Bit 3 - Protocol Status Flag 3"]
    #[inline(always)]
    #[must_use]
    pub fn st3(&mut self) -> ST3_W<3> {
        ST3_W::new(self)
    }
    #[doc = "Bit 4 - Protocol Status Flag 4"]
    #[inline(always)]
    #[must_use]
    pub fn st4(&mut self) -> ST4_W<4> {
        ST4_W::new(self)
    }
    #[doc = "Bit 5 - Protocol Status Flag 5"]
    #[inline(always)]
    #[must_use]
    pub fn st5(&mut self) -> ST5_W<5> {
        ST5_W::new(self)
    }
    #[doc = "Bit 6 - Protocol Status Flag 6"]
    #[inline(always)]
    #[must_use]
    pub fn st6(&mut self) -> ST6_W<6> {
        ST6_W::new(self)
    }
    #[doc = "Bit 7 - Protocol Status Flag 7"]
    #[inline(always)]
    #[must_use]
    pub fn st7(&mut self) -> ST7_W<7> {
        ST7_W::new(self)
    }
    #[doc = "Bit 8 - Protocol Status Flag 8"]
    #[inline(always)]
    #[must_use]
    pub fn st8(&mut self) -> ST8_W<8> {
        ST8_W::new(self)
    }
    #[doc = "Bit 9 - Protocol Status Flag 9"]
    #[inline(always)]
    #[must_use]
    pub fn st9(&mut self) -> ST9_W<9> {
        ST9_W::new(self)
    }
    #[doc = "Bit 10 - Receiver Start Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rsif(&mut self) -> RSIF_W<10> {
        RSIF_W::new(self)
    }
    #[doc = "Bit 11 - Data Lost Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dlif(&mut self) -> DLIF_W<11> {
        DLIF_W::new(self)
    }
    #[doc = "Bit 12 - Transmit Shift Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tsif(&mut self) -> TSIF_W<12> {
        TSIF_W::new(self)
    }
    #[doc = "Bit 13 - Transmit Buffer Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tbif(&mut self) -> TBIF_W<13> {
        TBIF_W::new(self)
    }
    #[doc = "Bit 14 - Receive Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rif(&mut self) -> RIF_W<14> {
        RIF_W::new(self)
    }
    #[doc = "Bit 15 - Alternative Receive Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn aif(&mut self) -> AIF_W<15> {
        AIF_W::new(self)
    }
    #[doc = "Bit 16 - Baud Rate Generator Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn brgif(&mut self) -> BRGIF_W<16> {
        BRGIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Protocol Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psr](index.html) module"]
pub struct PSR_SPEC;
impl crate::RegisterSpec for PSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psr::R](R) reader structure"]
impl crate::Readable for PSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psr::W](W) writer structure"]
impl crate::Writable for PSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSR to value 0"]
impl crate::Resettable for PSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
