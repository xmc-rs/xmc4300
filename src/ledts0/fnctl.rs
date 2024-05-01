#[doc = "Register `FNCTL` reader"]
pub type R = crate::R<FNCTL_SPEC>;
#[doc = "Register `FNCTL` writer"]
pub type W = crate::W<FNCTL_SPEC>;
#[doc = "Touch-Sense TSIN Pad Turn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PADT_A {
    #[doc = "0: TSIN0"]
    VALUE1 = 0,
    #[doc = "7: TSIN7"]
    VALUE2 = 7,
}
impl From<PADT_A> for u8 {
    #[inline(always)]
    fn from(variant: PADT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PADT_A {
    type Ux = u8;
}
impl crate::IsEnum for PADT_A {}
#[doc = "Field `PADT` reader - Touch-Sense TSIN Pad Turn"]
pub type PADT_R = crate::FieldReader<PADT_A>;
impl PADT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PADT_A> {
        match self.bits {
            0 => Some(PADT_A::VALUE1),
            7 => Some(PADT_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "TSIN0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PADT_A::VALUE1
    }
    #[doc = "TSIN7"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PADT_A::VALUE2
    }
}
#[doc = "Field `PADT` writer - Touch-Sense TSIN Pad Turn"]
pub type PADT_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PADT_A>;
impl<'a, REG> PADT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TSIN0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PADT_A::VALUE1)
    }
    #[doc = "TSIN7"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PADT_A::VALUE2)
    }
}
#[doc = "Software Control for Touch-Sense Pad Turn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PADTSW_A {
    #[doc = "0: The hardware automatically enables the touch-sense inputs in sequence round-robin, starting from TSIN0."]
    VALUE1 = 0,
    #[doc = "1: Disable hardware control for software control only. The touch-sense input is configured in bit PADT."]
    VALUE2 = 1,
}
impl From<PADTSW_A> for bool {
    #[inline(always)]
    fn from(variant: PADTSW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PADTSW` reader - Software Control for Touch-Sense Pad Turn"]
pub type PADTSW_R = crate::BitReader<PADTSW_A>;
impl PADTSW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PADTSW_A {
        match self.bits {
            false => PADTSW_A::VALUE1,
            true => PADTSW_A::VALUE2,
        }
    }
    #[doc = "The hardware automatically enables the touch-sense inputs in sequence round-robin, starting from TSIN0."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PADTSW_A::VALUE1
    }
    #[doc = "Disable hardware control for software control only. The touch-sense input is configured in bit PADT."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PADTSW_A::VALUE2
    }
}
#[doc = "Field `PADTSW` writer - Software Control for Touch-Sense Pad Turn"]
pub type PADTSW_W<'a, REG> = crate::BitWriter<'a, REG, PADTSW_A>;
impl<'a, REG> PADTSW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The hardware automatically enables the touch-sense inputs in sequence round-robin, starting from TSIN0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PADTSW_A::VALUE1)
    }
    #[doc = "Disable hardware control for software control only. The touch-sense input is configured in bit PADT."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PADTSW_A::VALUE2)
    }
}
#[doc = "Enable External Pull-up Configuration on Pin COLA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EPULL_A {
    #[doc = "0: HW over-rule to enable internal pull-up is active on TSIN\\[x\\]
for set duration in touch-sense time slice. With this setting, it is not specified to assign the COLA to any pin."]
    VALUE1 = 0,
    #[doc = "1: Enable external pull-up: Output 1 on pin COLA for whole duration of touch-sense time slice."]
    VALUE2 = 1,
}
impl From<EPULL_A> for bool {
    #[inline(always)]
    fn from(variant: EPULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPULL` reader - Enable External Pull-up Configuration on Pin COLA"]
pub type EPULL_R = crate::BitReader<EPULL_A>;
impl EPULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EPULL_A {
        match self.bits {
            false => EPULL_A::VALUE1,
            true => EPULL_A::VALUE2,
        }
    }
    #[doc = "HW over-rule to enable internal pull-up is active on TSIN\\[x\\]
for set duration in touch-sense time slice. With this setting, it is not specified to assign the COLA to any pin."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EPULL_A::VALUE1
    }
    #[doc = "Enable external pull-up: Output 1 on pin COLA for whole duration of touch-sense time slice."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EPULL_A::VALUE2
    }
}
#[doc = "Field `EPULL` writer - Enable External Pull-up Configuration on Pin COLA"]
pub type EPULL_W<'a, REG> = crate::BitWriter<'a, REG, EPULL_A>;
impl<'a, REG> EPULL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HW over-rule to enable internal pull-up is active on TSIN\\[x\\]
for set duration in touch-sense time slice. With this setting, it is not specified to assign the COLA to any pin."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EPULL_A::VALUE1)
    }
    #[doc = "Enable external pull-up: Output 1 on pin COLA for whole duration of touch-sense time slice."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EPULL_A::VALUE2)
    }
}
#[doc = "Field `FNCOL` reader - Previous Active Function/LED Column Status"]
pub type FNCOL_R = crate::FieldReader;
#[doc = "Accumulate Count on Touch-Sense Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACCCNT_A {
    #[doc = "0: 1 time"]
    VALUE1 = 0,
    #[doc = "1: 2 times"]
    VALUE2 = 1,
    #[doc = "15: 16 times"]
    VALUE3 = 15,
}
impl From<ACCCNT_A> for u8 {
    #[inline(always)]
    fn from(variant: ACCCNT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ACCCNT_A {
    type Ux = u8;
}
impl crate::IsEnum for ACCCNT_A {}
#[doc = "Field `ACCCNT` reader - Accumulate Count on Touch-Sense Input"]
pub type ACCCNT_R = crate::FieldReader<ACCCNT_A>;
impl ACCCNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ACCCNT_A> {
        match self.bits {
            0 => Some(ACCCNT_A::VALUE1),
            1 => Some(ACCCNT_A::VALUE2),
            15 => Some(ACCCNT_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "1 time"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ACCCNT_A::VALUE1
    }
    #[doc = "2 times"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ACCCNT_A::VALUE2
    }
    #[doc = "16 times"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ACCCNT_A::VALUE3
    }
}
#[doc = "Field `ACCCNT` writer - Accumulate Count on Touch-Sense Input"]
pub type ACCCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, ACCCNT_A>;
impl<'a, REG> ACCCNT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 time"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ACCCNT_A::VALUE1)
    }
    #[doc = "2 times"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ACCCNT_A::VALUE2)
    }
    #[doc = "16 times"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(ACCCNT_A::VALUE3)
    }
}
#[doc = "Common Compare Enable for Touch-Sense\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSCCMP_A {
    #[doc = "0: Disable common compare for touch-sense"]
    VALUE1 = 0,
    #[doc = "1: Enable common compare for touch-sense"]
    VALUE2 = 1,
}
impl From<TSCCMP_A> for bool {
    #[inline(always)]
    fn from(variant: TSCCMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSCCMP` reader - Common Compare Enable for Touch-Sense"]
pub type TSCCMP_R = crate::BitReader<TSCCMP_A>;
impl TSCCMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSCCMP_A {
        match self.bits {
            false => TSCCMP_A::VALUE1,
            true => TSCCMP_A::VALUE2,
        }
    }
    #[doc = "Disable common compare for touch-sense"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TSCCMP_A::VALUE1
    }
    #[doc = "Enable common compare for touch-sense"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TSCCMP_A::VALUE2
    }
}
#[doc = "Field `TSCCMP` writer - Common Compare Enable for Touch-Sense"]
pub type TSCCMP_W<'a, REG> = crate::BitWriter<'a, REG, TSCCMP_A>;
impl<'a, REG> TSCCMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable common compare for touch-sense"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TSCCMP_A::VALUE1)
    }
    #[doc = "Enable common compare for touch-sense"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TSCCMP_A::VALUE2)
    }
}
#[doc = "Extension for Touch-Sense Output for Pin-Low-Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSOEXT_A {
    #[doc = "0: Extend by 1 ledts_clk"]
    VALUE1 = 0,
    #[doc = "1: Extend by 4 ledts_clk"]
    VALUE2 = 1,
    #[doc = "2: Extend by 8 ledts_clk"]
    VALUE3 = 2,
    #[doc = "3: Extend by 16 ledts_clk"]
    VALUE4 = 3,
}
impl From<TSOEXT_A> for u8 {
    #[inline(always)]
    fn from(variant: TSOEXT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TSOEXT_A {
    type Ux = u8;
}
impl crate::IsEnum for TSOEXT_A {}
#[doc = "Field `TSOEXT` reader - Extension for Touch-Sense Output for Pin-Low-Level"]
pub type TSOEXT_R = crate::FieldReader<TSOEXT_A>;
impl TSOEXT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSOEXT_A {
        match self.bits {
            0 => TSOEXT_A::VALUE1,
            1 => TSOEXT_A::VALUE2,
            2 => TSOEXT_A::VALUE3,
            3 => TSOEXT_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Extend by 1 ledts_clk"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TSOEXT_A::VALUE1
    }
    #[doc = "Extend by 4 ledts_clk"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TSOEXT_A::VALUE2
    }
    #[doc = "Extend by 8 ledts_clk"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TSOEXT_A::VALUE3
    }
    #[doc = "Extend by 16 ledts_clk"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == TSOEXT_A::VALUE4
    }
}
#[doc = "Field `TSOEXT` writer - Extension for Touch-Sense Output for Pin-Low-Level"]
pub type TSOEXT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TSOEXT_A, crate::Safe>;
impl<'a, REG> TSOEXT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Extend by 1 ledts_clk"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TSOEXT_A::VALUE1)
    }
    #[doc = "Extend by 4 ledts_clk"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TSOEXT_A::VALUE2)
    }
    #[doc = "Extend by 8 ledts_clk"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(TSOEXT_A::VALUE3)
    }
    #[doc = "Extend by 16 ledts_clk"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(TSOEXT_A::VALUE4)
    }
}
#[doc = "TS-Counter Auto Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSCTRR_A {
    #[doc = "0: Disable TS-counter automatic reset"]
    VALUE1 = 0,
    #[doc = "1: Enable TS-counter automatic reset to 00H on the first pad turn of a new TSIN\\[x\\]. Triggered on compare match in time slice."]
    VALUE2 = 1,
}
impl From<TSCTRR_A> for bool {
    #[inline(always)]
    fn from(variant: TSCTRR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSCTRR` reader - TS-Counter Auto Reset"]
pub type TSCTRR_R = crate::BitReader<TSCTRR_A>;
impl TSCTRR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSCTRR_A {
        match self.bits {
            false => TSCTRR_A::VALUE1,
            true => TSCTRR_A::VALUE2,
        }
    }
    #[doc = "Disable TS-counter automatic reset"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TSCTRR_A::VALUE1
    }
    #[doc = "Enable TS-counter automatic reset to 00H on the first pad turn of a new TSIN\\[x\\]. Triggered on compare match in time slice."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TSCTRR_A::VALUE2
    }
}
#[doc = "Field `TSCTRR` writer - TS-Counter Auto Reset"]
pub type TSCTRR_W<'a, REG> = crate::BitWriter<'a, REG, TSCTRR_A>;
impl<'a, REG> TSCTRR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable TS-counter automatic reset"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TSCTRR_A::VALUE1)
    }
    #[doc = "Enable TS-counter automatic reset to 00H on the first pad turn of a new TSIN\\[x\\]. Triggered on compare match in time slice."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TSCTRR_A::VALUE2)
    }
}
#[doc = "Saturation of TS-Counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSCTRSAT_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable. TS-counter stops counting in the touch-sense time slice(s) of the same (extended) frame when it reaches FFH. Counter starts to count again on the first pad turn of a new TSIN\\[x\\], triggered on compare match."]
    VALUE2 = 1,
}
impl From<TSCTRSAT_A> for bool {
    #[inline(always)]
    fn from(variant: TSCTRSAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSCTRSAT` reader - Saturation of TS-Counter"]
pub type TSCTRSAT_R = crate::BitReader<TSCTRSAT_A>;
impl TSCTRSAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSCTRSAT_A {
        match self.bits {
            false => TSCTRSAT_A::VALUE1,
            true => TSCTRSAT_A::VALUE2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TSCTRSAT_A::VALUE1
    }
    #[doc = "Enable. TS-counter stops counting in the touch-sense time slice(s) of the same (extended) frame when it reaches FFH. Counter starts to count again on the first pad turn of a new TSIN\\[x\\], triggered on compare match."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TSCTRSAT_A::VALUE2
    }
}
#[doc = "Field `TSCTRSAT` writer - Saturation of TS-Counter"]
pub type TSCTRSAT_W<'a, REG> = crate::BitWriter<'a, REG, TSCTRSAT_A>;
impl<'a, REG> TSCTRSAT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TSCTRSAT_A::VALUE1)
    }
    #[doc = "Enable. TS-counter stops counting in the touch-sense time slice(s) of the same (extended) frame when it reaches FFH. Counter starts to count again on the first pad turn of a new TSIN\\[x\\], triggered on compare match."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TSCTRSAT_A::VALUE2)
    }
}
#[doc = "Number of Touch-Sense Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NR_TSIN_A {
    #[doc = "0: 1"]
    VALUE1 = 0,
    #[doc = "7: 8"]
    VALUE2 = 7,
}
impl From<NR_TSIN_A> for u8 {
    #[inline(always)]
    fn from(variant: NR_TSIN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NR_TSIN_A {
    type Ux = u8;
}
impl crate::IsEnum for NR_TSIN_A {}
#[doc = "Field `NR_TSIN` reader - Number of Touch-Sense Input"]
pub type NR_TSIN_R = crate::FieldReader<NR_TSIN_A>;
impl NR_TSIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<NR_TSIN_A> {
        match self.bits {
            0 => Some(NR_TSIN_A::VALUE1),
            7 => Some(NR_TSIN_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "1"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NR_TSIN_A::VALUE1
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NR_TSIN_A::VALUE2
    }
}
#[doc = "Field `NR_TSIN` writer - Number of Touch-Sense Input"]
pub type NR_TSIN_W<'a, REG> = crate::FieldWriter<'a, REG, 3, NR_TSIN_A>;
impl<'a, REG> NR_TSIN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(NR_TSIN_A::VALUE1)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(NR_TSIN_A::VALUE2)
    }
}
#[doc = "Active Level of LED Column\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COLLEV_A {
    #[doc = "0: Active low"]
    VALUE1 = 0,
    #[doc = "1: Active high"]
    VALUE2 = 1,
}
impl From<COLLEV_A> for bool {
    #[inline(always)]
    fn from(variant: COLLEV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COLLEV` reader - Active Level of LED Column"]
pub type COLLEV_R = crate::BitReader<COLLEV_A>;
impl COLLEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COLLEV_A {
        match self.bits {
            false => COLLEV_A::VALUE1,
            true => COLLEV_A::VALUE2,
        }
    }
    #[doc = "Active low"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == COLLEV_A::VALUE1
    }
    #[doc = "Active high"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == COLLEV_A::VALUE2
    }
}
#[doc = "Field `COLLEV` writer - Active Level of LED Column"]
pub type COLLEV_W<'a, REG> = crate::BitWriter<'a, REG, COLLEV_A>;
impl<'a, REG> COLLEV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active low"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(COLLEV_A::VALUE1)
    }
    #[doc = "Active high"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(COLLEV_A::VALUE2)
    }
}
#[doc = "Number of LED Columns\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NR_LEDCOL_A {
    #[doc = "0: 1 LED column"]
    VALUE1 = 0,
    #[doc = "1: 2 LED columns"]
    VALUE2 = 1,
    #[doc = "2: 3 LED columns"]
    VALUE3 = 2,
    #[doc = "3: 4 LED columns"]
    VALUE4 = 3,
    #[doc = "4: 5 LED columns"]
    VALUE5 = 4,
    #[doc = "5: 6 LED columns"]
    VALUE6 = 5,
    #[doc = "6: 7 LED columns"]
    VALUE7 = 6,
    #[doc = "7: 8 LED columns (max. LED columns = 7 if bit TS_EN = 1)"]
    VALUE8 = 7,
}
impl From<NR_LEDCOL_A> for u8 {
    #[inline(always)]
    fn from(variant: NR_LEDCOL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NR_LEDCOL_A {
    type Ux = u8;
}
impl crate::IsEnum for NR_LEDCOL_A {}
#[doc = "Field `NR_LEDCOL` reader - Number of LED Columns"]
pub type NR_LEDCOL_R = crate::FieldReader<NR_LEDCOL_A>;
impl NR_LEDCOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NR_LEDCOL_A {
        match self.bits {
            0 => NR_LEDCOL_A::VALUE1,
            1 => NR_LEDCOL_A::VALUE2,
            2 => NR_LEDCOL_A::VALUE3,
            3 => NR_LEDCOL_A::VALUE4,
            4 => NR_LEDCOL_A::VALUE5,
            5 => NR_LEDCOL_A::VALUE6,
            6 => NR_LEDCOL_A::VALUE7,
            7 => NR_LEDCOL_A::VALUE8,
            _ => unreachable!(),
        }
    }
    #[doc = "1 LED column"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NR_LEDCOL_A::VALUE1
    }
    #[doc = "2 LED columns"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NR_LEDCOL_A::VALUE2
    }
    #[doc = "3 LED columns"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == NR_LEDCOL_A::VALUE3
    }
    #[doc = "4 LED columns"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == NR_LEDCOL_A::VALUE4
    }
    #[doc = "5 LED columns"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == NR_LEDCOL_A::VALUE5
    }
    #[doc = "6 LED columns"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == NR_LEDCOL_A::VALUE6
    }
    #[doc = "7 LED columns"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == NR_LEDCOL_A::VALUE7
    }
    #[doc = "8 LED columns (max. LED columns = 7 if bit TS_EN = 1)"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == NR_LEDCOL_A::VALUE8
    }
}
#[doc = "Field `NR_LEDCOL` writer - Number of LED Columns"]
pub type NR_LEDCOL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, NR_LEDCOL_A, crate::Safe>;
impl<'a, REG> NR_LEDCOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 LED column"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(NR_LEDCOL_A::VALUE1)
    }
    #[doc = "2 LED columns"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(NR_LEDCOL_A::VALUE2)
    }
    #[doc = "3 LED columns"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(NR_LEDCOL_A::VALUE3)
    }
    #[doc = "4 LED columns"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(NR_LEDCOL_A::VALUE4)
    }
    #[doc = "5 LED columns"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(NR_LEDCOL_A::VALUE5)
    }
    #[doc = "6 LED columns"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(NR_LEDCOL_A::VALUE6)
    }
    #[doc = "7 LED columns"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(NR_LEDCOL_A::VALUE7)
    }
    #[doc = "8 LED columns (max. LED columns = 7 if bit TS_EN = 1)"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(NR_LEDCOL_A::VALUE8)
    }
}
impl R {
    #[doc = "Bits 0:2 - Touch-Sense TSIN Pad Turn"]
    #[inline(always)]
    pub fn padt(&self) -> PADT_R {
        PADT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Software Control for Touch-Sense Pad Turn"]
    #[inline(always)]
    pub fn padtsw(&self) -> PADTSW_R {
        PADTSW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable External Pull-up Configuration on Pin COLA"]
    #[inline(always)]
    pub fn epull(&self) -> EPULL_R {
        EPULL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Previous Active Function/LED Column Status"]
    #[inline(always)]
    pub fn fncol(&self) -> FNCOL_R {
        FNCOL_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Accumulate Count on Touch-Sense Input"]
    #[inline(always)]
    pub fn acccnt(&self) -> ACCCNT_R {
        ACCCNT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Common Compare Enable for Touch-Sense"]
    #[inline(always)]
    pub fn tsccmp(&self) -> TSCCMP_R {
        TSCCMP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Extension for Touch-Sense Output for Pin-Low-Level"]
    #[inline(always)]
    pub fn tsoext(&self) -> TSOEXT_R {
        TSOEXT_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - TS-Counter Auto Reset"]
    #[inline(always)]
    pub fn tsctrr(&self) -> TSCTRR_R {
        TSCTRR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Saturation of TS-Counter"]
    #[inline(always)]
    pub fn tsctrsat(&self) -> TSCTRSAT_R {
        TSCTRSAT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - Number of Touch-Sense Input"]
    #[inline(always)]
    pub fn nr_tsin(&self) -> NR_TSIN_R {
        NR_TSIN_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - Active Level of LED Column"]
    #[inline(always)]
    pub fn collev(&self) -> COLLEV_R {
        COLLEV_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - Number of LED Columns"]
    #[inline(always)]
    pub fn nr_ledcol(&self) -> NR_LEDCOL_R {
        NR_LEDCOL_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Touch-Sense TSIN Pad Turn"]
    #[inline(always)]
    #[must_use]
    pub fn padt(&mut self) -> PADT_W<FNCTL_SPEC> {
        PADT_W::new(self, 0)
    }
    #[doc = "Bit 3 - Software Control for Touch-Sense Pad Turn"]
    #[inline(always)]
    #[must_use]
    pub fn padtsw(&mut self) -> PADTSW_W<FNCTL_SPEC> {
        PADTSW_W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable External Pull-up Configuration on Pin COLA"]
    #[inline(always)]
    #[must_use]
    pub fn epull(&mut self) -> EPULL_W<FNCTL_SPEC> {
        EPULL_W::new(self, 4)
    }
    #[doc = "Bits 16:19 - Accumulate Count on Touch-Sense Input"]
    #[inline(always)]
    #[must_use]
    pub fn acccnt(&mut self) -> ACCCNT_W<FNCTL_SPEC> {
        ACCCNT_W::new(self, 16)
    }
    #[doc = "Bit 20 - Common Compare Enable for Touch-Sense"]
    #[inline(always)]
    #[must_use]
    pub fn tsccmp(&mut self) -> TSCCMP_W<FNCTL_SPEC> {
        TSCCMP_W::new(self, 20)
    }
    #[doc = "Bits 21:22 - Extension for Touch-Sense Output for Pin-Low-Level"]
    #[inline(always)]
    #[must_use]
    pub fn tsoext(&mut self) -> TSOEXT_W<FNCTL_SPEC> {
        TSOEXT_W::new(self, 21)
    }
    #[doc = "Bit 23 - TS-Counter Auto Reset"]
    #[inline(always)]
    #[must_use]
    pub fn tsctrr(&mut self) -> TSCTRR_W<FNCTL_SPEC> {
        TSCTRR_W::new(self, 23)
    }
    #[doc = "Bit 24 - Saturation of TS-Counter"]
    #[inline(always)]
    #[must_use]
    pub fn tsctrsat(&mut self) -> TSCTRSAT_W<FNCTL_SPEC> {
        TSCTRSAT_W::new(self, 24)
    }
    #[doc = "Bits 25:27 - Number of Touch-Sense Input"]
    #[inline(always)]
    #[must_use]
    pub fn nr_tsin(&mut self) -> NR_TSIN_W<FNCTL_SPEC> {
        NR_TSIN_W::new(self, 25)
    }
    #[doc = "Bit 28 - Active Level of LED Column"]
    #[inline(always)]
    #[must_use]
    pub fn collev(&mut self) -> COLLEV_W<FNCTL_SPEC> {
        COLLEV_W::new(self, 28)
    }
    #[doc = "Bits 29:31 - Number of LED Columns"]
    #[inline(always)]
    #[must_use]
    pub fn nr_ledcol(&mut self) -> NR_LEDCOL_W<FNCTL_SPEC> {
        NR_LEDCOL_W::new(self, 29)
    }
}
#[doc = "Function Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fnctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fnctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FNCTL_SPEC;
impl crate::RegisterSpec for FNCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fnctl::R`](R) reader structure"]
impl crate::Readable for FNCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fnctl::W`](W) writer structure"]
impl crate::Writable for FNCTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FNCTL to value 0"]
impl crate::Resettable for FNCTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
