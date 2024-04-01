#[doc = "Register `ARBPR` reader"]
pub type R = crate::R<ArbprSpec>;
#[doc = "Register `ARBPR` writer"]
pub type W = crate::W<ArbprSpec>;
#[doc = "Priority of Request Source x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prio0 {
    #[doc = "0: Lowest priority is selected."]
    Value1 = 0,
    #[doc = "3: Highest priority is selected."]
    Value2 = 3,
}
impl From<Prio0> for u8 {
    #[inline(always)]
    fn from(variant: Prio0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prio0 {
    type Ux = u8;
}
impl crate::IsEnum for Prio0 {}
#[doc = "Field `PRIO0` reader - Priority of Request Source x"]
pub type Prio0R = crate::FieldReader<Prio0>;
impl Prio0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prio0> {
        match self.bits {
            0 => Some(Prio0::Value1),
            3 => Some(Prio0::Value2),
            _ => None,
        }
    }
    #[doc = "Lowest priority is selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Prio0::Value1
    }
    #[doc = "Highest priority is selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Prio0::Value2
    }
}
#[doc = "Field `PRIO0` writer - Priority of Request Source x"]
pub type Prio0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Prio0>;
impl<'a, REG> Prio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Lowest priority is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Prio0::Value1)
    }
    #[doc = "Highest priority is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Prio0::Value2)
    }
}
#[doc = "Conversion Start Mode of Request Source x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Csm0 {
    #[doc = "0: Wait-for-start mode"]
    Value1 = 0,
    #[doc = "1: Cancel-inject-repeat mode, i.e. this source can cancel conversion of other sources."]
    Value2 = 1,
}
impl From<Csm0> for bool {
    #[inline(always)]
    fn from(variant: Csm0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSM0` reader - Conversion Start Mode of Request Source x"]
pub type Csm0R = crate::BitReader<Csm0>;
impl Csm0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Csm0 {
        match self.bits {
            false => Csm0::Value1,
            true => Csm0::Value2,
        }
    }
    #[doc = "Wait-for-start mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Csm0::Value1
    }
    #[doc = "Cancel-inject-repeat mode, i.e. this source can cancel conversion of other sources."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Csm0::Value2
    }
}
#[doc = "Field `CSM0` writer - Conversion Start Mode of Request Source x"]
pub type Csm0W<'a, REG> = crate::BitWriter<'a, REG, Csm0>;
impl<'a, REG> Csm0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wait-for-start mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Csm0::Value1)
    }
    #[doc = "Cancel-inject-repeat mode, i.e. this source can cancel conversion of other sources."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Csm0::Value2)
    }
}
#[doc = "Priority of Request Source x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prio1 {
    #[doc = "0: Lowest priority is selected."]
    Value1 = 0,
    #[doc = "3: Highest priority is selected."]
    Value2 = 3,
}
impl From<Prio1> for u8 {
    #[inline(always)]
    fn from(variant: Prio1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prio1 {
    type Ux = u8;
}
impl crate::IsEnum for Prio1 {}
#[doc = "Field `PRIO1` reader - Priority of Request Source x"]
pub type Prio1R = crate::FieldReader<Prio1>;
impl Prio1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prio1> {
        match self.bits {
            0 => Some(Prio1::Value1),
            3 => Some(Prio1::Value2),
            _ => None,
        }
    }
    #[doc = "Lowest priority is selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Prio1::Value1
    }
    #[doc = "Highest priority is selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Prio1::Value2
    }
}
#[doc = "Field `PRIO1` writer - Priority of Request Source x"]
pub type Prio1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Prio1>;
impl<'a, REG> Prio1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Lowest priority is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Prio1::Value1)
    }
    #[doc = "Highest priority is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Prio1::Value2)
    }
}
#[doc = "Conversion Start Mode of Request Source x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Csm1 {
    #[doc = "0: Wait-for-start mode"]
    Value1 = 0,
    #[doc = "1: Cancel-inject-repeat mode, i.e. this source can cancel conversion of other sources."]
    Value2 = 1,
}
impl From<Csm1> for bool {
    #[inline(always)]
    fn from(variant: Csm1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSM1` reader - Conversion Start Mode of Request Source x"]
pub type Csm1R = crate::BitReader<Csm1>;
impl Csm1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Csm1 {
        match self.bits {
            false => Csm1::Value1,
            true => Csm1::Value2,
        }
    }
    #[doc = "Wait-for-start mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Csm1::Value1
    }
    #[doc = "Cancel-inject-repeat mode, i.e. this source can cancel conversion of other sources."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Csm1::Value2
    }
}
#[doc = "Field `CSM1` writer - Conversion Start Mode of Request Source x"]
pub type Csm1W<'a, REG> = crate::BitWriter<'a, REG, Csm1>;
impl<'a, REG> Csm1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wait-for-start mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Csm1::Value1)
    }
    #[doc = "Cancel-inject-repeat mode, i.e. this source can cancel conversion of other sources."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Csm1::Value2)
    }
}
#[doc = "Priority of Request Source x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prio2 {
    #[doc = "0: Lowest priority is selected."]
    Value1 = 0,
    #[doc = "3: Highest priority is selected."]
    Value2 = 3,
}
impl From<Prio2> for u8 {
    #[inline(always)]
    fn from(variant: Prio2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prio2 {
    type Ux = u8;
}
impl crate::IsEnum for Prio2 {}
#[doc = "Field `PRIO2` reader - Priority of Request Source x"]
pub type Prio2R = crate::FieldReader<Prio2>;
impl Prio2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prio2> {
        match self.bits {
            0 => Some(Prio2::Value1),
            3 => Some(Prio2::Value2),
            _ => None,
        }
    }
    #[doc = "Lowest priority is selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Prio2::Value1
    }
    #[doc = "Highest priority is selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Prio2::Value2
    }
}
#[doc = "Field `PRIO2` writer - Priority of Request Source x"]
pub type Prio2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Prio2>;
impl<'a, REG> Prio2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Lowest priority is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Prio2::Value1)
    }
    #[doc = "Highest priority is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Prio2::Value2)
    }
}
#[doc = "Conversion Start Mode of Request Source x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Csm2 {
    #[doc = "0: Wait-for-start mode"]
    Value1 = 0,
    #[doc = "1: Cancel-inject-repeat mode, i.e. this source can cancel conversion of other sources."]
    Value2 = 1,
}
impl From<Csm2> for bool {
    #[inline(always)]
    fn from(variant: Csm2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSM2` reader - Conversion Start Mode of Request Source x"]
pub type Csm2R = crate::BitReader<Csm2>;
impl Csm2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Csm2 {
        match self.bits {
            false => Csm2::Value1,
            true => Csm2::Value2,
        }
    }
    #[doc = "Wait-for-start mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Csm2::Value1
    }
    #[doc = "Cancel-inject-repeat mode, i.e. this source can cancel conversion of other sources."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Csm2::Value2
    }
}
#[doc = "Field `CSM2` writer - Conversion Start Mode of Request Source x"]
pub type Csm2W<'a, REG> = crate::BitWriter<'a, REG, Csm2>;
impl<'a, REG> Csm2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wait-for-start mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Csm2::Value1)
    }
    #[doc = "Cancel-inject-repeat mode, i.e. this source can cancel conversion of other sources."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Csm2::Value2)
    }
}
#[doc = "Arbitration Slot 0 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asen0 {
    #[doc = "0: The corresponding arbitration slot is disabled and considered as empty. Pending conversion requests from the associated request source are disregarded."]
    Value1 = 0,
    #[doc = "1: The corresponding arbitration slot is enabled. Pending conversion requests from the associated request source are arbitrated."]
    Value2 = 1,
}
impl From<Asen0> for bool {
    #[inline(always)]
    fn from(variant: Asen0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASEN0` reader - Arbitration Slot 0 Enable"]
pub type Asen0R = crate::BitReader<Asen0>;
impl Asen0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Asen0 {
        match self.bits {
            false => Asen0::Value1,
            true => Asen0::Value2,
        }
    }
    #[doc = "The corresponding arbitration slot is disabled and considered as empty. Pending conversion requests from the associated request source are disregarded."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Asen0::Value1
    }
    #[doc = "The corresponding arbitration slot is enabled. Pending conversion requests from the associated request source are arbitrated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Asen0::Value2
    }
}
#[doc = "Field `ASEN0` writer - Arbitration Slot 0 Enable"]
pub type Asen0W<'a, REG> = crate::BitWriter<'a, REG, Asen0>;
impl<'a, REG> Asen0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The corresponding arbitration slot is disabled and considered as empty. Pending conversion requests from the associated request source are disregarded."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Asen0::Value1)
    }
    #[doc = "The corresponding arbitration slot is enabled. Pending conversion requests from the associated request source are arbitrated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Asen0::Value2)
    }
}
#[doc = "Arbitration Slot 1 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asen1 {
    #[doc = "0: The corresponding arbitration slot is disabled and considered as empty. Pending conversion requests from the associated request source are disregarded."]
    Value1 = 0,
    #[doc = "1: The corresponding arbitration slot is enabled. Pending conversion requests from the associated request source are arbitrated."]
    Value2 = 1,
}
impl From<Asen1> for bool {
    #[inline(always)]
    fn from(variant: Asen1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASEN1` reader - Arbitration Slot 1 Enable"]
pub type Asen1R = crate::BitReader<Asen1>;
impl Asen1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Asen1 {
        match self.bits {
            false => Asen1::Value1,
            true => Asen1::Value2,
        }
    }
    #[doc = "The corresponding arbitration slot is disabled and considered as empty. Pending conversion requests from the associated request source are disregarded."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Asen1::Value1
    }
    #[doc = "The corresponding arbitration slot is enabled. Pending conversion requests from the associated request source are arbitrated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Asen1::Value2
    }
}
#[doc = "Field `ASEN1` writer - Arbitration Slot 1 Enable"]
pub type Asen1W<'a, REG> = crate::BitWriter<'a, REG, Asen1>;
impl<'a, REG> Asen1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The corresponding arbitration slot is disabled and considered as empty. Pending conversion requests from the associated request source are disregarded."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Asen1::Value1)
    }
    #[doc = "The corresponding arbitration slot is enabled. Pending conversion requests from the associated request source are arbitrated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Asen1::Value2)
    }
}
#[doc = "Arbitration Slot 2 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asen2 {
    #[doc = "0: The corresponding arbitration slot is disabled and considered as empty. Pending conversion requests from the associated request source are disregarded."]
    Value1 = 0,
    #[doc = "1: The corresponding arbitration slot is enabled. Pending conversion requests from the associated request source are arbitrated."]
    Value2 = 1,
}
impl From<Asen2> for bool {
    #[inline(always)]
    fn from(variant: Asen2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASEN2` reader - Arbitration Slot 2 Enable"]
pub type Asen2R = crate::BitReader<Asen2>;
impl Asen2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Asen2 {
        match self.bits {
            false => Asen2::Value1,
            true => Asen2::Value2,
        }
    }
    #[doc = "The corresponding arbitration slot is disabled and considered as empty. Pending conversion requests from the associated request source are disregarded."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Asen2::Value1
    }
    #[doc = "The corresponding arbitration slot is enabled. Pending conversion requests from the associated request source are arbitrated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Asen2::Value2
    }
}
#[doc = "Field `ASEN2` writer - Arbitration Slot 2 Enable"]
pub type Asen2W<'a, REG> = crate::BitWriter<'a, REG, Asen2>;
impl<'a, REG> Asen2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The corresponding arbitration slot is disabled and considered as empty. Pending conversion requests from the associated request source are disregarded."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Asen2::Value1)
    }
    #[doc = "The corresponding arbitration slot is enabled. Pending conversion requests from the associated request source are arbitrated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Asen2::Value2)
    }
}
impl R {
    #[doc = "Bits 0:1 - Priority of Request Source x"]
    #[inline(always)]
    pub fn prio0(&self) -> Prio0R {
        Prio0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Conversion Start Mode of Request Source x"]
    #[inline(always)]
    pub fn csm0(&self) -> Csm0R {
        Csm0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Priority of Request Source x"]
    #[inline(always)]
    pub fn prio1(&self) -> Prio1R {
        Prio1R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Conversion Start Mode of Request Source x"]
    #[inline(always)]
    pub fn csm1(&self) -> Csm1R {
        Csm1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Priority of Request Source x"]
    #[inline(always)]
    pub fn prio2(&self) -> Prio2R {
        Prio2R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - Conversion Start Mode of Request Source x"]
    #[inline(always)]
    pub fn csm2(&self) -> Csm2R {
        Csm2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 24 - Arbitration Slot 0 Enable"]
    #[inline(always)]
    pub fn asen0(&self) -> Asen0R {
        Asen0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Arbitration Slot 1 Enable"]
    #[inline(always)]
    pub fn asen1(&self) -> Asen1R {
        Asen1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Arbitration Slot 2 Enable"]
    #[inline(always)]
    pub fn asen2(&self) -> Asen2R {
        Asen2R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Priority of Request Source x"]
    #[inline(always)]
    #[must_use]
    pub fn prio0(&mut self) -> Prio0W<ArbprSpec> {
        Prio0W::new(self, 0)
    }
    #[doc = "Bit 3 - Conversion Start Mode of Request Source x"]
    #[inline(always)]
    #[must_use]
    pub fn csm0(&mut self) -> Csm0W<ArbprSpec> {
        Csm0W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Priority of Request Source x"]
    #[inline(always)]
    #[must_use]
    pub fn prio1(&mut self) -> Prio1W<ArbprSpec> {
        Prio1W::new(self, 4)
    }
    #[doc = "Bit 7 - Conversion Start Mode of Request Source x"]
    #[inline(always)]
    #[must_use]
    pub fn csm1(&mut self) -> Csm1W<ArbprSpec> {
        Csm1W::new(self, 7)
    }
    #[doc = "Bits 8:9 - Priority of Request Source x"]
    #[inline(always)]
    #[must_use]
    pub fn prio2(&mut self) -> Prio2W<ArbprSpec> {
        Prio2W::new(self, 8)
    }
    #[doc = "Bit 11 - Conversion Start Mode of Request Source x"]
    #[inline(always)]
    #[must_use]
    pub fn csm2(&mut self) -> Csm2W<ArbprSpec> {
        Csm2W::new(self, 11)
    }
    #[doc = "Bit 24 - Arbitration Slot 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn asen0(&mut self) -> Asen0W<ArbprSpec> {
        Asen0W::new(self, 24)
    }
    #[doc = "Bit 25 - Arbitration Slot 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn asen1(&mut self) -> Asen1W<ArbprSpec> {
        Asen1W::new(self, 25)
    }
    #[doc = "Bit 26 - Arbitration Slot 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn asen2(&mut self) -> Asen2W<ArbprSpec> {
        Asen2W::new(self, 26)
    }
}
#[doc = "Arbitration Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arbpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arbpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ArbprSpec;
impl crate::RegisterSpec for ArbprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arbpr::R`](R) reader structure"]
impl crate::Readable for ArbprSpec {}
#[doc = "`write(|w| ..)` method takes [`arbpr::W`](W) writer structure"]
impl crate::Writable for ArbprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARBPR to value 0"]
impl crate::Resettable for ArbprSpec {
    const RESET_VALUE: u32 = 0;
}
