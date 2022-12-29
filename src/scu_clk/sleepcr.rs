#[doc = "Register `SLEEPCR` reader"]
pub struct R(crate::R<SLEEPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLEEPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLEEPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLEEPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLEEPCR` writer"]
pub struct W(crate::W<SLEEPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLEEPCR_SPEC>;
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
impl From<crate::W<SLEEPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLEEPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSSEL` reader - System Clock Selection Value"]
pub type SYSSEL_R = crate::BitReader<SYSSEL_A>;
#[doc = "System Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSSEL_A {
    #[doc = "0: fOFI clock"]
    CONST_0 = 0,
    #[doc = "1: fPLL clock"]
    CONST_1 = 1,
}
impl From<SYSSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SYSSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSSEL_A {
        match self.bits {
            false => SYSSEL_A::CONST_0,
            true => SYSSEL_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == SYSSEL_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == SYSSEL_A::CONST_1
    }
}
#[doc = "Field `SYSSEL` writer - System Clock Selection Value"]
pub type SYSSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLEEPCR_SPEC, SYSSEL_A, O>;
impl<'a, const O: u8> SYSSEL_W<'a, O> {
    #[doc = "fOFI clock"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(SYSSEL_A::CONST_0)
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(SYSSEL_A::CONST_1)
    }
}
#[doc = "Field `USBCR` reader - USB Clock Control in Sleep Mode"]
pub type USBCR_R = crate::BitReader<USBCR_A>;
#[doc = "USB Clock Control in Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBCR_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<USBCR_A> for bool {
    #[inline(always)]
    fn from(variant: USBCR_A) -> Self {
        variant as u8 != 0
    }
}
impl USBCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBCR_A {
        match self.bits {
            false => USBCR_A::CONST_0,
            true => USBCR_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == USBCR_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == USBCR_A::CONST_1
    }
}
#[doc = "Field `USBCR` writer - USB Clock Control in Sleep Mode"]
pub type USBCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLEEPCR_SPEC, USBCR_A, O>;
impl<'a, const O: u8> USBCR_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(USBCR_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(USBCR_A::CONST_1)
    }
}
#[doc = "Field `MMCCR` reader - MMC Clock Control in Sleep Mode"]
pub type MMCCR_R = crate::BitReader<MMCCR_A>;
#[doc = "MMC Clock Control in Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MMCCR_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<MMCCR_A> for bool {
    #[inline(always)]
    fn from(variant: MMCCR_A) -> Self {
        variant as u8 != 0
    }
}
impl MMCCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MMCCR_A {
        match self.bits {
            false => MMCCR_A::CONST_0,
            true => MMCCR_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == MMCCR_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == MMCCR_A::CONST_1
    }
}
#[doc = "Field `MMCCR` writer - MMC Clock Control in Sleep Mode"]
pub type MMCCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLEEPCR_SPEC, MMCCR_A, O>;
impl<'a, const O: u8> MMCCR_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(MMCCR_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(MMCCR_A::CONST_1)
    }
}
#[doc = "Field `ETH0CR` reader - Ethernet Clock Control in Sleep Mode"]
pub type ETH0CR_R = crate::BitReader<ETH0CR_A>;
#[doc = "Ethernet Clock Control in Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETH0CR_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<ETH0CR_A> for bool {
    #[inline(always)]
    fn from(variant: ETH0CR_A) -> Self {
        variant as u8 != 0
    }
}
impl ETH0CR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETH0CR_A {
        match self.bits {
            false => ETH0CR_A::CONST_0,
            true => ETH0CR_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == ETH0CR_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == ETH0CR_A::CONST_1
    }
}
#[doc = "Field `ETH0CR` writer - Ethernet Clock Control in Sleep Mode"]
pub type ETH0CR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLEEPCR_SPEC, ETH0CR_A, O>;
impl<'a, const O: u8> ETH0CR_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ETH0CR_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ETH0CR_A::CONST_1)
    }
}
#[doc = "Field `CCUCR` reader - CCU Clock Control in Sleep Mode"]
pub type CCUCR_R = crate::BitReader<CCUCR_A>;
#[doc = "CCU Clock Control in Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCUCR_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<CCUCR_A> for bool {
    #[inline(always)]
    fn from(variant: CCUCR_A) -> Self {
        variant as u8 != 0
    }
}
impl CCUCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCUCR_A {
        match self.bits {
            false => CCUCR_A::CONST_0,
            true => CCUCR_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == CCUCR_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == CCUCR_A::CONST_1
    }
}
#[doc = "Field `CCUCR` writer - CCU Clock Control in Sleep Mode"]
pub type CCUCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLEEPCR_SPEC, CCUCR_A, O>;
impl<'a, const O: u8> CCUCR_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(CCUCR_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(CCUCR_A::CONST_1)
    }
}
#[doc = "Field `WDTCR` reader - WDT Clock Control in Sleep Mode"]
pub type WDTCR_R = crate::BitReader<WDTCR_A>;
#[doc = "WDT Clock Control in Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDTCR_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<WDTCR_A> for bool {
    #[inline(always)]
    fn from(variant: WDTCR_A) -> Self {
        variant as u8 != 0
    }
}
impl WDTCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTCR_A {
        match self.bits {
            false => WDTCR_A::CONST_0,
            true => WDTCR_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == WDTCR_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == WDTCR_A::CONST_1
    }
}
#[doc = "Field `WDTCR` writer - WDT Clock Control in Sleep Mode"]
pub type WDTCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLEEPCR_SPEC, WDTCR_A, O>;
impl<'a, const O: u8> WDTCR_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(WDTCR_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(WDTCR_A::CONST_1)
    }
}
impl R {
    #[doc = "Bit 0 - System Clock Selection Value"]
    #[inline(always)]
    pub fn syssel(&self) -> SYSSEL_R {
        SYSSEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - USB Clock Control in Sleep Mode"]
    #[inline(always)]
    pub fn usbcr(&self) -> USBCR_R {
        USBCR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MMC Clock Control in Sleep Mode"]
    #[inline(always)]
    pub fn mmccr(&self) -> MMCCR_R {
        MMCCR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Ethernet Clock Control in Sleep Mode"]
    #[inline(always)]
    pub fn eth0cr(&self) -> ETH0CR_R {
        ETH0CR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - CCU Clock Control in Sleep Mode"]
    #[inline(always)]
    pub fn ccucr(&self) -> CCUCR_R {
        CCUCR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - WDT Clock Control in Sleep Mode"]
    #[inline(always)]
    pub fn wdtcr(&self) -> WDTCR_R {
        WDTCR_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System Clock Selection Value"]
    #[inline(always)]
    #[must_use]
    pub fn syssel(&mut self) -> SYSSEL_W<0> {
        SYSSEL_W::new(self)
    }
    #[doc = "Bit 16 - USB Clock Control in Sleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn usbcr(&mut self) -> USBCR_W<16> {
        USBCR_W::new(self)
    }
    #[doc = "Bit 17 - MMC Clock Control in Sleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mmccr(&mut self) -> MMCCR_W<17> {
        MMCCR_W::new(self)
    }
    #[doc = "Bit 18 - Ethernet Clock Control in Sleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn eth0cr(&mut self) -> ETH0CR_W<18> {
        ETH0CR_W::new(self)
    }
    #[doc = "Bit 20 - CCU Clock Control in Sleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ccucr(&mut self) -> CCUCR_W<20> {
        CCUCR_W::new(self)
    }
    #[doc = "Bit 21 - WDT Clock Control in Sleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wdtcr(&mut self) -> WDTCR_W<21> {
        WDTCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sleep Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sleepcr](index.html) module"]
pub struct SLEEPCR_SPEC;
impl crate::RegisterSpec for SLEEPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sleepcr::R](R) reader structure"]
impl crate::Readable for SLEEPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sleepcr::W](W) writer structure"]
impl crate::Writable for SLEEPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLEEPCR to value 0"]
impl crate::Resettable for SLEEPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
