#[doc = "Register `SRACT` writer"]
pub type W = crate::W<SractSpec>;
#[doc = "Activate Group Service Request Node 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Agsr0 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Activate the associated service request line"]
    Value2 = 1,
}
impl From<Agsr0> for bool {
    #[inline(always)]
    fn from(variant: Agsr0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AGSR0` writer - Activate Group Service Request Node 0"]
pub type Agsr0W<'a, REG> = crate::BitWriter<'a, REG, Agsr0>;
impl<'a, REG> Agsr0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Agsr0::Value1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Agsr0::Value2)
    }
}
#[doc = "Activate Group Service Request Node 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Agsr1 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Activate the associated service request line"]
    Value2 = 1,
}
impl From<Agsr1> for bool {
    #[inline(always)]
    fn from(variant: Agsr1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AGSR1` writer - Activate Group Service Request Node 1"]
pub type Agsr1W<'a, REG> = crate::BitWriter<'a, REG, Agsr1>;
impl<'a, REG> Agsr1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Agsr1::Value1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Agsr1::Value2)
    }
}
#[doc = "Activate Group Service Request Node 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Agsr2 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Activate the associated service request line"]
    Value2 = 1,
}
impl From<Agsr2> for bool {
    #[inline(always)]
    fn from(variant: Agsr2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AGSR2` writer - Activate Group Service Request Node 2"]
pub type Agsr2W<'a, REG> = crate::BitWriter<'a, REG, Agsr2>;
impl<'a, REG> Agsr2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Agsr2::Value1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Agsr2::Value2)
    }
}
#[doc = "Activate Group Service Request Node 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Agsr3 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Activate the associated service request line"]
    Value2 = 1,
}
impl From<Agsr3> for bool {
    #[inline(always)]
    fn from(variant: Agsr3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AGSR3` writer - Activate Group Service Request Node 3"]
pub type Agsr3W<'a, REG> = crate::BitWriter<'a, REG, Agsr3>;
impl<'a, REG> Agsr3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Agsr3::Value1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Agsr3::Value2)
    }
}
#[doc = "Activate Shared Service Request Node 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Assr0 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Activate the associated service request line"]
    Value2 = 1,
}
impl From<Assr0> for bool {
    #[inline(always)]
    fn from(variant: Assr0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASSR0` writer - Activate Shared Service Request Node 0"]
pub type Assr0W<'a, REG> = crate::BitWriter<'a, REG, Assr0>;
impl<'a, REG> Assr0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Assr0::Value1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Assr0::Value2)
    }
}
#[doc = "Activate Shared Service Request Node 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Assr1 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Activate the associated service request line"]
    Value2 = 1,
}
impl From<Assr1> for bool {
    #[inline(always)]
    fn from(variant: Assr1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASSR1` writer - Activate Shared Service Request Node 1"]
pub type Assr1W<'a, REG> = crate::BitWriter<'a, REG, Assr1>;
impl<'a, REG> Assr1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Assr1::Value1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Assr1::Value2)
    }
}
#[doc = "Activate Shared Service Request Node 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Assr2 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Activate the associated service request line"]
    Value2 = 1,
}
impl From<Assr2> for bool {
    #[inline(always)]
    fn from(variant: Assr2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASSR2` writer - Activate Shared Service Request Node 2"]
pub type Assr2W<'a, REG> = crate::BitWriter<'a, REG, Assr2>;
impl<'a, REG> Assr2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Assr2::Value1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Assr2::Value2)
    }
}
#[doc = "Activate Shared Service Request Node 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Assr3 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Activate the associated service request line"]
    Value2 = 1,
}
impl From<Assr3> for bool {
    #[inline(always)]
    fn from(variant: Assr3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASSR3` writer - Activate Shared Service Request Node 3"]
pub type Assr3W<'a, REG> = crate::BitWriter<'a, REG, Assr3>;
impl<'a, REG> Assr3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Assr3::Value1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Assr3::Value2)
    }
}
impl W {
    #[doc = "Bit 0 - Activate Group Service Request Node 0"]
    #[inline(always)]
    #[must_use]
    pub fn agsr0(&mut self) -> Agsr0W<SractSpec> {
        Agsr0W::new(self, 0)
    }
    #[doc = "Bit 1 - Activate Group Service Request Node 1"]
    #[inline(always)]
    #[must_use]
    pub fn agsr1(&mut self) -> Agsr1W<SractSpec> {
        Agsr1W::new(self, 1)
    }
    #[doc = "Bit 2 - Activate Group Service Request Node 2"]
    #[inline(always)]
    #[must_use]
    pub fn agsr2(&mut self) -> Agsr2W<SractSpec> {
        Agsr2W::new(self, 2)
    }
    #[doc = "Bit 3 - Activate Group Service Request Node 3"]
    #[inline(always)]
    #[must_use]
    pub fn agsr3(&mut self) -> Agsr3W<SractSpec> {
        Agsr3W::new(self, 3)
    }
    #[doc = "Bit 8 - Activate Shared Service Request Node 0"]
    #[inline(always)]
    #[must_use]
    pub fn assr0(&mut self) -> Assr0W<SractSpec> {
        Assr0W::new(self, 8)
    }
    #[doc = "Bit 9 - Activate Shared Service Request Node 1"]
    #[inline(always)]
    #[must_use]
    pub fn assr1(&mut self) -> Assr1W<SractSpec> {
        Assr1W::new(self, 9)
    }
    #[doc = "Bit 10 - Activate Shared Service Request Node 2"]
    #[inline(always)]
    #[must_use]
    pub fn assr2(&mut self) -> Assr2W<SractSpec> {
        Assr2W::new(self, 10)
    }
    #[doc = "Bit 11 - Activate Shared Service Request Node 3"]
    #[inline(always)]
    #[must_use]
    pub fn assr3(&mut self) -> Assr3W<SractSpec> {
        Assr3W::new(self, 11)
    }
}
#[doc = "Service Request Software Activation Trigger\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sract::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SractSpec;
impl crate::RegisterSpec for SractSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sract::W`](W) writer structure"]
impl crate::Writable for SractSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRACT to value 0"]
impl crate::Resettable for SractSpec {
    const RESET_VALUE: u32 = 0;
}
