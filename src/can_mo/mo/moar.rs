#[doc = "Register `MOAR` reader"]
pub type R = crate::R<MOAR_SPEC>;
#[doc = "Register `MOAR` writer"]
pub type W = crate::W<MOAR_SPEC>;
#[doc = "Field `ID` reader - CAN Identifier of Message Object n"]
pub type ID_R = crate::FieldReader<u32>;
#[doc = "Field `ID` writer - CAN Identifier of Message Object n"]
pub type ID_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
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
    pub const fn variant(&self) -> IDE_A {
        match self.bits {
            false => IDE_A::VALUE1,
            true => IDE_A::VALUE2,
        }
    }
    #[doc = "Message object n handles standard frames with 11-bit identifier."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IDE_A::VALUE1
    }
    #[doc = "Message object n handles extended frames with 29-bit identifier."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IDE_A::VALUE2
    }
}
#[doc = "Field `IDE` writer - Identifier Extension Bit of Message Object n"]
pub type IDE_W<'a, REG> = crate::BitWriter<'a, REG, IDE_A>;
impl<'a, REG> IDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Message object n handles standard frames with 11-bit identifier."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(IDE_A::VALUE1)
    }
    #[doc = "Message object n handles extended frames with 29-bit identifier."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(IDE_A::VALUE2)
    }
}
#[doc = "Field `PRI` reader - Priority Class"]
pub type PRI_R = crate::FieldReader<PRI_A>;
#[doc = "Priority Class\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRI_A {
    #[doc = "1: Transmit acceptance filtering is based on the list order. This means that message object n is considered for transmission only if there is no other message object with valid transmit request (MSGVAL &amp; TXEN0 &amp; TXEN1 = 1) somewhere before this object in the list."]
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
impl crate::FieldSpec for PRI_A {
    type Ux = u8;
}
impl PRI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRI_A> {
        match self.bits {
            1 => Some(PRI_A::VALUE2),
            2 => Some(PRI_A::VALUE3),
            3 => Some(PRI_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Transmit acceptance filtering is based on the list order. This means that message object n is considered for transmission only if there is no other message object with valid transmit request (MSGVAL &amp; TXEN0 &amp; TXEN1 = 1) somewhere before this object in the list."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PRI_A::VALUE2
    }
    #[doc = "Transmit acceptance filtering is based on the CAN identifier. This means, message object n is considered for transmission only if there is no other message object with higher priority identifier + IDE + DIR (with respect to CAN arbitration rules) somewhere in the list (see )."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PRI_A::VALUE3
    }
    #[doc = "Transmit acceptance filtering is based on the list order (as PRI = 01B)."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PRI_A::VALUE4
    }
}
#[doc = "Field `PRI` writer - Priority Class"]
pub type PRI_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PRI_A>;
impl<'a, REG> PRI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Transmit acceptance filtering is based on the list order. This means that message object n is considered for transmission only if there is no other message object with valid transmit request (MSGVAL &amp; TXEN0 &amp; TXEN1 = 1) somewhere before this object in the list."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PRI_A::VALUE2)
    }
    #[doc = "Transmit acceptance filtering is based on the CAN identifier. This means, message object n is considered for transmission only if there is no other message object with higher priority identifier + IDE + DIR (with respect to CAN arbitration rules) somewhere in the list (see )."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(PRI_A::VALUE3)
    }
    #[doc = "Transmit acceptance filtering is based on the list order (as PRI = 01B)."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
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
    pub fn id(&mut self) -> ID_W<MOAR_SPEC> {
        ID_W::new(self, 0)
    }
    #[doc = "Bit 29 - Identifier Extension Bit of Message Object n"]
    #[inline(always)]
    #[must_use]
    pub fn ide(&mut self) -> IDE_W<MOAR_SPEC> {
        IDE_W::new(self, 29)
    }
    #[doc = "Bits 30:31 - Priority Class"]
    #[inline(always)]
    #[must_use]
    pub fn pri(&mut self) -> PRI_W<MOAR_SPEC> {
        PRI_W::new(self, 30)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Message Object Arbitration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`moar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`moar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MOAR_SPEC;
impl crate::RegisterSpec for MOAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`moar::R`](R) reader structure"]
impl crate::Readable for MOAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`moar::W`](W) writer structure"]
impl crate::Writable for MOAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MOAR to value 0"]
impl crate::Resettable for MOAR_SPEC {
    const RESET_VALUE: u32 = 0;
}
