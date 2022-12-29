#[doc = "Register `MOAR` reader"]
pub struct R(crate::R<MOAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MOAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MOAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MOAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MOAR` writer"]
pub struct W(crate::W<MOAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MOAR_SPEC>;
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
impl From<crate::W<MOAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MOAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ID` reader - CAN Identifier of Message Object n"]
pub type ID_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ID` writer - CAN Identifier of Message Object n"]
pub type ID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MOAR_SPEC, u32, u32, 29, O>;
#[doc = "Field `IDE` reader - Identifier Extension Bit of Message Object n"]
pub type IDE_R = crate::BitReader<IDE_A>;
#[doc = "Identifier Extension Bit of Message Object n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDE_A {
    #[doc = "0: Message object n handles standard frames with 11-bit identifier."]
    VALUE1 = 0,
    #[doc = "1: Message object n handles extended frames with 29-bit identifier."]
    VALUE2 = 1,
}
impl From<IDE_A> for bool {
    #[inline(always)]
    fn from(variant: IDE_A) -> Self {
        variant as u8 != 0
    }
}
impl IDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDE_A {
        match self.bits {
            false => IDE_A::VALUE1,
            true => IDE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IDE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IDE_A::VALUE2
    }
}
#[doc = "Field `IDE` writer - Identifier Extension Bit of Message Object n"]
pub type IDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MOAR_SPEC, IDE_A, O>;
impl<'a, const O: u8> IDE_W<'a, O> {
    #[doc = "Message object n handles standard frames with 11-bit identifier."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(IDE_A::VALUE1)
    }
    #[doc = "Message object n handles extended frames with 29-bit identifier."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(IDE_A::VALUE2)
    }
}
#[doc = "Field `PRI` reader - Priority Class"]
pub type PRI_R = crate::FieldReader<u8, PRI_A>;
#[doc = "Priority Class\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRI_A {
    #[doc = "1: Transmit acceptance filtering is based on the list order. This means that message object n is considered for transmission only if there is no other message object with valid transmit request (MSGVAL & TXEN0 & TXEN1 = 1) somewhere before this object in the list."]
    VALUE2 = 1,
    #[doc = "2: Transmit acceptance filtering is based on the CAN identifier. This means, message object n is considered for transmission only if there is no other message object with higher priority identifier + IDE + DIR (with respect to CAN arbitration rules) somewhere in the list (see )."]
    VALUE3 = 2,
    #[doc = "3: Transmit acceptance filtering is based on the list order (as PRI = 01B)."]
    VALUE4 = 3,
}
impl From<PRI_A> for u8 {
    #[inline(always)]
    fn from(variant: PRI_A) -> Self {
        variant as _
    }
}
impl PRI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRI_A> {
        match self.bits {
            1 => Some(PRI_A::VALUE2),
            2 => Some(PRI_A::VALUE3),
            3 => Some(PRI_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PRI_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PRI_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PRI_A::VALUE4
    }
}
#[doc = "Field `PRI` writer - Priority Class"]
pub type PRI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MOAR_SPEC, u8, PRI_A, 2, O>;
impl<'a, const O: u8> PRI_W<'a, O> {
    #[doc = "Transmit acceptance filtering is based on the list order. This means that message object n is considered for transmission only if there is no other message object with valid transmit request (MSGVAL & TXEN0 & TXEN1 = 1) somewhere before this object in the list."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRI_A::VALUE2)
    }
    #[doc = "Transmit acceptance filtering is based on the CAN identifier. This means, message object n is considered for transmission only if there is no other message object with higher priority identifier + IDE + DIR (with respect to CAN arbitration rules) somewhere in the list (see )."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PRI_A::VALUE3)
    }
    #[doc = "Transmit acceptance filtering is based on the list order (as PRI = 01B)."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PRI_A::VALUE4)
    }
}
impl R {
    #[doc = "Bits 0:28 - CAN Identifier of Message Object n"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits & 0x1fff_ffff)
    }
    #[doc = "Bit 29 - Identifier Extension Bit of Message Object n"]
    #[inline(always)]
    pub fn ide(&self) -> IDE_R {
        IDE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Priority Class"]
    #[inline(always)]
    pub fn pri(&self) -> PRI_R {
        PRI_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:28 - CAN Identifier of Message Object n"]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> ID_W<0> {
        ID_W::new(self)
    }
    #[doc = "Bit 29 - Identifier Extension Bit of Message Object n"]
    #[inline(always)]
    #[must_use]
    pub fn ide(&mut self) -> IDE_W<29> {
        IDE_W::new(self)
    }
    #[doc = "Bits 30:31 - Priority Class"]
    #[inline(always)]
    #[must_use]
    pub fn pri(&mut self) -> PRI_W<30> {
        PRI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Message Object Arbitration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [moar](index.html) module"]
pub struct MOAR_SPEC;
impl crate::RegisterSpec for MOAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [moar::R](R) reader structure"]
impl crate::Readable for MOAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [moar::W](W) writer structure"]
impl crate::Writable for MOAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MOAR to value 0"]
impl crate::Resettable for MOAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
