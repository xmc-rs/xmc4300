#[doc = "Register `GCTRL` reader"]
pub type R = crate::R<GctrlSpec>;
#[doc = "Register `GCTRL` writer"]
pub type W = crate::W<GctrlSpec>;
#[doc = "Prescaler Clear Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prbc {
    #[doc = "0: SW only"]
    Value1 = 0,
    #[doc = "1: GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC40 is cleared."]
    Value2 = 1,
    #[doc = "2: GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC41 is cleared."]
    Value3 = 2,
    #[doc = "3: GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC42 is cleared."]
    Value4 = 3,
    #[doc = "4: GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC43 is cleared."]
    Value5 = 4,
}
impl From<Prbc> for u8 {
    #[inline(always)]
    fn from(variant: Prbc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prbc {
    type Ux = u8;
}
#[doc = "Field `PRBC` reader - Prescaler Clear Configuration"]
pub type PrbcR = crate::FieldReader<Prbc>;
impl PrbcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prbc> {
        match self.bits {
            0 => Some(Prbc::Value1),
            1 => Some(Prbc::Value2),
            2 => Some(Prbc::Value3),
            3 => Some(Prbc::Value4),
            4 => Some(Prbc::Value5),
            _ => None,
        }
    }
    #[doc = "SW only"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Prbc::Value1
    }
    #[doc = "GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC40 is cleared."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Prbc::Value2
    }
    #[doc = "GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC41 is cleared."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Prbc::Value3
    }
    #[doc = "GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC42 is cleared."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Prbc::Value4
    }
    #[doc = "GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC43 is cleared."]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Prbc::Value5
    }
}
#[doc = "Field `PRBC` writer - Prescaler Clear Configuration"]
pub type PrbcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Prbc>;
impl<'a, REG> PrbcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SW only"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Prbc::Value1)
    }
    #[doc = "GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC40 is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Prbc::Value2)
    }
    #[doc = "GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC41 is cleared."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Prbc::Value3)
    }
    #[doc = "GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC42 is cleared."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Prbc::Value4)
    }
    #[doc = "GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC43 is cleared."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Prbc::Value5)
    }
}
#[doc = "Prescaler Input Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pcis {
    #[doc = "0: Module clock"]
    Value1 = 0,
    #[doc = "1: CCU4x.ECLKA"]
    Value2 = 1,
    #[doc = "2: CCU4x.ECLKB"]
    Value3 = 2,
    #[doc = "3: CCU4x.ECLKC"]
    Value4 = 3,
}
impl From<Pcis> for u8 {
    #[inline(always)]
    fn from(variant: Pcis) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pcis {
    type Ux = u8;
}
#[doc = "Field `PCIS` reader - Prescaler Input Clock Selection"]
pub type PcisR = crate::FieldReader<Pcis>;
impl PcisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcis {
        match self.bits {
            0 => Pcis::Value1,
            1 => Pcis::Value2,
            2 => Pcis::Value3,
            3 => Pcis::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Module clock"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pcis::Value1
    }
    #[doc = "CCU4x.ECLKA"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pcis::Value2
    }
    #[doc = "CCU4x.ECLKB"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Pcis::Value3
    }
    #[doc = "CCU4x.ECLKC"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Pcis::Value4
    }
}
#[doc = "Field `PCIS` writer - Prescaler Input Clock Selection"]
pub type PcisW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Pcis>;
impl<'a, REG> PcisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Module clock"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pcis::Value1)
    }
    #[doc = "CCU4x.ECLKA"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pcis::Value2)
    }
    #[doc = "CCU4x.ECLKB"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Pcis::Value3)
    }
    #[doc = "CCU4x.ECLKC"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Pcis::Value4)
    }
}
#[doc = "Suspend Mode Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Suscfg {
    #[doc = "0: Suspend request ignored. The module never enters in suspend"]
    Value1 = 0,
    #[doc = "1: Stops all the running slices immediately. Safe stop is not applied."]
    Value2 = 1,
    #[doc = "2: Stops the block immediately and clamps all the outputs to PASSIVE state. Safe stop is applied."]
    Value3 = 2,
    #[doc = "3: Waits for the roll over of each slice to stop and clamp the slices outputs. Safe stop is applied."]
    Value4 = 3,
}
impl From<Suscfg> for u8 {
    #[inline(always)]
    fn from(variant: Suscfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Suscfg {
    type Ux = u8;
}
#[doc = "Field `SUSCFG` reader - Suspend Mode Configuration"]
pub type SuscfgR = crate::FieldReader<Suscfg>;
impl SuscfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Suscfg {
        match self.bits {
            0 => Suscfg::Value1,
            1 => Suscfg::Value2,
            2 => Suscfg::Value3,
            3 => Suscfg::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Suspend request ignored. The module never enters in suspend"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Suscfg::Value1
    }
    #[doc = "Stops all the running slices immediately. Safe stop is not applied."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Suscfg::Value2
    }
    #[doc = "Stops the block immediately and clamps all the outputs to PASSIVE state. Safe stop is applied."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Suscfg::Value3
    }
    #[doc = "Waits for the roll over of each slice to stop and clamp the slices outputs. Safe stop is applied."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Suscfg::Value4
    }
}
#[doc = "Field `SUSCFG` writer - Suspend Mode Configuration"]
pub type SuscfgW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Suscfg>;
impl<'a, REG> SuscfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Suspend request ignored. The module never enters in suspend"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Suscfg::Value1)
    }
    #[doc = "Stops all the running slices immediately. Safe stop is not applied."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Suscfg::Value2)
    }
    #[doc = "Stops the block immediately and clamps all the outputs to PASSIVE state. Safe stop is applied."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Suscfg::Value3)
    }
    #[doc = "Waits for the roll over of each slice to stop and clamp the slices outputs. Safe stop is applied."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Suscfg::Value4)
    }
}
#[doc = "Slice 0 Multi Channel shadow transfer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mse0 {
    #[doc = "0: Shadow transfer can only be requested by SW"]
    Value1 = 0,
    #[doc = "1: Shadow transfer can be requested via SW and via the CCU4x.MCSS input."]
    Value2 = 1,
}
impl From<Mse0> for bool {
    #[inline(always)]
    fn from(variant: Mse0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSE0` reader - Slice 0 Multi Channel shadow transfer enable"]
pub type Mse0R = crate::BitReader<Mse0>;
impl Mse0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mse0 {
        match self.bits {
            false => Mse0::Value1,
            true => Mse0::Value2,
        }
    }
    #[doc = "Shadow transfer can only be requested by SW"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mse0::Value1
    }
    #[doc = "Shadow transfer can be requested via SW and via the CCU4x.MCSS input."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mse0::Value2
    }
}
#[doc = "Field `MSE0` writer - Slice 0 Multi Channel shadow transfer enable"]
pub type Mse0W<'a, REG> = crate::BitWriter<'a, REG, Mse0>;
impl<'a, REG> Mse0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Shadow transfer can only be requested by SW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mse0::Value1)
    }
    #[doc = "Shadow transfer can be requested via SW and via the CCU4x.MCSS input."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mse0::Value2)
    }
}
#[doc = "Slice 1 Multi Channel shadow transfer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mse1 {
    #[doc = "0: Shadow transfer can only be requested by SW"]
    Value1 = 0,
    #[doc = "1: Shadow transfer can be requested via SW and via the CCU4x.MCSS input."]
    Value2 = 1,
}
impl From<Mse1> for bool {
    #[inline(always)]
    fn from(variant: Mse1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSE1` reader - Slice 1 Multi Channel shadow transfer enable"]
pub type Mse1R = crate::BitReader<Mse1>;
impl Mse1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mse1 {
        match self.bits {
            false => Mse1::Value1,
            true => Mse1::Value2,
        }
    }
    #[doc = "Shadow transfer can only be requested by SW"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mse1::Value1
    }
    #[doc = "Shadow transfer can be requested via SW and via the CCU4x.MCSS input."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mse1::Value2
    }
}
#[doc = "Field `MSE1` writer - Slice 1 Multi Channel shadow transfer enable"]
pub type Mse1W<'a, REG> = crate::BitWriter<'a, REG, Mse1>;
impl<'a, REG> Mse1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Shadow transfer can only be requested by SW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mse1::Value1)
    }
    #[doc = "Shadow transfer can be requested via SW and via the CCU4x.MCSS input."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mse1::Value2)
    }
}
#[doc = "Slice 2 Multi Channel shadow transfer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mse2 {
    #[doc = "0: Shadow transfer can only be requested by SW"]
    Value1 = 0,
    #[doc = "1: Shadow transfer can be requested via SW and via the CCU4x.MCSS input."]
    Value2 = 1,
}
impl From<Mse2> for bool {
    #[inline(always)]
    fn from(variant: Mse2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSE2` reader - Slice 2 Multi Channel shadow transfer enable"]
pub type Mse2R = crate::BitReader<Mse2>;
impl Mse2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mse2 {
        match self.bits {
            false => Mse2::Value1,
            true => Mse2::Value2,
        }
    }
    #[doc = "Shadow transfer can only be requested by SW"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mse2::Value1
    }
    #[doc = "Shadow transfer can be requested via SW and via the CCU4x.MCSS input."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mse2::Value2
    }
}
#[doc = "Field `MSE2` writer - Slice 2 Multi Channel shadow transfer enable"]
pub type Mse2W<'a, REG> = crate::BitWriter<'a, REG, Mse2>;
impl<'a, REG> Mse2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Shadow transfer can only be requested by SW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mse2::Value1)
    }
    #[doc = "Shadow transfer can be requested via SW and via the CCU4x.MCSS input."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mse2::Value2)
    }
}
#[doc = "Slice 3 Multi Channel shadow transfer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mse3 {
    #[doc = "0: Shadow transfer can only be requested by SW"]
    Value1 = 0,
    #[doc = "1: Shadow transfer can be requested via SW and via the CCU4x.MCSS input."]
    Value2 = 1,
}
impl From<Mse3> for bool {
    #[inline(always)]
    fn from(variant: Mse3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSE3` reader - Slice 3 Multi Channel shadow transfer enable"]
pub type Mse3R = crate::BitReader<Mse3>;
impl Mse3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mse3 {
        match self.bits {
            false => Mse3::Value1,
            true => Mse3::Value2,
        }
    }
    #[doc = "Shadow transfer can only be requested by SW"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mse3::Value1
    }
    #[doc = "Shadow transfer can be requested via SW and via the CCU4x.MCSS input."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mse3::Value2
    }
}
#[doc = "Field `MSE3` writer - Slice 3 Multi Channel shadow transfer enable"]
pub type Mse3W<'a, REG> = crate::BitWriter<'a, REG, Mse3>;
impl<'a, REG> Mse3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Shadow transfer can only be requested by SW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mse3::Value1)
    }
    #[doc = "Shadow transfer can be requested via SW and via the CCU4x.MCSS input."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mse3::Value2)
    }
}
#[doc = "Multi Channel shadow transfer request configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Msde {
    #[doc = "0: Only the shadow transfer for period and compare values is requested"]
    Value1 = 0,
    #[doc = "1: Shadow transfer for the compare, period and prescaler compare values is requested"]
    Value2 = 1,
    #[doc = "3: Shadow transfer for the compare, period, prescaler and dither compare values is requested"]
    Value4 = 3,
}
impl From<Msde> for u8 {
    #[inline(always)]
    fn from(variant: Msde) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Msde {
    type Ux = u8;
}
#[doc = "Field `MSDE` reader - Multi Channel shadow transfer request configuration"]
pub type MsdeR = crate::FieldReader<Msde>;
impl MsdeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Msde> {
        match self.bits {
            0 => Some(Msde::Value1),
            1 => Some(Msde::Value2),
            3 => Some(Msde::Value4),
            _ => None,
        }
    }
    #[doc = "Only the shadow transfer for period and compare values is requested"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Msde::Value1
    }
    #[doc = "Shadow transfer for the compare, period and prescaler compare values is requested"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Msde::Value2
    }
    #[doc = "Shadow transfer for the compare, period, prescaler and dither compare values is requested"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Msde::Value4
    }
}
#[doc = "Field `MSDE` writer - Multi Channel shadow transfer request configuration"]
pub type MsdeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Msde>;
impl<'a, REG> MsdeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Only the shadow transfer for period and compare values is requested"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Msde::Value1)
    }
    #[doc = "Shadow transfer for the compare, period and prescaler compare values is requested"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Msde::Value2)
    }
    #[doc = "Shadow transfer for the compare, period, prescaler and dither compare values is requested"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Msde::Value4)
    }
}
impl R {
    #[doc = "Bits 0:2 - Prescaler Clear Configuration"]
    #[inline(always)]
    pub fn prbc(&self) -> PrbcR {
        PrbcR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5 - Prescaler Input Clock Selection"]
    #[inline(always)]
    pub fn pcis(&self) -> PcisR {
        PcisR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Suspend Mode Configuration"]
    #[inline(always)]
    pub fn suscfg(&self) -> SuscfgR {
        SuscfgR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Slice 0 Multi Channel shadow transfer enable"]
    #[inline(always)]
    pub fn mse0(&self) -> Mse0R {
        Mse0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Slice 1 Multi Channel shadow transfer enable"]
    #[inline(always)]
    pub fn mse1(&self) -> Mse1R {
        Mse1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Slice 2 Multi Channel shadow transfer enable"]
    #[inline(always)]
    pub fn mse2(&self) -> Mse2R {
        Mse2R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Slice 3 Multi Channel shadow transfer enable"]
    #[inline(always)]
    pub fn mse3(&self) -> Mse3R {
        Mse3R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Multi Channel shadow transfer request configuration"]
    #[inline(always)]
    pub fn msde(&self) -> MsdeR {
        MsdeR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Prescaler Clear Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn prbc(&mut self) -> PrbcW<GctrlSpec> {
        PrbcW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Prescaler Input Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn pcis(&mut self) -> PcisW<GctrlSpec> {
        PcisW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Suspend Mode Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn suscfg(&mut self) -> SuscfgW<GctrlSpec> {
        SuscfgW::new(self, 8)
    }
    #[doc = "Bit 10 - Slice 0 Multi Channel shadow transfer enable"]
    #[inline(always)]
    #[must_use]
    pub fn mse0(&mut self) -> Mse0W<GctrlSpec> {
        Mse0W::new(self, 10)
    }
    #[doc = "Bit 11 - Slice 1 Multi Channel shadow transfer enable"]
    #[inline(always)]
    #[must_use]
    pub fn mse1(&mut self) -> Mse1W<GctrlSpec> {
        Mse1W::new(self, 11)
    }
    #[doc = "Bit 12 - Slice 2 Multi Channel shadow transfer enable"]
    #[inline(always)]
    #[must_use]
    pub fn mse2(&mut self) -> Mse2W<GctrlSpec> {
        Mse2W::new(self, 12)
    }
    #[doc = "Bit 13 - Slice 3 Multi Channel shadow transfer enable"]
    #[inline(always)]
    #[must_use]
    pub fn mse3(&mut self) -> Mse3W<GctrlSpec> {
        Mse3W::new(self, 13)
    }
    #[doc = "Bits 14:15 - Multi Channel shadow transfer request configuration"]
    #[inline(always)]
    #[must_use]
    pub fn msde(&mut self) -> MsdeW<GctrlSpec> {
        MsdeW::new(self, 14)
    }
}
#[doc = "Global Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GctrlSpec;
impl crate::RegisterSpec for GctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gctrl::R`](R) reader structure"]
impl crate::Readable for GctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`gctrl::W`](W) writer structure"]
impl crate::Writable for GctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GCTRL to value 0"]
impl crate::Resettable for GctrlSpec {
    const RESET_VALUE: u32 = 0;
}
