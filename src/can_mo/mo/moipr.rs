#[doc = "Register `MOIPR` reader"]
pub struct R(crate::R<MOIPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MOIPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MOIPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MOIPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MOIPR` writer"]
pub struct W(crate::W<MOIPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MOIPR_SPEC>;
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
impl From<crate::W<MOIPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MOIPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Receive Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXINP_A {
    #[doc = "0: Interrupt output line INT_O0 is selected."]
    VALUE1 = 0,
    #[doc = "1: Interrupt output line INT_O1 is selected."]
    VALUE2 = 1,
    #[doc = "14: Interrupt output line INT_O14 is selected."]
    VALUE3 = 14,
    #[doc = "15: Interrupt output line INT_O15 is selected."]
    VALUE4 = 15,
}
impl From<RXINP_A> for u8 {
    #[inline(always)]
    fn from(variant: RXINP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RXINP` reader - Receive Interrupt Node Pointer"]
pub struct RXINP_R(crate::FieldReader<u8, RXINP_A>);
impl RXINP_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXINP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RXINP_A> {
        match self.bits {
            0 => Some(RXINP_A::VALUE1),
            1 => Some(RXINP_A::VALUE2),
            14 => Some(RXINP_A::VALUE3),
            15 => Some(RXINP_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == RXINP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RXINP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == RXINP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == RXINP_A::VALUE4
    }
}
impl core::ops::Deref for RXINP_R {
    type Target = crate::FieldReader<u8, RXINP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXINP` writer - Receive Interrupt Node Pointer"]
pub struct RXINP_W<'a> {
    w: &'a mut W,
}
impl<'a> RXINP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXINP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXINP_A::VALUE1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RXINP_A::VALUE2)
    }
    #[doc = "Interrupt output line INT_O14 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(RXINP_A::VALUE3)
    }
    #[doc = "Interrupt output line INT_O15 is selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(RXINP_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Transmit Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXINP_A {
    #[doc = "0: Interrupt output line INT_O0 is selected."]
    VALUE1 = 0,
    #[doc = "1: Interrupt output line INT_O1 is selected."]
    VALUE2 = 1,
    #[doc = "14: Interrupt output line INT_O14 is selected."]
    VALUE3 = 14,
    #[doc = "15: Interrupt output line INT_O15 is selected."]
    VALUE4 = 15,
}
impl From<TXINP_A> for u8 {
    #[inline(always)]
    fn from(variant: TXINP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TXINP` reader - Transmit Interrupt Node Pointer"]
pub struct TXINP_R(crate::FieldReader<u8, TXINP_A>);
impl TXINP_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXINP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TXINP_A> {
        match self.bits {
            0 => Some(TXINP_A::VALUE1),
            1 => Some(TXINP_A::VALUE2),
            14 => Some(TXINP_A::VALUE3),
            15 => Some(TXINP_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TXINP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TXINP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == TXINP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == TXINP_A::VALUE4
    }
}
impl core::ops::Deref for TXINP_R {
    type Target = crate::FieldReader<u8, TXINP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXINP` writer - Transmit Interrupt Node Pointer"]
pub struct TXINP_W<'a> {
    w: &'a mut W,
}
impl<'a> TXINP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXINP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TXINP_A::VALUE1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TXINP_A::VALUE2)
    }
    #[doc = "Interrupt output line INT_O14 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(TXINP_A::VALUE3)
    }
    #[doc = "Interrupt output line INT_O15 is selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(TXINP_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `MPN` reader - Message Pending Number"]
pub struct MPN_R(crate::FieldReader<u8, u8>);
impl MPN_R {
    pub(crate) fn new(bits: u8) -> Self {
        MPN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPN` writer - Message Pending Number"]
pub struct MPN_W<'a> {
    w: &'a mut W,
}
impl<'a> MPN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `CFCVAL` reader - CAN Frame Counter Value"]
pub struct CFCVAL_R(crate::FieldReader<u16, u16>);
impl CFCVAL_R {
    pub(crate) fn new(bits: u16) -> Self {
        CFCVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFCVAL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFCVAL` writer - CAN Frame Counter Value"]
pub struct CFCVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CFCVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Receive Interrupt Node Pointer"]
    #[inline(always)]
    pub fn rxinp(&self) -> RXINP_R {
        RXINP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Transmit Interrupt Node Pointer"]
    #[inline(always)]
    pub fn txinp(&self) -> TXINP_R {
        TXINP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Message Pending Number"]
    #[inline(always)]
    pub fn mpn(&self) -> MPN_R {
        MPN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - CAN Frame Counter Value"]
    #[inline(always)]
    pub fn cfcval(&self) -> CFCVAL_R {
        CFCVAL_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Receive Interrupt Node Pointer"]
    #[inline(always)]
    pub fn rxinp(&mut self) -> RXINP_W {
        RXINP_W { w: self }
    }
    #[doc = "Bits 4:7 - Transmit Interrupt Node Pointer"]
    #[inline(always)]
    pub fn txinp(&mut self) -> TXINP_W {
        TXINP_W { w: self }
    }
    #[doc = "Bits 8:15 - Message Pending Number"]
    #[inline(always)]
    pub fn mpn(&mut self) -> MPN_W {
        MPN_W { w: self }
    }
    #[doc = "Bits 16:31 - CAN Frame Counter Value"]
    #[inline(always)]
    pub fn cfcval(&mut self) -> CFCVAL_W {
        CFCVAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Message Object Interrupt Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [moipr](index.html) module"]
pub struct MOIPR_SPEC;
impl crate::RegisterSpec for MOIPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [moipr::R](R) reader structure"]
impl crate::Readable for MOIPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [moipr::W](W) writer structure"]
impl crate::Writable for MOIPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MOIPR to value 0"]
impl crate::Resettable for MOIPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
