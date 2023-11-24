#[doc = "Register `ARBPR` reader"]
pub type R = crate::R<ARBPR_SPEC>;
#[doc = "Register `ARBPR` writer"]
pub type W = crate::W<ARBPR_SPEC>;
#[doc = "Field `PRIO0` reader - Priority of Request Source x"]
pub type PRIO0_R = crate::FieldReader<PRIO0_A>;
#[doc = "Priority of Request Source x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRIO0_A {
    #[doc = "0: Lowest priority is selected."]
    VALUE1 = 0,
    #[doc = "3: Highest priority is selected."]
    VALUE2 = 3,
}
impl From<PRIO0_A> for u8 {
    #[inline(always)]
    fn from(variant: PRIO0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRIO0_A {
    type Ux = u8;
}
impl PRIO0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRIO0_A> {
        match self.bits {
            0 => Some(PRIO0_A::VALUE1),
            3 => Some(PRIO0_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Lowest priority is selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PRIO0_A::VALUE1
    }
    #[doc = "Highest priority is selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PRIO0_A::VALUE2
    }
}
#[doc = "Field `PRIO0` writer - Priority of Request Source x"]
pub type PRIO0_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PRIO0_A>;
impl<'a, REG> PRIO0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Lowest priority is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIO0_A::VALUE1)
    }
    #[doc = "Highest priority is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PRIO0_A::VALUE2)
    }
}
#[doc = "Field `CSM0` reader - Conversion Start Mode of Request Source x"]
pub type CSM0_R = crate::BitReader<CSM0_A>;
#[doc = "Conversion Start Mode of Request Source x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSM0_A {
    #[doc = "0: Wait-for-start mode"]
    VALUE1 = 0,
    #[doc = "1: Cancel-inject-repeat mode, i.e. this source can cancel conversion of other sources."]
    VALUE2 = 1,
}
impl From<CSM0_A> for bool {
    #[inline(always)]
    fn from(variant: CSM0_A) -> Self {
        variant as u8 != 0
    }
}
impl CSM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSM0_A {
        match self.bits {
            false => CSM0_A::VALUE1,
            true => CSM0_A::VALUE2,
        }
    }
    #[doc = "Wait-for-start mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CSM0_A::VALUE1
    }
    #[doc = "Cancel-inject-repeat mode, i.e. this source can cancel conversion of other sources."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CSM0_A::VALUE2
    }
}
#[doc = "Field `CSM0` writer - Conversion Start Mode of Request Source x"]
pub type CSM0_W<'a, REG> = crate::BitWriter<'a, REG, CSM0_A>;
impl<'a, REG> CSM0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wait-for-start mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CSM0_A::VALUE1)
    }
    #[doc = "Cancel-inject-repeat mode, i.e. this source can cancel conversion of other sources."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CSM0_A::VALUE2)
    }
}
#[doc = "Field `PRIO1` reader - Priority of Request Source x"]
pub type PRIO1_R = crate::FieldReader<PRIO1_A>;
#[doc = "Priority of Request Source x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRIO1_A {
    #[doc = "0: Lowest priority is selected."]
    VALUE1 = 0,
    #[doc = "3: Highest priority is selected."]
    VALUE2 = 3,
}
impl From<PRIO1_A> for u8 {
    #[inline(always)]
    fn from(variant: PRIO1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRIO1_A {
    type Ux = u8;
}
impl PRIO1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRIO1_A> {
        match self.bits {
            0 => Some(PRIO1_A::VALUE1),
            3 => Some(PRIO1_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Lowest priority is selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PRIO1_A::VALUE1
    }
    #[doc = "Highest priority is selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PRIO1_A::VALUE2
    }
}
#[doc = "Field `PRIO1` writer - Priority of Request Source x"]
pub type PRIO1_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PRIO1_A>;
impl<'a, REG> PRIO1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Lowest priority is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIO1_A::VALUE1)
    }
    #[doc = "Highest priority is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PRIO1_A::VALUE2)
    }
}
#[doc = "Field `CSM1` reader - Conversion Start Mode of Request Source x"]
pub type CSM1_R = crate::BitReader<CSM1_A>;
#[doc = "Conversion Start Mode of Request Source x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSM1_A {
    #[doc = "0: Wait-for-start mode"]
    VALUE1 = 0,
    #[doc = "1: Cancel-inject-repeat mode, i.e. this source can cancel conversion of other sources."]
    VALUE2 = 1,
}
impl From<CSM1_A> for bool {
    #[inline(always)]
    fn from(variant: CSM1_A) -> Self {
        variant as u8 != 0
    }
}
impl CSM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSM1_A {
        match self.bits {
            false => CSM1_A::VALUE1,
            true => CSM1_A::VALUE2,
        }
    }
    #[doc = "Wait-for-start mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CSM1_A::VALUE1
    }
    #[doc = "Cancel-inject-repeat mode, i.e. this source can cancel conversion of other sources."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CSM1_A::VALUE2
    }
}
#[doc = "Field `CSM1` writer - Conversion Start Mode of Request Source x"]
pub type CSM1_W<'a, REG> = crate::BitWriter<'a, REG, CSM1_A>;
impl<'a, REG> CSM1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wait-for-start mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CSM1_A::VALUE1)
    }
    #[doc = "Cancel-inject-repeat mode, i.e. this source can cancel conversion of other sources."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CSM1_A::VALUE2)
    }
}
#[doc = "Field `PRIO2` reader - Priority of Request Source x"]
pub type PRIO2_R = crate::FieldReader<PRIO2_A>;
#[doc = "Priority of Request Source x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRIO2_A {
    #[doc = "0: Lowest priority is selected."]
    VALUE1 = 0,
    #[doc = "3: Highest priority is selected."]
    VALUE2 = 3,
}
impl From<PRIO2_A> for u8 {
    #[inline(always)]
    fn from(variant: PRIO2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRIO2_A {
    type Ux = u8;
}
impl PRIO2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRIO2_A> {
        match self.bits {
            0 => Some(PRIO2_A::VALUE1),
            3 => Some(PRIO2_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Lowest priority is selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PRIO2_A::VALUE1
    }
    #[doc = "Highest priority is selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PRIO2_A::VALUE2
    }
}
#[doc = "Field `PRIO2` writer - Priority of Request Source x"]
pub type PRIO2_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PRIO2_A>;
impl<'a, REG> PRIO2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Lowest priority is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIO2_A::VALUE1)
    }
    #[doc = "Highest priority is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PRIO2_A::VALUE2)
    }
}
#[doc = "Field `CSM2` reader - Conversion Start Mode of Request Source x"]
pub type CSM2_R = crate::BitReader<CSM2_A>;
#[doc = "Conversion Start Mode of Request Source x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSM2_A {
    #[doc = "0: Wait-for-start mode"]
    VALUE1 = 0,
    #[doc = "1: Cancel-inject-repeat mode, i.e. this source can cancel conversion of other sources."]
    VALUE2 = 1,
}
impl From<CSM2_A> for bool {
    #[inline(always)]
    fn from(variant: CSM2_A) -> Self {
        variant as u8 != 0
    }
}
impl CSM2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSM2_A {
        match self.bits {
            false => CSM2_A::VALUE1,
            true => CSM2_A::VALUE2,
        }
    }
    #[doc = "Wait-for-start mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CSM2_A::VALUE1
    }
    #[doc = "Cancel-inject-repeat mode, i.e. this source can cancel conversion of other sources."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CSM2_A::VALUE2
    }
}
#[doc = "Field `CSM2` writer - Conversion Start Mode of Request Source x"]
pub type CSM2_W<'a, REG> = crate::BitWriter<'a, REG, CSM2_A>;
impl<'a, REG> CSM2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wait-for-start mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CSM2_A::VALUE1)
    }
    #[doc = "Cancel-inject-repeat mode, i.e. this source can cancel conversion of other sources."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CSM2_A::VALUE2)
    }
}
#[doc = "Field `ASEN0` reader - Arbitration Slot 0 Enable"]
pub type ASEN0_R = crate::BitReader<ASEN0_A>;
#[doc = "Arbitration Slot 0 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASEN0_A {
    #[doc = "0: The corresponding arbitration slot is disabled and considered as empty. Pending conversion requests from the associated request source are disregarded."]
    VALUE1 = 0,
    #[doc = "1: The corresponding arbitration slot is enabled. Pending conversion requests from the associated request source are arbitrated."]
    VALUE2 = 1,
}
impl From<ASEN0_A> for bool {
    #[inline(always)]
    fn from(variant: ASEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl ASEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ASEN0_A {
        match self.bits {
            false => ASEN0_A::VALUE1,
            true => ASEN0_A::VALUE2,
        }
    }
    #[doc = "The corresponding arbitration slot is disabled and considered as empty. Pending conversion requests from the associated request source are disregarded."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ASEN0_A::VALUE1
    }
    #[doc = "The corresponding arbitration slot is enabled. Pending conversion requests from the associated request source are arbitrated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ASEN0_A::VALUE2
    }
}
#[doc = "Field `ASEN0` writer - Arbitration Slot 0 Enable"]
pub type ASEN0_W<'a, REG> = crate::BitWriter<'a, REG, ASEN0_A>;
impl<'a, REG> ASEN0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The corresponding arbitration slot is disabled and considered as empty. Pending conversion requests from the associated request source are disregarded."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ASEN0_A::VALUE1)
    }
    #[doc = "The corresponding arbitration slot is enabled. Pending conversion requests from the associated request source are arbitrated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ASEN0_A::VALUE2)
    }
}
#[doc = "Field `ASEN1` reader - Arbitration Slot 1 Enable"]
pub type ASEN1_R = crate::BitReader<ASEN1_A>;
#[doc = "Arbitration Slot 1 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASEN1_A {
    #[doc = "0: The corresponding arbitration slot is disabled and considered as empty. Pending conversion requests from the associated request source are disregarded."]
    VALUE1 = 0,
    #[doc = "1: The corresponding arbitration slot is enabled. Pending conversion requests from the associated request source are arbitrated."]
    VALUE2 = 1,
}
impl From<ASEN1_A> for bool {
    #[inline(always)]
    fn from(variant: ASEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl ASEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ASEN1_A {
        match self.bits {
            false => ASEN1_A::VALUE1,
            true => ASEN1_A::VALUE2,
        }
    }
    #[doc = "The corresponding arbitration slot is disabled and considered as empty. Pending conversion requests from the associated request source are disregarded."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ASEN1_A::VALUE1
    }
    #[doc = "The corresponding arbitration slot is enabled. Pending conversion requests from the associated request source are arbitrated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ASEN1_A::VALUE2
    }
}
#[doc = "Field `ASEN1` writer - Arbitration Slot 1 Enable"]
pub type ASEN1_W<'a, REG> = crate::BitWriter<'a, REG, ASEN1_A>;
impl<'a, REG> ASEN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The corresponding arbitration slot is disabled and considered as empty. Pending conversion requests from the associated request source are disregarded."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ASEN1_A::VALUE1)
    }
    #[doc = "The corresponding arbitration slot is enabled. Pending conversion requests from the associated request source are arbitrated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ASEN1_A::VALUE2)
    }
}
#[doc = "Field `ASEN2` reader - Arbitration Slot 2 Enable"]
pub type ASEN2_R = crate::BitReader<ASEN2_A>;
#[doc = "Arbitration Slot 2 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASEN2_A {
    #[doc = "0: The corresponding arbitration slot is disabled and considered as empty. Pending conversion requests from the associated request source are disregarded."]
    VALUE1 = 0,
    #[doc = "1: The corresponding arbitration slot is enabled. Pending conversion requests from the associated request source are arbitrated."]
    VALUE2 = 1,
}
impl From<ASEN2_A> for bool {
    #[inline(always)]
    fn from(variant: ASEN2_A) -> Self {
        variant as u8 != 0
    }
}
impl ASEN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ASEN2_A {
        match self.bits {
            false => ASEN2_A::VALUE1,
            true => ASEN2_A::VALUE2,
        }
    }
    #[doc = "The corresponding arbitration slot is disabled and considered as empty. Pending conversion requests from the associated request source are disregarded."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ASEN2_A::VALUE1
    }
    #[doc = "The corresponding arbitration slot is enabled. Pending conversion requests from the associated request source are arbitrated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ASEN2_A::VALUE2
    }
}
#[doc = "Field `ASEN2` writer - Arbitration Slot 2 Enable"]
pub type ASEN2_W<'a, REG> = crate::BitWriter<'a, REG, ASEN2_A>;
impl<'a, REG> ASEN2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The corresponding arbitration slot is disabled and considered as empty. Pending conversion requests from the associated request source are disregarded."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ASEN2_A::VALUE1)
    }
    #[doc = "The corresponding arbitration slot is enabled. Pending conversion requests from the associated request source are arbitrated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ASEN2_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:1 - Priority of Request Source x"]
    #[inline(always)]
    pub fn prio0(&self) -> PRIO0_R {
        PRIO0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Conversion Start Mode of Request Source x"]
    #[inline(always)]
    pub fn csm0(&self) -> CSM0_R {
        CSM0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Priority of Request Source x"]
    #[inline(always)]
    pub fn prio1(&self) -> PRIO1_R {
        PRIO1_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Conversion Start Mode of Request Source x"]
    #[inline(always)]
    pub fn csm1(&self) -> CSM1_R {
        CSM1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Priority of Request Source x"]
    #[inline(always)]
    pub fn prio2(&self) -> PRIO2_R {
        PRIO2_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - Conversion Start Mode of Request Source x"]
    #[inline(always)]
    pub fn csm2(&self) -> CSM2_R {
        CSM2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 24 - Arbitration Slot 0 Enable"]
    #[inline(always)]
    pub fn asen0(&self) -> ASEN0_R {
        ASEN0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Arbitration Slot 1 Enable"]
    #[inline(always)]
    pub fn asen1(&self) -> ASEN1_R {
        ASEN1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Arbitration Slot 2 Enable"]
    #[inline(always)]
    pub fn asen2(&self) -> ASEN2_R {
        ASEN2_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Priority of Request Source x"]
    #[inline(always)]
    #[must_use]
    pub fn prio0(&mut self) -> PRIO0_W<ARBPR_SPEC> {
        PRIO0_W::new(self, 0)
    }
    #[doc = "Bit 3 - Conversion Start Mode of Request Source x"]
    #[inline(always)]
    #[must_use]
    pub fn csm0(&mut self) -> CSM0_W<ARBPR_SPEC> {
        CSM0_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Priority of Request Source x"]
    #[inline(always)]
    #[must_use]
    pub fn prio1(&mut self) -> PRIO1_W<ARBPR_SPEC> {
        PRIO1_W::new(self, 4)
    }
    #[doc = "Bit 7 - Conversion Start Mode of Request Source x"]
    #[inline(always)]
    #[must_use]
    pub fn csm1(&mut self) -> CSM1_W<ARBPR_SPEC> {
        CSM1_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - Priority of Request Source x"]
    #[inline(always)]
    #[must_use]
    pub fn prio2(&mut self) -> PRIO2_W<ARBPR_SPEC> {
        PRIO2_W::new(self, 8)
    }
    #[doc = "Bit 11 - Conversion Start Mode of Request Source x"]
    #[inline(always)]
    #[must_use]
    pub fn csm2(&mut self) -> CSM2_W<ARBPR_SPEC> {
        CSM2_W::new(self, 11)
    }
    #[doc = "Bit 24 - Arbitration Slot 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn asen0(&mut self) -> ASEN0_W<ARBPR_SPEC> {
        ASEN0_W::new(self, 24)
    }
    #[doc = "Bit 25 - Arbitration Slot 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn asen1(&mut self) -> ASEN1_W<ARBPR_SPEC> {
        ASEN1_W::new(self, 25)
    }
    #[doc = "Bit 26 - Arbitration Slot 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn asen2(&mut self) -> ASEN2_W<ARBPR_SPEC> {
        ASEN2_W::new(self, 26)
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
#[doc = "Arbitration Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arbpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arbpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARBPR_SPEC;
impl crate::RegisterSpec for ARBPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arbpr::R`](R) reader structure"]
impl crate::Readable for ARBPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`arbpr::W`](W) writer structure"]
impl crate::Writable for ARBPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ARBPR to value 0"]
impl crate::Resettable for ARBPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
