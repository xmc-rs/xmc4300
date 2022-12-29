#[doc = "Register `NCR` reader"]
pub struct R(crate::R<NCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NCR` writer"]
pub struct W(crate::W<NCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NCR_SPEC>;
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
impl From<crate::W<NCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INIT` reader - Node Initialization"]
pub type INIT_R = crate::BitReader<INIT_A>;
#[doc = "Node Initialization\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INIT_A {
    #[doc = "0: Resetting bit INIT enables the participation of the node in the CAN traffic. If the CAN node is in the bus-off state, the ongoing bus-off recovery (which does not depend on the INIT bit) is continued. With the end of the bus-off recovery sequence the CAN node is allowed to take part in the CAN traffic. If the CAN node is not in the bus-off state, a sequence of 11 consecutive recessive bits must be detected before the node is allowed to take part in the CAN traffic."]
    VALUE1 = 0,
    #[doc = "1: Setting this bit terminates the participation of this node in the CAN traffic. Any ongoing frame transfer is cancelled and the transmit line goes recessive. If the CAN node is in the bus-off state, then the running bus-off recovery sequence is continued. If the INIT bit is still set after the successful completion of the bus-off recovery sequence, i.e. after detecting 128 sequences of 11 consecutive recessive bits (11 1), then the CAN node leaves the bus-off state but remains inactive as long as INIT remains set."]
    VALUE2 = 1,
}
impl From<INIT_A> for bool {
    #[inline(always)]
    fn from(variant: INIT_A) -> Self {
        variant as u8 != 0
    }
}
impl INIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INIT_A {
        match self.bits {
            false => INIT_A::VALUE1,
            true => INIT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == INIT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == INIT_A::VALUE2
    }
}
#[doc = "Field `INIT` writer - Node Initialization"]
pub type INIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCR_SPEC, INIT_A, O>;
impl<'a, const O: u8> INIT_W<'a, O> {
    #[doc = "Resetting bit INIT enables the participation of the node in the CAN traffic. If the CAN node is in the bus-off state, the ongoing bus-off recovery (which does not depend on the INIT bit) is continued. With the end of the bus-off recovery sequence the CAN node is allowed to take part in the CAN traffic. If the CAN node is not in the bus-off state, a sequence of 11 consecutive recessive bits must be detected before the node is allowed to take part in the CAN traffic."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(INIT_A::VALUE1)
    }
    #[doc = "Setting this bit terminates the participation of this node in the CAN traffic. Any ongoing frame transfer is cancelled and the transmit line goes recessive. If the CAN node is in the bus-off state, then the running bus-off recovery sequence is continued. If the INIT bit is still set after the successful completion of the bus-off recovery sequence, i.e. after detecting 128 sequences of 11 consecutive recessive bits (11 1), then the CAN node leaves the bus-off state but remains inactive as long as INIT remains set."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(INIT_A::VALUE2)
    }
}
#[doc = "Field `TRIE` reader - Transfer Interrupt Enable"]
pub type TRIE_R = crate::BitReader<TRIE_A>;
#[doc = "Transfer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIE_A {
    #[doc = "0: Transfer interrupt is disabled."]
    VALUE1 = 0,
    #[doc = "1: Transfer interrupt is enabled."]
    VALUE2 = 1,
}
impl From<TRIE_A> for bool {
    #[inline(always)]
    fn from(variant: TRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIE_A {
        match self.bits {
            false => TRIE_A::VALUE1,
            true => TRIE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TRIE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TRIE_A::VALUE2
    }
}
#[doc = "Field `TRIE` writer - Transfer Interrupt Enable"]
pub type TRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCR_SPEC, TRIE_A, O>;
impl<'a, const O: u8> TRIE_W<'a, O> {
    #[doc = "Transfer interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRIE_A::VALUE1)
    }
    #[doc = "Transfer interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRIE_A::VALUE2)
    }
}
#[doc = "Field `LECIE` reader - LEC Indicated Error Interrupt Enable"]
pub type LECIE_R = crate::BitReader<LECIE_A>;
#[doc = "LEC Indicated Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LECIE_A {
    #[doc = "0: Last error code interrupt is disabled."]
    VALUE1 = 0,
    #[doc = "1: Last error code interrupt is enabled."]
    VALUE2 = 1,
}
impl From<LECIE_A> for bool {
    #[inline(always)]
    fn from(variant: LECIE_A) -> Self {
        variant as u8 != 0
    }
}
impl LECIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LECIE_A {
        match self.bits {
            false => LECIE_A::VALUE1,
            true => LECIE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LECIE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LECIE_A::VALUE2
    }
}
#[doc = "Field `LECIE` writer - LEC Indicated Error Interrupt Enable"]
pub type LECIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCR_SPEC, LECIE_A, O>;
impl<'a, const O: u8> LECIE_W<'a, O> {
    #[doc = "Last error code interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LECIE_A::VALUE1)
    }
    #[doc = "Last error code interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LECIE_A::VALUE2)
    }
}
#[doc = "Field `ALIE` reader - Alert Interrupt Enable"]
pub type ALIE_R = crate::BitReader<ALIE_A>;
#[doc = "Alert Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALIE_A {
    #[doc = "0: Alert interrupt is disabled."]
    VALUE1 = 0,
    #[doc = "1: Alert interrupt is enabled."]
    VALUE2 = 1,
}
impl From<ALIE_A> for bool {
    #[inline(always)]
    fn from(variant: ALIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ALIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALIE_A {
        match self.bits {
            false => ALIE_A::VALUE1,
            true => ALIE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALIE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALIE_A::VALUE2
    }
}
#[doc = "Field `ALIE` writer - Alert Interrupt Enable"]
pub type ALIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCR_SPEC, ALIE_A, O>;
impl<'a, const O: u8> ALIE_W<'a, O> {
    #[doc = "Alert interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALIE_A::VALUE1)
    }
    #[doc = "Alert interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALIE_A::VALUE2)
    }
}
#[doc = "Field `CANDIS` reader - CAN Disable"]
pub type CANDIS_R = crate::BitReader<bool>;
#[doc = "Field `CANDIS` writer - CAN Disable"]
pub type CANDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCR_SPEC, bool, O>;
#[doc = "Field `TXDIS` reader - Transmit Disable"]
pub type TXDIS_R = crate::BitReader<bool>;
#[doc = "Field `TXDIS` writer - Transmit Disable"]
pub type TXDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCR_SPEC, bool, O>;
#[doc = "Field `CCE` reader - Configuration Change Enable"]
pub type CCE_R = crate::BitReader<CCE_A>;
#[doc = "Configuration Change Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCE_A {
    #[doc = "0: The Bit Timing Register, the Port Control Register, Error Counter Register may only be read. All attempts to modify them are ignored."]
    VALUE1 = 0,
    #[doc = "1: The Bit Timing Register, the Port Control Register, Error Counter Register may be read and written."]
    VALUE2 = 1,
}
impl From<CCE_A> for bool {
    #[inline(always)]
    fn from(variant: CCE_A) -> Self {
        variant as u8 != 0
    }
}
impl CCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCE_A {
        match self.bits {
            false => CCE_A::VALUE1,
            true => CCE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CCE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CCE_A::VALUE2
    }
}
#[doc = "Field `CCE` writer - Configuration Change Enable"]
pub type CCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCR_SPEC, CCE_A, O>;
impl<'a, const O: u8> CCE_W<'a, O> {
    #[doc = "The Bit Timing Register, the Port Control Register, Error Counter Register may only be read. All attempts to modify them are ignored."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCE_A::VALUE1)
    }
    #[doc = "The Bit Timing Register, the Port Control Register, Error Counter Register may be read and written."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCE_A::VALUE2)
    }
}
#[doc = "Field `CALM` reader - CAN Analyzer Mode"]
pub type CALM_R = crate::BitReader<bool>;
#[doc = "Field `CALM` writer - CAN Analyzer Mode"]
pub type CALM_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Node Initialization"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Interrupt Enable"]
    #[inline(always)]
    pub fn trie(&self) -> TRIE_R {
        TRIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LEC Indicated Error Interrupt Enable"]
    #[inline(always)]
    pub fn lecie(&self) -> LECIE_R {
        LECIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Alert Interrupt Enable"]
    #[inline(always)]
    pub fn alie(&self) -> ALIE_R {
        ALIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CAN Disable"]
    #[inline(always)]
    pub fn candis(&self) -> CANDIS_R {
        CANDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Disable"]
    #[inline(always)]
    pub fn txdis(&self) -> TXDIS_R {
        TXDIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configuration Change Enable"]
    #[inline(always)]
    pub fn cce(&self) -> CCE_R {
        CCE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CAN Analyzer Mode"]
    #[inline(always)]
    pub fn calm(&self) -> CALM_R {
        CALM_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Node Initialization"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<0> {
        INIT_W::new(self)
    }
    #[doc = "Bit 1 - Transfer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trie(&mut self) -> TRIE_W<1> {
        TRIE_W::new(self)
    }
    #[doc = "Bit 2 - LEC Indicated Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lecie(&mut self) -> LECIE_W<2> {
        LECIE_W::new(self)
    }
    #[doc = "Bit 3 - Alert Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn alie(&mut self) -> ALIE_W<3> {
        ALIE_W::new(self)
    }
    #[doc = "Bit 4 - CAN Disable"]
    #[inline(always)]
    #[must_use]
    pub fn candis(&mut self) -> CANDIS_W<4> {
        CANDIS_W::new(self)
    }
    #[doc = "Bit 5 - Transmit Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txdis(&mut self) -> TXDIS_W<5> {
        TXDIS_W::new(self)
    }
    #[doc = "Bit 6 - Configuration Change Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cce(&mut self) -> CCE_W<6> {
        CCE_W::new(self)
    }
    #[doc = "Bit 7 - CAN Analyzer Mode"]
    #[inline(always)]
    #[must_use]
    pub fn calm(&mut self) -> CALM_W<7> {
        CALM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Node Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ncr](index.html) module"]
pub struct NCR_SPEC;
impl crate::RegisterSpec for NCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ncr::R](R) reader structure"]
impl crate::Readable for NCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ncr::W](W) writer structure"]
impl crate::Writable for NCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NCR to value 0x41"]
impl crate::Resettable for NCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x41;
}
