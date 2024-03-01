#[doc = "Register `FNCTL` reader"]
pub type R = crate::R<FnctlSpec>;
#[doc = "Register `FNCTL` writer"]
pub type W = crate::W<FnctlSpec>;
#[doc = "Touch-Sense TSIN Pad Turn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Padt {
    #[doc = "0: TSIN0"]
    Value1 = 0,
    #[doc = "7: TSIN7"]
    Value2 = 7,
}
impl From<Padt> for u8 {
    #[inline(always)]
    fn from(variant: Padt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Padt {
    type Ux = u8;
}
#[doc = "Field `PADT` reader - Touch-Sense TSIN Pad Turn"]
pub type PadtR = crate::FieldReader<Padt>;
impl PadtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Padt> {
        match self.bits {
            0 => Some(Padt::Value1),
            7 => Some(Padt::Value2),
            _ => None,
        }
    }
    #[doc = "TSIN0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Padt::Value1
    }
    #[doc = "TSIN7"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Padt::Value2
    }
}
#[doc = "Field `PADT` writer - Touch-Sense TSIN Pad Turn"]
pub type PadtW<'a, REG> = crate::FieldWriter<'a, REG, 3, Padt>;
impl<'a, REG> PadtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TSIN0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Padt::Value1)
    }
    #[doc = "TSIN7"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Padt::Value2)
    }
}
#[doc = "Software Control for Touch-Sense Pad Turn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Padtsw {
    #[doc = "0: The hardware automatically enables the touch-sense inputs in sequence round-robin, starting from TSIN0."]
    Value1 = 0,
    #[doc = "1: Disable hardware control for software control only. The touch-sense input is configured in bit PADT."]
    Value2 = 1,
}
impl From<Padtsw> for bool {
    #[inline(always)]
    fn from(variant: Padtsw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PADTSW` reader - Software Control for Touch-Sense Pad Turn"]
pub type PadtswR = crate::BitReader<Padtsw>;
impl PadtswR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Padtsw {
        match self.bits {
            false => Padtsw::Value1,
            true => Padtsw::Value2,
        }
    }
    #[doc = "The hardware automatically enables the touch-sense inputs in sequence round-robin, starting from TSIN0."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Padtsw::Value1
    }
    #[doc = "Disable hardware control for software control only. The touch-sense input is configured in bit PADT."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Padtsw::Value2
    }
}
#[doc = "Field `PADTSW` writer - Software Control for Touch-Sense Pad Turn"]
pub type PadtswW<'a, REG> = crate::BitWriter<'a, REG, Padtsw>;
impl<'a, REG> PadtswW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The hardware automatically enables the touch-sense inputs in sequence round-robin, starting from TSIN0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Padtsw::Value1)
    }
    #[doc = "Disable hardware control for software control only. The touch-sense input is configured in bit PADT."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Padtsw::Value2)
    }
}
#[doc = "Enable External Pull-up Configuration on Pin COLA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epull {
    #[doc = "0: HW over-rule to enable internal pull-up is active on TSIN\\[x\\]
for set duration in touch-sense time slice. With this setting, it is not specified to assign the COLA to any pin."]
    Value1 = 0,
    #[doc = "1: Enable external pull-up: Output 1 on pin COLA for whole duration of touch-sense time slice."]
    Value2 = 1,
}
impl From<Epull> for bool {
    #[inline(always)]
    fn from(variant: Epull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPULL` reader - Enable External Pull-up Configuration on Pin COLA"]
pub type EpullR = crate::BitReader<Epull>;
impl EpullR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epull {
        match self.bits {
            false => Epull::Value1,
            true => Epull::Value2,
        }
    }
    #[doc = "HW over-rule to enable internal pull-up is active on TSIN\\[x\\]
for set duration in touch-sense time slice. With this setting, it is not specified to assign the COLA to any pin."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Epull::Value1
    }
    #[doc = "Enable external pull-up: Output 1 on pin COLA for whole duration of touch-sense time slice."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Epull::Value2
    }
}
#[doc = "Field `EPULL` writer - Enable External Pull-up Configuration on Pin COLA"]
pub type EpullW<'a, REG> = crate::BitWriter<'a, REG, Epull>;
impl<'a, REG> EpullW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HW over-rule to enable internal pull-up is active on TSIN\\[x\\]
for set duration in touch-sense time slice. With this setting, it is not specified to assign the COLA to any pin."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Epull::Value1)
    }
    #[doc = "Enable external pull-up: Output 1 on pin COLA for whole duration of touch-sense time slice."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Epull::Value2)
    }
}
#[doc = "Field `FNCOL` reader - Previous Active Function/LED Column Status"]
pub type FncolR = crate::FieldReader;
#[doc = "Accumulate Count on Touch-Sense Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Acccnt {
    #[doc = "0: 1 time"]
    Value1 = 0,
    #[doc = "1: 2 times"]
    Value2 = 1,
    #[doc = "15: 16 times"]
    Value3 = 15,
}
impl From<Acccnt> for u8 {
    #[inline(always)]
    fn from(variant: Acccnt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Acccnt {
    type Ux = u8;
}
#[doc = "Field `ACCCNT` reader - Accumulate Count on Touch-Sense Input"]
pub type AcccntR = crate::FieldReader<Acccnt>;
impl AcccntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Acccnt> {
        match self.bits {
            0 => Some(Acccnt::Value1),
            1 => Some(Acccnt::Value2),
            15 => Some(Acccnt::Value3),
            _ => None,
        }
    }
    #[doc = "1 time"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Acccnt::Value1
    }
    #[doc = "2 times"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Acccnt::Value2
    }
    #[doc = "16 times"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Acccnt::Value3
    }
}
#[doc = "Field `ACCCNT` writer - Accumulate Count on Touch-Sense Input"]
pub type AcccntW<'a, REG> = crate::FieldWriter<'a, REG, 4, Acccnt>;
impl<'a, REG> AcccntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 time"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Acccnt::Value1)
    }
    #[doc = "2 times"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Acccnt::Value2)
    }
    #[doc = "16 times"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Acccnt::Value3)
    }
}
#[doc = "Common Compare Enable for Touch-Sense\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsccmp {
    #[doc = "0: Disable common compare for touch-sense"]
    Value1 = 0,
    #[doc = "1: Enable common compare for touch-sense"]
    Value2 = 1,
}
impl From<Tsccmp> for bool {
    #[inline(always)]
    fn from(variant: Tsccmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSCCMP` reader - Common Compare Enable for Touch-Sense"]
pub type TsccmpR = crate::BitReader<Tsccmp>;
impl TsccmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsccmp {
        match self.bits {
            false => Tsccmp::Value1,
            true => Tsccmp::Value2,
        }
    }
    #[doc = "Disable common compare for touch-sense"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tsccmp::Value1
    }
    #[doc = "Enable common compare for touch-sense"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tsccmp::Value2
    }
}
#[doc = "Field `TSCCMP` writer - Common Compare Enable for Touch-Sense"]
pub type TsccmpW<'a, REG> = crate::BitWriter<'a, REG, Tsccmp>;
impl<'a, REG> TsccmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable common compare for touch-sense"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Tsccmp::Value1)
    }
    #[doc = "Enable common compare for touch-sense"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Tsccmp::Value2)
    }
}
#[doc = "Extension for Touch-Sense Output for Pin-Low-Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tsoext {
    #[doc = "0: Extend by 1 ledts_clk"]
    Value1 = 0,
    #[doc = "1: Extend by 4 ledts_clk"]
    Value2 = 1,
    #[doc = "2: Extend by 8 ledts_clk"]
    Value3 = 2,
    #[doc = "3: Extend by 16 ledts_clk"]
    Value4 = 3,
}
impl From<Tsoext> for u8 {
    #[inline(always)]
    fn from(variant: Tsoext) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tsoext {
    type Ux = u8;
}
#[doc = "Field `TSOEXT` reader - Extension for Touch-Sense Output for Pin-Low-Level"]
pub type TsoextR = crate::FieldReader<Tsoext>;
impl TsoextR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsoext {
        match self.bits {
            0 => Tsoext::Value1,
            1 => Tsoext::Value2,
            2 => Tsoext::Value3,
            3 => Tsoext::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Extend by 1 ledts_clk"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tsoext::Value1
    }
    #[doc = "Extend by 4 ledts_clk"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tsoext::Value2
    }
    #[doc = "Extend by 8 ledts_clk"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Tsoext::Value3
    }
    #[doc = "Extend by 16 ledts_clk"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Tsoext::Value4
    }
}
#[doc = "Field `TSOEXT` writer - Extension for Touch-Sense Output for Pin-Low-Level"]
pub type TsoextW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Tsoext>;
impl<'a, REG> TsoextW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Extend by 1 ledts_clk"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Tsoext::Value1)
    }
    #[doc = "Extend by 4 ledts_clk"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Tsoext::Value2)
    }
    #[doc = "Extend by 8 ledts_clk"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Tsoext::Value3)
    }
    #[doc = "Extend by 16 ledts_clk"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Tsoext::Value4)
    }
}
#[doc = "TS-Counter Auto Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsctrr {
    #[doc = "0: Disable TS-counter automatic reset"]
    Value1 = 0,
    #[doc = "1: Enable TS-counter automatic reset to 00H on the first pad turn of a new TSIN\\[x\\]. Triggered on compare match in time slice."]
    Value2 = 1,
}
impl From<Tsctrr> for bool {
    #[inline(always)]
    fn from(variant: Tsctrr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSCTRR` reader - TS-Counter Auto Reset"]
pub type TsctrrR = crate::BitReader<Tsctrr>;
impl TsctrrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsctrr {
        match self.bits {
            false => Tsctrr::Value1,
            true => Tsctrr::Value2,
        }
    }
    #[doc = "Disable TS-counter automatic reset"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tsctrr::Value1
    }
    #[doc = "Enable TS-counter automatic reset to 00H on the first pad turn of a new TSIN\\[x\\]. Triggered on compare match in time slice."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tsctrr::Value2
    }
}
#[doc = "Field `TSCTRR` writer - TS-Counter Auto Reset"]
pub type TsctrrW<'a, REG> = crate::BitWriter<'a, REG, Tsctrr>;
impl<'a, REG> TsctrrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable TS-counter automatic reset"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Tsctrr::Value1)
    }
    #[doc = "Enable TS-counter automatic reset to 00H on the first pad turn of a new TSIN\\[x\\]. Triggered on compare match in time slice."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Tsctrr::Value2)
    }
}
#[doc = "Saturation of TS-Counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsctrsat {
    #[doc = "0: Disable"]
    Value1 = 0,
    #[doc = "1: Enable. TS-counter stops counting in the touch-sense time slice(s) of the same (extended) frame when it reaches FFH. Counter starts to count again on the first pad turn of a new TSIN\\[x\\], triggered on compare match."]
    Value2 = 1,
}
impl From<Tsctrsat> for bool {
    #[inline(always)]
    fn from(variant: Tsctrsat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSCTRSAT` reader - Saturation of TS-Counter"]
pub type TsctrsatR = crate::BitReader<Tsctrsat>;
impl TsctrsatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsctrsat {
        match self.bits {
            false => Tsctrsat::Value1,
            true => Tsctrsat::Value2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tsctrsat::Value1
    }
    #[doc = "Enable. TS-counter stops counting in the touch-sense time slice(s) of the same (extended) frame when it reaches FFH. Counter starts to count again on the first pad turn of a new TSIN\\[x\\], triggered on compare match."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tsctrsat::Value2
    }
}
#[doc = "Field `TSCTRSAT` writer - Saturation of TS-Counter"]
pub type TsctrsatW<'a, REG> = crate::BitWriter<'a, REG, Tsctrsat>;
impl<'a, REG> TsctrsatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Tsctrsat::Value1)
    }
    #[doc = "Enable. TS-counter stops counting in the touch-sense time slice(s) of the same (extended) frame when it reaches FFH. Counter starts to count again on the first pad turn of a new TSIN\\[x\\], triggered on compare match."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Tsctrsat::Value2)
    }
}
#[doc = "Number of Touch-Sense Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NrTsin {
    #[doc = "0: 1"]
    Value1 = 0,
    #[doc = "7: 8"]
    Value2 = 7,
}
impl From<NrTsin> for u8 {
    #[inline(always)]
    fn from(variant: NrTsin) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NrTsin {
    type Ux = u8;
}
#[doc = "Field `NR_TSIN` reader - Number of Touch-Sense Input"]
pub type NrTsinR = crate::FieldReader<NrTsin>;
impl NrTsinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<NrTsin> {
        match self.bits {
            0 => Some(NrTsin::Value1),
            7 => Some(NrTsin::Value2),
            _ => None,
        }
    }
    #[doc = "1"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NrTsin::Value1
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NrTsin::Value2
    }
}
#[doc = "Field `NR_TSIN` writer - Number of Touch-Sense Input"]
pub type NrTsinW<'a, REG> = crate::FieldWriter<'a, REG, 3, NrTsin>;
impl<'a, REG> NrTsinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(NrTsin::Value1)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(NrTsin::Value2)
    }
}
#[doc = "Active Level of LED Column\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Collev {
    #[doc = "0: Active low"]
    Value1 = 0,
    #[doc = "1: Active high"]
    Value2 = 1,
}
impl From<Collev> for bool {
    #[inline(always)]
    fn from(variant: Collev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COLLEV` reader - Active Level of LED Column"]
pub type CollevR = crate::BitReader<Collev>;
impl CollevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Collev {
        match self.bits {
            false => Collev::Value1,
            true => Collev::Value2,
        }
    }
    #[doc = "Active low"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Collev::Value1
    }
    #[doc = "Active high"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Collev::Value2
    }
}
#[doc = "Field `COLLEV` writer - Active Level of LED Column"]
pub type CollevW<'a, REG> = crate::BitWriter<'a, REG, Collev>;
impl<'a, REG> CollevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active low"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Collev::Value1)
    }
    #[doc = "Active high"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Collev::Value2)
    }
}
#[doc = "Number of LED Columns\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NrLedcol {
    #[doc = "0: 1 LED column"]
    Value1 = 0,
    #[doc = "1: 2 LED columns"]
    Value2 = 1,
    #[doc = "2: 3 LED columns"]
    Value3 = 2,
    #[doc = "3: 4 LED columns"]
    Value4 = 3,
    #[doc = "4: 5 LED columns"]
    Value5 = 4,
    #[doc = "5: 6 LED columns"]
    Value6 = 5,
    #[doc = "6: 7 LED columns"]
    Value7 = 6,
    #[doc = "7: 8 LED columns (max. LED columns = 7 if bit TS_EN = 1)"]
    Value8 = 7,
}
impl From<NrLedcol> for u8 {
    #[inline(always)]
    fn from(variant: NrLedcol) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NrLedcol {
    type Ux = u8;
}
#[doc = "Field `NR_LEDCOL` reader - Number of LED Columns"]
pub type NrLedcolR = crate::FieldReader<NrLedcol>;
impl NrLedcolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NrLedcol {
        match self.bits {
            0 => NrLedcol::Value1,
            1 => NrLedcol::Value2,
            2 => NrLedcol::Value3,
            3 => NrLedcol::Value4,
            4 => NrLedcol::Value5,
            5 => NrLedcol::Value6,
            6 => NrLedcol::Value7,
            7 => NrLedcol::Value8,
            _ => unreachable!(),
        }
    }
    #[doc = "1 LED column"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NrLedcol::Value1
    }
    #[doc = "2 LED columns"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NrLedcol::Value2
    }
    #[doc = "3 LED columns"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == NrLedcol::Value3
    }
    #[doc = "4 LED columns"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == NrLedcol::Value4
    }
    #[doc = "5 LED columns"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == NrLedcol::Value5
    }
    #[doc = "6 LED columns"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == NrLedcol::Value6
    }
    #[doc = "7 LED columns"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == NrLedcol::Value7
    }
    #[doc = "8 LED columns (max. LED columns = 7 if bit TS_EN = 1)"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == NrLedcol::Value8
    }
}
#[doc = "Field `NR_LEDCOL` writer - Number of LED Columns"]
pub type NrLedcolW<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, NrLedcol>;
impl<'a, REG> NrLedcolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 LED column"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(NrLedcol::Value1)
    }
    #[doc = "2 LED columns"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(NrLedcol::Value2)
    }
    #[doc = "3 LED columns"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(NrLedcol::Value3)
    }
    #[doc = "4 LED columns"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(NrLedcol::Value4)
    }
    #[doc = "5 LED columns"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(NrLedcol::Value5)
    }
    #[doc = "6 LED columns"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(NrLedcol::Value6)
    }
    #[doc = "7 LED columns"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(NrLedcol::Value7)
    }
    #[doc = "8 LED columns (max. LED columns = 7 if bit TS_EN = 1)"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(NrLedcol::Value8)
    }
}
impl R {
    #[doc = "Bits 0:2 - Touch-Sense TSIN Pad Turn"]
    #[inline(always)]
    pub fn padt(&self) -> PadtR {
        PadtR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Software Control for Touch-Sense Pad Turn"]
    #[inline(always)]
    pub fn padtsw(&self) -> PadtswR {
        PadtswR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable External Pull-up Configuration on Pin COLA"]
    #[inline(always)]
    pub fn epull(&self) -> EpullR {
        EpullR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Previous Active Function/LED Column Status"]
    #[inline(always)]
    pub fn fncol(&self) -> FncolR {
        FncolR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Accumulate Count on Touch-Sense Input"]
    #[inline(always)]
    pub fn acccnt(&self) -> AcccntR {
        AcccntR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Common Compare Enable for Touch-Sense"]
    #[inline(always)]
    pub fn tsccmp(&self) -> TsccmpR {
        TsccmpR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Extension for Touch-Sense Output for Pin-Low-Level"]
    #[inline(always)]
    pub fn tsoext(&self) -> TsoextR {
        TsoextR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - TS-Counter Auto Reset"]
    #[inline(always)]
    pub fn tsctrr(&self) -> TsctrrR {
        TsctrrR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Saturation of TS-Counter"]
    #[inline(always)]
    pub fn tsctrsat(&self) -> TsctrsatR {
        TsctrsatR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - Number of Touch-Sense Input"]
    #[inline(always)]
    pub fn nr_tsin(&self) -> NrTsinR {
        NrTsinR::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - Active Level of LED Column"]
    #[inline(always)]
    pub fn collev(&self) -> CollevR {
        CollevR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - Number of LED Columns"]
    #[inline(always)]
    pub fn nr_ledcol(&self) -> NrLedcolR {
        NrLedcolR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Touch-Sense TSIN Pad Turn"]
    #[inline(always)]
    #[must_use]
    pub fn padt(&mut self) -> PadtW<FnctlSpec> {
        PadtW::new(self, 0)
    }
    #[doc = "Bit 3 - Software Control for Touch-Sense Pad Turn"]
    #[inline(always)]
    #[must_use]
    pub fn padtsw(&mut self) -> PadtswW<FnctlSpec> {
        PadtswW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable External Pull-up Configuration on Pin COLA"]
    #[inline(always)]
    #[must_use]
    pub fn epull(&mut self) -> EpullW<FnctlSpec> {
        EpullW::new(self, 4)
    }
    #[doc = "Bits 16:19 - Accumulate Count on Touch-Sense Input"]
    #[inline(always)]
    #[must_use]
    pub fn acccnt(&mut self) -> AcccntW<FnctlSpec> {
        AcccntW::new(self, 16)
    }
    #[doc = "Bit 20 - Common Compare Enable for Touch-Sense"]
    #[inline(always)]
    #[must_use]
    pub fn tsccmp(&mut self) -> TsccmpW<FnctlSpec> {
        TsccmpW::new(self, 20)
    }
    #[doc = "Bits 21:22 - Extension for Touch-Sense Output for Pin-Low-Level"]
    #[inline(always)]
    #[must_use]
    pub fn tsoext(&mut self) -> TsoextW<FnctlSpec> {
        TsoextW::new(self, 21)
    }
    #[doc = "Bit 23 - TS-Counter Auto Reset"]
    #[inline(always)]
    #[must_use]
    pub fn tsctrr(&mut self) -> TsctrrW<FnctlSpec> {
        TsctrrW::new(self, 23)
    }
    #[doc = "Bit 24 - Saturation of TS-Counter"]
    #[inline(always)]
    #[must_use]
    pub fn tsctrsat(&mut self) -> TsctrsatW<FnctlSpec> {
        TsctrsatW::new(self, 24)
    }
    #[doc = "Bits 25:27 - Number of Touch-Sense Input"]
    #[inline(always)]
    #[must_use]
    pub fn nr_tsin(&mut self) -> NrTsinW<FnctlSpec> {
        NrTsinW::new(self, 25)
    }
    #[doc = "Bit 28 - Active Level of LED Column"]
    #[inline(always)]
    #[must_use]
    pub fn collev(&mut self) -> CollevW<FnctlSpec> {
        CollevW::new(self, 28)
    }
    #[doc = "Bits 29:31 - Number of LED Columns"]
    #[inline(always)]
    #[must_use]
    pub fn nr_ledcol(&mut self) -> NrLedcolW<FnctlSpec> {
        NrLedcolW::new(self, 29)
    }
}
#[doc = "Function Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fnctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fnctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FnctlSpec;
impl crate::RegisterSpec for FnctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fnctl::R`](R) reader structure"]
impl crate::Readable for FnctlSpec {}
#[doc = "`write(|w| ..)` method takes [`fnctl::W`](W) writer structure"]
impl crate::Writable for FnctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FNCTL to value 0"]
impl crate::Resettable for FnctlSpec {
    const RESET_VALUE: u32 = 0;
}
