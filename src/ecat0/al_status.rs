#[doc = "Register `AL_STATUS` reader"]
pub type R = crate::R<AlStatusSpec>;
#[doc = "Register `AL_STATUS` writer"]
pub type W = crate::W<AlStatusSpec>;
#[doc = "Actual State of the Device State Machine\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum State {
    #[doc = "1: Init State"]
    Value1 = 1,
    #[doc = "2: Pre-Operational State"]
    Value2 = 2,
    #[doc = "3: Bootstrap State"]
    Value3 = 3,
    #[doc = "4: Safe-Operational State"]
    Value4 = 4,
    #[doc = "8: Operational State"]
    Value5 = 8,
}
impl From<State> for u8 {
    #[inline(always)]
    fn from(variant: State) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for State {
    type Ux = u8;
}
impl crate::IsEnum for State {}
#[doc = "Field `STATE` reader - Actual State of the Device State Machine"]
pub type StateR = crate::FieldReader<State>;
impl StateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<State> {
        match self.bits {
            1 => Some(State::Value1),
            2 => Some(State::Value2),
            3 => Some(State::Value3),
            4 => Some(State::Value4),
            8 => Some(State::Value5),
            _ => None,
        }
    }
    #[doc = "Init State"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == State::Value1
    }
    #[doc = "Pre-Operational State"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == State::Value2
    }
    #[doc = "Bootstrap State"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == State::Value3
    }
    #[doc = "Safe-Operational State"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == State::Value4
    }
    #[doc = "Operational State"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == State::Value5
    }
}
#[doc = "Field `STATE` writer - Actual State of the Device State Machine"]
pub type StateW<'a, REG> = crate::FieldWriter<'a, REG, 4, State>;
impl<'a, REG> StateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Init State"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(State::Value1)
    }
    #[doc = "Pre-Operational State"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(State::Value2)
    }
    #[doc = "Bootstrap State"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(State::Value3)
    }
    #[doc = "Safe-Operational State"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(State::Value4)
    }
    #[doc = "Operational State"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(State::Value5)
    }
}
#[doc = "Error Ind\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erri {
    #[doc = "0: Device is in State as requested or Flag cleared by command"]
    Value1 = 0,
    #[doc = "1: Device has not entered requested State or changed State as result of a local action"]
    Value2 = 1,
}
impl From<Erri> for bool {
    #[inline(always)]
    fn from(variant: Erri) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRI` reader - Error Ind"]
pub type ErriR = crate::BitReader<Erri>;
impl ErriR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Erri {
        match self.bits {
            false => Erri::Value1,
            true => Erri::Value2,
        }
    }
    #[doc = "Device is in State as requested or Flag cleared by command"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Erri::Value1
    }
    #[doc = "Device has not entered requested State or changed State as result of a local action"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Erri::Value2
    }
}
#[doc = "Field `ERRI` writer - Error Ind"]
pub type ErriW<'a, REG> = crate::BitWriter<'a, REG, Erri>;
impl<'a, REG> ErriW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Device is in State as requested or Flag cleared by command"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Erri::Value1)
    }
    #[doc = "Device has not entered requested State or changed State as result of a local action"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Erri::Value2)
    }
}
#[doc = "Device Identification\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Did {
    #[doc = "0: Device Identification not valid"]
    Value1 = 0,
    #[doc = "1: Device Identification loaded"]
    Value2 = 1,
}
impl From<Did> for bool {
    #[inline(always)]
    fn from(variant: Did) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DID` reader - Device Identification"]
pub type DidR = crate::BitReader<Did>;
impl DidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Did {
        match self.bits {
            false => Did::Value1,
            true => Did::Value2,
        }
    }
    #[doc = "Device Identification not valid"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Did::Value1
    }
    #[doc = "Device Identification loaded"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Did::Value2
    }
}
#[doc = "Field `DID` writer - Device Identification"]
pub type DidW<'a, REG> = crate::BitWriter<'a, REG, Did>;
impl<'a, REG> DidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Device Identification not valid"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Did::Value1)
    }
    #[doc = "Device Identification loaded"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Did::Value2)
    }
}
impl R {
    #[doc = "Bits 0:3 - Actual State of the Device State Machine"]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Error Ind"]
    #[inline(always)]
    pub fn erri(&self) -> ErriR {
        ErriR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Device Identification"]
    #[inline(always)]
    pub fn did(&self) -> DidR {
        DidR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Actual State of the Device State Machine"]
    #[inline(always)]
    #[must_use]
    pub fn state(&mut self) -> StateW<AlStatusSpec> {
        StateW::new(self, 0)
    }
    #[doc = "Bit 4 - Error Ind"]
    #[inline(always)]
    #[must_use]
    pub fn erri(&mut self) -> ErriW<AlStatusSpec> {
        ErriW::new(self, 4)
    }
    #[doc = "Bit 5 - Device Identification"]
    #[inline(always)]
    #[must_use]
    pub fn did(&mut self) -> DidW<AlStatusSpec> {
        DidW::new(self, 5)
    }
}
#[doc = "AL Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`al_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`al_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlStatusSpec;
impl crate::RegisterSpec for AlStatusSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`al_status::R`](R) reader structure"]
impl crate::Readable for AlStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`al_status::W`](W) writer structure"]
impl crate::Writable for AlStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets AL_STATUS to value 0x01"]
impl crate::Resettable for AlStatusSpec {
    const RESET_VALUE: u16 = 0x01;
}
