#[doc = "Register `EXICON[%s]` reader"]
pub type R = crate::R<EXICON_SPEC>;
#[doc = "Register `EXICON[%s]` writer"]
pub type W = crate::W<EXICON_SPEC>;
#[doc = "Output Trigger Pulse Enable for ETLx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PE_A {
    #[doc = "0: The trigger pulse generation is disabled"]
    VALUE1 = 0,
    #[doc = "1: The trigger pulse generation is enabled"]
    VALUE2 = 1,
}
impl From<PE_A> for bool {
    #[inline(always)]
    fn from(variant: PE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PE` reader - Output Trigger Pulse Enable for ETLx"]
pub type PE_R = crate::BitReader<PE_A>;
impl PE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PE_A {
        match self.bits {
            false => PE_A::VALUE1,
            true => PE_A::VALUE2,
        }
    }
    #[doc = "The trigger pulse generation is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PE_A::VALUE1
    }
    #[doc = "The trigger pulse generation is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PE_A::VALUE2
    }
}
#[doc = "Field `PE` writer - Output Trigger Pulse Enable for ETLx"]
pub type PE_W<'a, REG> = crate::BitWriter<'a, REG, PE_A>;
impl<'a, REG> PE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trigger pulse generation is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PE_A::VALUE1)
    }
    #[doc = "The trigger pulse generation is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PE_A::VALUE2)
    }
}
#[doc = "Rebuild Level Detection for Status Flag for ETLx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LD_A {
    #[doc = "0: The status flag FL is not cleared by hardware and is used as \"sticky\" bit. Once set, it is not influenced by any edge until it becomes cleared by software."]
    VALUE1 = 0,
    #[doc = "1: The status flag FL rebuilds a level detection of the desired event. It becomes automatically set with a rising edge if RE = 1 or with a falling edge if FE = 1. It becomes automatically cleared with a rising edge if RE = 0 or with a falling edge if FE = 0."]
    VALUE2 = 1,
}
impl From<LD_A> for bool {
    #[inline(always)]
    fn from(variant: LD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LD` reader - Rebuild Level Detection for Status Flag for ETLx"]
pub type LD_R = crate::BitReader<LD_A>;
impl LD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LD_A {
        match self.bits {
            false => LD_A::VALUE1,
            true => LD_A::VALUE2,
        }
    }
    #[doc = "The status flag FL is not cleared by hardware and is used as \"sticky\" bit. Once set, it is not influenced by any edge until it becomes cleared by software."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LD_A::VALUE1
    }
    #[doc = "The status flag FL rebuilds a level detection of the desired event. It becomes automatically set with a rising edge if RE = 1 or with a falling edge if FE = 1. It becomes automatically cleared with a rising edge if RE = 0 or with a falling edge if FE = 0."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LD_A::VALUE2
    }
}
#[doc = "Field `LD` writer - Rebuild Level Detection for Status Flag for ETLx"]
pub type LD_W<'a, REG> = crate::BitWriter<'a, REG, LD_A>;
impl<'a, REG> LD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The status flag FL is not cleared by hardware and is used as \"sticky\" bit. Once set, it is not influenced by any edge until it becomes cleared by software."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LD_A::VALUE1)
    }
    #[doc = "The status flag FL rebuilds a level detection of the desired event. It becomes automatically set with a rising edge if RE = 1 or with a falling edge if FE = 1. It becomes automatically cleared with a rising edge if RE = 0 or with a falling edge if FE = 0."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LD_A::VALUE2)
    }
}
#[doc = "Rising Edge Detection Enable ETLx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RE_A {
    #[doc = "0: A rising edge is not considered as edge event"]
    VALUE1 = 0,
    #[doc = "1: A rising edge is considered as edge event"]
    VALUE2 = 1,
}
impl From<RE_A> for bool {
    #[inline(always)]
    fn from(variant: RE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RE` reader - Rising Edge Detection Enable ETLx"]
pub type RE_R = crate::BitReader<RE_A>;
impl RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RE_A {
        match self.bits {
            false => RE_A::VALUE1,
            true => RE_A::VALUE2,
        }
    }
    #[doc = "A rising edge is not considered as edge event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RE_A::VALUE1
    }
    #[doc = "A rising edge is considered as edge event"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RE_A::VALUE2
    }
}
#[doc = "Field `RE` writer - Rising Edge Detection Enable ETLx"]
pub type RE_W<'a, REG> = crate::BitWriter<'a, REG, RE_A>;
impl<'a, REG> RE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A rising edge is not considered as edge event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RE_A::VALUE1)
    }
    #[doc = "A rising edge is considered as edge event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RE_A::VALUE2)
    }
}
#[doc = "Falling Edge Detection Enable ETLx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FE_A {
    #[doc = "0: A falling edge is not considered as edge event"]
    VALUE1 = 0,
    #[doc = "1: A falling edge is considered as edge event"]
    VALUE2 = 1,
}
impl From<FE_A> for bool {
    #[inline(always)]
    fn from(variant: FE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE` reader - Falling Edge Detection Enable ETLx"]
pub type FE_R = crate::BitReader<FE_A>;
impl FE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FE_A {
        match self.bits {
            false => FE_A::VALUE1,
            true => FE_A::VALUE2,
        }
    }
    #[doc = "A falling edge is not considered as edge event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FE_A::VALUE1
    }
    #[doc = "A falling edge is considered as edge event"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FE_A::VALUE2
    }
}
#[doc = "Field `FE` writer - Falling Edge Detection Enable ETLx"]
pub type FE_W<'a, REG> = crate::BitWriter<'a, REG, FE_A>;
impl<'a, REG> FE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A falling edge is not considered as edge event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FE_A::VALUE1)
    }
    #[doc = "A falling edge is considered as edge event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FE_A::VALUE2)
    }
}
#[doc = "Output Channel Select for ETLx Output Trigger Pulse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OCS_A {
    #[doc = "0: Trigger pulses are sent to OGU0"]
    VALUE1 = 0,
    #[doc = "1: Trigger pulses are sent to OGU1"]
    VALUE2 = 1,
    #[doc = "2: Trigger pulses are sent to OGU2"]
    VALUE3 = 2,
    #[doc = "3: Trigger pulses are sent to OGU3"]
    VALUE4 = 3,
}
impl From<OCS_A> for u8 {
    #[inline(always)]
    fn from(variant: OCS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OCS_A {
    type Ux = u8;
}
impl crate::IsEnum for OCS_A {}
#[doc = "Field `OCS` reader - Output Channel Select for ETLx Output Trigger Pulse"]
pub type OCS_R = crate::FieldReader<OCS_A>;
impl OCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OCS_A> {
        match self.bits {
            0 => Some(OCS_A::VALUE1),
            1 => Some(OCS_A::VALUE2),
            2 => Some(OCS_A::VALUE3),
            3 => Some(OCS_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Trigger pulses are sent to OGU0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OCS_A::VALUE1
    }
    #[doc = "Trigger pulses are sent to OGU1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OCS_A::VALUE2
    }
    #[doc = "Trigger pulses are sent to OGU2"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == OCS_A::VALUE3
    }
    #[doc = "Trigger pulses are sent to OGU3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == OCS_A::VALUE4
    }
}
#[doc = "Field `OCS` writer - Output Channel Select for ETLx Output Trigger Pulse"]
pub type OCS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, OCS_A>;
impl<'a, REG> OCS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trigger pulses are sent to OGU0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(OCS_A::VALUE1)
    }
    #[doc = "Trigger pulses are sent to OGU1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(OCS_A::VALUE2)
    }
    #[doc = "Trigger pulses are sent to OGU2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(OCS_A::VALUE3)
    }
    #[doc = "Trigger pulses are sent to OGU3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(OCS_A::VALUE4)
    }
}
#[doc = "Status Flag for ETLx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FL_A {
    #[doc = "0: The enabled edge event has not been detected"]
    VALUE1 = 0,
    #[doc = "1: The enabled edge event has been detected"]
    VALUE2 = 1,
}
impl From<FL_A> for bool {
    #[inline(always)]
    fn from(variant: FL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FL` reader - Status Flag for ETLx"]
pub type FL_R = crate::BitReader<FL_A>;
impl FL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FL_A {
        match self.bits {
            false => FL_A::VALUE1,
            true => FL_A::VALUE2,
        }
    }
    #[doc = "The enabled edge event has not been detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FL_A::VALUE1
    }
    #[doc = "The enabled edge event has been detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FL_A::VALUE2
    }
}
#[doc = "Field `FL` writer - Status Flag for ETLx"]
pub type FL_W<'a, REG> = crate::BitWriter<'a, REG, FL_A>;
impl<'a, REG> FL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The enabled edge event has not been detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FL_A::VALUE1)
    }
    #[doc = "The enabled edge event has been detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FL_A::VALUE2)
    }
}
#[doc = "Input Source Select for ERSx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SS_A {
    #[doc = "0: Input A without additional combination"]
    VALUE1 = 0,
    #[doc = "1: Input B without additional combination"]
    VALUE2 = 1,
    #[doc = "2: Input A OR input B"]
    VALUE3 = 2,
    #[doc = "3: Input A AND input B"]
    VALUE4 = 3,
}
impl From<SS_A> for u8 {
    #[inline(always)]
    fn from(variant: SS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SS_A {
    type Ux = u8;
}
impl crate::IsEnum for SS_A {}
#[doc = "Field `SS` reader - Input Source Select for ERSx"]
pub type SS_R = crate::FieldReader<SS_A>;
impl SS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SS_A {
        match self.bits {
            0 => SS_A::VALUE1,
            1 => SS_A::VALUE2,
            2 => SS_A::VALUE3,
            3 => SS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Input A without additional combination"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SS_A::VALUE1
    }
    #[doc = "Input B without additional combination"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SS_A::VALUE2
    }
    #[doc = "Input A OR input B"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SS_A::VALUE3
    }
    #[doc = "Input A AND input B"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SS_A::VALUE4
    }
}
#[doc = "Field `SS` writer - Input Source Select for ERSx"]
pub type SS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SS_A, crate::Safe>;
impl<'a, REG> SS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input A without additional combination"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SS_A::VALUE1)
    }
    #[doc = "Input B without additional combination"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SS_A::VALUE2)
    }
    #[doc = "Input A OR input B"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(SS_A::VALUE3)
    }
    #[doc = "Input A AND input B"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(SS_A::VALUE4)
    }
}
#[doc = "Input A Negation Select for ERSx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NA_A {
    #[doc = "0: Input A is used directly"]
    VALUE1 = 0,
    #[doc = "1: Input A is inverted"]
    VALUE2 = 1,
}
impl From<NA_A> for bool {
    #[inline(always)]
    fn from(variant: NA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NA` reader - Input A Negation Select for ERSx"]
pub type NA_R = crate::BitReader<NA_A>;
impl NA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NA_A {
        match self.bits {
            false => NA_A::VALUE1,
            true => NA_A::VALUE2,
        }
    }
    #[doc = "Input A is used directly"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NA_A::VALUE1
    }
    #[doc = "Input A is inverted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NA_A::VALUE2
    }
}
#[doc = "Field `NA` writer - Input A Negation Select for ERSx"]
pub type NA_W<'a, REG> = crate::BitWriter<'a, REG, NA_A>;
impl<'a, REG> NA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input A is used directly"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(NA_A::VALUE1)
    }
    #[doc = "Input A is inverted"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(NA_A::VALUE2)
    }
}
#[doc = "Input B Negation Select for ERSx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NB_A {
    #[doc = "0: Input B is used directly"]
    VALUE1 = 0,
    #[doc = "1: Input B is inverted"]
    VALUE2 = 1,
}
impl From<NB_A> for bool {
    #[inline(always)]
    fn from(variant: NB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NB` reader - Input B Negation Select for ERSx"]
pub type NB_R = crate::BitReader<NB_A>;
impl NB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NB_A {
        match self.bits {
            false => NB_A::VALUE1,
            true => NB_A::VALUE2,
        }
    }
    #[doc = "Input B is used directly"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NB_A::VALUE1
    }
    #[doc = "Input B is inverted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NB_A::VALUE2
    }
}
#[doc = "Field `NB` writer - Input B Negation Select for ERSx"]
pub type NB_W<'a, REG> = crate::BitWriter<'a, REG, NB_A>;
impl<'a, REG> NB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input B is used directly"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(NB_A::VALUE1)
    }
    #[doc = "Input B is inverted"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(NB_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Output Trigger Pulse Enable for ETLx"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rebuild Level Detection for Status Flag for ETLx"]
    #[inline(always)]
    pub fn ld(&self) -> LD_R {
        LD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rising Edge Detection Enable ETLx"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Falling Edge Detection Enable ETLx"]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Output Channel Select for ETLx Output Trigger Pulse"]
    #[inline(always)]
    pub fn ocs(&self) -> OCS_R {
        OCS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Status Flag for ETLx"]
    #[inline(always)]
    pub fn fl(&self) -> FL_R {
        FL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Input Source Select for ERSx"]
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Input A Negation Select for ERSx"]
    #[inline(always)]
    pub fn na(&self) -> NA_R {
        NA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Input B Negation Select for ERSx"]
    #[inline(always)]
    pub fn nb(&self) -> NB_R {
        NB_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output Trigger Pulse Enable for ETLx"]
    #[inline(always)]
    pub fn pe(&mut self) -> PE_W<EXICON_SPEC> {
        PE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Rebuild Level Detection for Status Flag for ETLx"]
    #[inline(always)]
    pub fn ld(&mut self) -> LD_W<EXICON_SPEC> {
        LD_W::new(self, 1)
    }
    #[doc = "Bit 2 - Rising Edge Detection Enable ETLx"]
    #[inline(always)]
    pub fn re(&mut self) -> RE_W<EXICON_SPEC> {
        RE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Falling Edge Detection Enable ETLx"]
    #[inline(always)]
    pub fn fe(&mut self) -> FE_W<EXICON_SPEC> {
        FE_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - Output Channel Select for ETLx Output Trigger Pulse"]
    #[inline(always)]
    pub fn ocs(&mut self) -> OCS_W<EXICON_SPEC> {
        OCS_W::new(self, 4)
    }
    #[doc = "Bit 7 - Status Flag for ETLx"]
    #[inline(always)]
    pub fn fl(&mut self) -> FL_W<EXICON_SPEC> {
        FL_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - Input Source Select for ERSx"]
    #[inline(always)]
    pub fn ss(&mut self) -> SS_W<EXICON_SPEC> {
        SS_W::new(self, 8)
    }
    #[doc = "Bit 10 - Input A Negation Select for ERSx"]
    #[inline(always)]
    pub fn na(&mut self) -> NA_W<EXICON_SPEC> {
        NA_W::new(self, 10)
    }
    #[doc = "Bit 11 - Input B Negation Select for ERSx"]
    #[inline(always)]
    pub fn nb(&mut self) -> NB_W<EXICON_SPEC> {
        NB_W::new(self, 11)
    }
}
#[doc = "Event Input Control\n\nYou can [`read`](crate::Reg::read) this register and get [`exicon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exicon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXICON_SPEC;
impl crate::RegisterSpec for EXICON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exicon::R`](R) reader structure"]
impl crate::Readable for EXICON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`exicon::W`](W) writer structure"]
impl crate::Writable for EXICON_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXICON[%s]
to value 0"]
impl crate::Resettable for EXICON_SPEC {
    const RESET_VALUE: u32 = 0;
}
