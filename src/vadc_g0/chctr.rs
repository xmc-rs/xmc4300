#[doc = "Register `CHCTR[%s]` reader"]
pub type R = crate::R<CHCTR_SPEC>;
#[doc = "Register `CHCTR[%s]` writer"]
pub type W = crate::W<CHCTR_SPEC>;
#[doc = "Input Class Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICLSEL_A {
    #[doc = "0: Use group-specific class 0"]
    VALUE1 = 0,
    #[doc = "1: Use group-specific class 1"]
    VALUE2 = 1,
    #[doc = "2: Use global class 0"]
    VALUE3 = 2,
    #[doc = "3: Use global class 1"]
    VALUE4 = 3,
}
impl From<ICLSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ICLSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ICLSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for ICLSEL_A {}
#[doc = "Field `ICLSEL` reader - Input Class Select"]
pub type ICLSEL_R = crate::FieldReader<ICLSEL_A>;
impl ICLSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ICLSEL_A {
        match self.bits {
            0 => ICLSEL_A::VALUE1,
            1 => ICLSEL_A::VALUE2,
            2 => ICLSEL_A::VALUE3,
            3 => ICLSEL_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Use group-specific class 0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ICLSEL_A::VALUE1
    }
    #[doc = "Use group-specific class 1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ICLSEL_A::VALUE2
    }
    #[doc = "Use global class 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ICLSEL_A::VALUE3
    }
    #[doc = "Use global class 1"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == ICLSEL_A::VALUE4
    }
}
#[doc = "Field `ICLSEL` writer - Input Class Select"]
pub type ICLSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ICLSEL_A, crate::Safe>;
impl<'a, REG> ICLSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Use group-specific class 0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ICLSEL_A::VALUE1)
    }
    #[doc = "Use group-specific class 1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ICLSEL_A::VALUE2)
    }
    #[doc = "Use global class 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(ICLSEL_A::VALUE3)
    }
    #[doc = "Use global class 1"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(ICLSEL_A::VALUE4)
    }
}
#[doc = "Lower Boundary Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BNDSELL_A {
    #[doc = "0: Use group-specific boundary 0"]
    VALUE1 = 0,
    #[doc = "1: Use group-specific boundary 1"]
    VALUE2 = 1,
    #[doc = "2: Use global boundary 0"]
    VALUE3 = 2,
    #[doc = "3: Use global boundary 1"]
    VALUE4 = 3,
}
impl From<BNDSELL_A> for u8 {
    #[inline(always)]
    fn from(variant: BNDSELL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BNDSELL_A {
    type Ux = u8;
}
impl crate::IsEnum for BNDSELL_A {}
#[doc = "Field `BNDSELL` reader - Lower Boundary Select"]
pub type BNDSELL_R = crate::FieldReader<BNDSELL_A>;
impl BNDSELL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BNDSELL_A {
        match self.bits {
            0 => BNDSELL_A::VALUE1,
            1 => BNDSELL_A::VALUE2,
            2 => BNDSELL_A::VALUE3,
            3 => BNDSELL_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Use group-specific boundary 0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BNDSELL_A::VALUE1
    }
    #[doc = "Use group-specific boundary 1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BNDSELL_A::VALUE2
    }
    #[doc = "Use global boundary 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == BNDSELL_A::VALUE3
    }
    #[doc = "Use global boundary 1"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == BNDSELL_A::VALUE4
    }
}
#[doc = "Field `BNDSELL` writer - Lower Boundary Select"]
pub type BNDSELL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, BNDSELL_A, crate::Safe>;
impl<'a, REG> BNDSELL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Use group-specific boundary 0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BNDSELL_A::VALUE1)
    }
    #[doc = "Use group-specific boundary 1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BNDSELL_A::VALUE2)
    }
    #[doc = "Use global boundary 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(BNDSELL_A::VALUE3)
    }
    #[doc = "Use global boundary 1"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(BNDSELL_A::VALUE4)
    }
}
#[doc = "Upper Boundary Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BNDSELU_A {
    #[doc = "0: Use group-specific boundary 0"]
    VALUE1 = 0,
    #[doc = "1: Use group-specific boundary 1"]
    VALUE2 = 1,
    #[doc = "2: Use global boundary 0"]
    VALUE3 = 2,
    #[doc = "3: Use global boundary 1"]
    VALUE4 = 3,
}
impl From<BNDSELU_A> for u8 {
    #[inline(always)]
    fn from(variant: BNDSELU_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BNDSELU_A {
    type Ux = u8;
}
impl crate::IsEnum for BNDSELU_A {}
#[doc = "Field `BNDSELU` reader - Upper Boundary Select"]
pub type BNDSELU_R = crate::FieldReader<BNDSELU_A>;
impl BNDSELU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BNDSELU_A {
        match self.bits {
            0 => BNDSELU_A::VALUE1,
            1 => BNDSELU_A::VALUE2,
            2 => BNDSELU_A::VALUE3,
            3 => BNDSELU_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Use group-specific boundary 0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BNDSELU_A::VALUE1
    }
    #[doc = "Use group-specific boundary 1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BNDSELU_A::VALUE2
    }
    #[doc = "Use global boundary 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == BNDSELU_A::VALUE3
    }
    #[doc = "Use global boundary 1"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == BNDSELU_A::VALUE4
    }
}
#[doc = "Field `BNDSELU` writer - Upper Boundary Select"]
pub type BNDSELU_W<'a, REG> = crate::FieldWriter<'a, REG, 2, BNDSELU_A, crate::Safe>;
impl<'a, REG> BNDSELU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Use group-specific boundary 0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BNDSELU_A::VALUE1)
    }
    #[doc = "Use group-specific boundary 1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BNDSELU_A::VALUE2)
    }
    #[doc = "Use global boundary 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(BNDSELU_A::VALUE3)
    }
    #[doc = "Use global boundary 1"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(BNDSELU_A::VALUE4)
    }
}
#[doc = "Channel Event Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHEVMODE_A {
    #[doc = "0: Never"]
    VALUE1 = 0,
    #[doc = "1: NCM: If result is inside the boundary band FCM: If result becomes high (above cmp. val.)"]
    VALUE2 = 1,
    #[doc = "2: NCM: If result is outside the boundary band FCM: If result becomes low (below cmp. val.)"]
    VALUE3 = 2,
    #[doc = "3: NCM: Always (ignore band) FCM: If result switches to either level"]
    VALUE4 = 3,
}
impl From<CHEVMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CHEVMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CHEVMODE_A {
    type Ux = u8;
}
impl crate::IsEnum for CHEVMODE_A {}
#[doc = "Field `CHEVMODE` reader - Channel Event Mode"]
pub type CHEVMODE_R = crate::FieldReader<CHEVMODE_A>;
impl CHEVMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHEVMODE_A {
        match self.bits {
            0 => CHEVMODE_A::VALUE1,
            1 => CHEVMODE_A::VALUE2,
            2 => CHEVMODE_A::VALUE3,
            3 => CHEVMODE_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Never"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHEVMODE_A::VALUE1
    }
    #[doc = "NCM: If result is inside the boundary band FCM: If result becomes high (above cmp. val.)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHEVMODE_A::VALUE2
    }
    #[doc = "NCM: If result is outside the boundary band FCM: If result becomes low (below cmp. val.)"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CHEVMODE_A::VALUE3
    }
    #[doc = "NCM: Always (ignore band) FCM: If result switches to either level"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CHEVMODE_A::VALUE4
    }
}
#[doc = "Field `CHEVMODE` writer - Channel Event Mode"]
pub type CHEVMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CHEVMODE_A, crate::Safe>;
impl<'a, REG> CHEVMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Never"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CHEVMODE_A::VALUE1)
    }
    #[doc = "NCM: If result is inside the boundary band FCM: If result becomes high (above cmp. val.)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CHEVMODE_A::VALUE2)
    }
    #[doc = "NCM: If result is outside the boundary band FCM: If result becomes low (below cmp. val.)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CHEVMODE_A::VALUE3)
    }
    #[doc = "NCM: Always (ignore band) FCM: If result switches to either level"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(CHEVMODE_A::VALUE4)
    }
}
#[doc = "Synchronization Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNC_A {
    #[doc = "0: No synchroniz. request, standalone operation"]
    VALUE1 = 0,
    #[doc = "1: Request a synchronized conversion of this channel (only taken into account for a master)"]
    VALUE2 = 1,
}
impl From<SYNC_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNC` reader - Synchronization Request"]
pub type SYNC_R = crate::BitReader<SYNC_A>;
impl SYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYNC_A {
        match self.bits {
            false => SYNC_A::VALUE1,
            true => SYNC_A::VALUE2,
        }
    }
    #[doc = "No synchroniz. request, standalone operation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SYNC_A::VALUE1
    }
    #[doc = "Request a synchronized conversion of this channel (only taken into account for a master)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SYNC_A::VALUE2
    }
}
#[doc = "Field `SYNC` writer - Synchronization Request"]
pub type SYNC_W<'a, REG> = crate::BitWriter<'a, REG, SYNC_A>;
impl<'a, REG> SYNC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No synchroniz. request, standalone operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_A::VALUE1)
    }
    #[doc = "Request a synchronized conversion of this channel (only taken into account for a master)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_A::VALUE2)
    }
}
#[doc = "Reference Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFSEL_A {
    #[doc = "0: Standard reference input VAREF"]
    VALUE1 = 0,
    #[doc = "1: Alternate reference input from CH0"]
    VALUE2 = 1,
}
impl From<REFSEL_A> for bool {
    #[inline(always)]
    fn from(variant: REFSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFSEL` reader - Reference Input Selection"]
pub type REFSEL_R = crate::BitReader<REFSEL_A>;
impl REFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REFSEL_A {
        match self.bits {
            false => REFSEL_A::VALUE1,
            true => REFSEL_A::VALUE2,
        }
    }
    #[doc = "Standard reference input VAREF"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REFSEL_A::VALUE1
    }
    #[doc = "Alternate reference input from CH0"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REFSEL_A::VALUE2
    }
}
#[doc = "Field `REFSEL` writer - Reference Input Selection"]
pub type REFSEL_W<'a, REG> = crate::BitWriter<'a, REG, REFSEL_A>;
impl<'a, REG> REFSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard reference input VAREF"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(REFSEL_A::VALUE1)
    }
    #[doc = "Alternate reference input from CH0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(REFSEL_A::VALUE2)
    }
}
#[doc = "Result Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RESREG_A {
    #[doc = "0: Store result in group result register GxRES0"]
    VALUE1 = 0,
    #[doc = "15: Store result in group result register GxRES15"]
    VALUE2 = 15,
}
impl From<RESREG_A> for u8 {
    #[inline(always)]
    fn from(variant: RESREG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RESREG_A {
    type Ux = u8;
}
impl crate::IsEnum for RESREG_A {}
#[doc = "Field `RESREG` reader - Result Register"]
pub type RESREG_R = crate::FieldReader<RESREG_A>;
impl RESREG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RESREG_A> {
        match self.bits {
            0 => Some(RESREG_A::VALUE1),
            15 => Some(RESREG_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Store result in group result register GxRES0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RESREG_A::VALUE1
    }
    #[doc = "Store result in group result register GxRES15"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RESREG_A::VALUE2
    }
}
#[doc = "Field `RESREG` writer - Result Register"]
pub type RESREG_W<'a, REG> = crate::FieldWriter<'a, REG, 4, RESREG_A>;
impl<'a, REG> RESREG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Store result in group result register GxRES0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RESREG_A::VALUE1)
    }
    #[doc = "Store result in group result register GxRES15"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RESREG_A::VALUE2)
    }
}
#[doc = "Result Target for Background Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESTBS_A {
    #[doc = "0: Store results in the selected group result register"]
    VALUE1 = 0,
    #[doc = "1: Store results in the global result register"]
    VALUE2 = 1,
}
impl From<RESTBS_A> for bool {
    #[inline(always)]
    fn from(variant: RESTBS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESTBS` reader - Result Target for Background Source"]
pub type RESTBS_R = crate::BitReader<RESTBS_A>;
impl RESTBS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RESTBS_A {
        match self.bits {
            false => RESTBS_A::VALUE1,
            true => RESTBS_A::VALUE2,
        }
    }
    #[doc = "Store results in the selected group result register"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RESTBS_A::VALUE1
    }
    #[doc = "Store results in the global result register"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RESTBS_A::VALUE2
    }
}
#[doc = "Field `RESTBS` writer - Result Target for Background Source"]
pub type RESTBS_W<'a, REG> = crate::BitWriter<'a, REG, RESTBS_A>;
impl<'a, REG> RESTBS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Store results in the selected group result register"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RESTBS_A::VALUE1)
    }
    #[doc = "Store results in the global result register"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RESTBS_A::VALUE2)
    }
}
#[doc = "Result Position\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESPOS_A {
    #[doc = "0: Store results left-aligned"]
    VALUE1 = 0,
    #[doc = "1: Store results right-aligned"]
    VALUE2 = 1,
}
impl From<RESPOS_A> for bool {
    #[inline(always)]
    fn from(variant: RESPOS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESPOS` reader - Result Position"]
pub type RESPOS_R = crate::BitReader<RESPOS_A>;
impl RESPOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RESPOS_A {
        match self.bits {
            false => RESPOS_A::VALUE1,
            true => RESPOS_A::VALUE2,
        }
    }
    #[doc = "Store results left-aligned"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RESPOS_A::VALUE1
    }
    #[doc = "Store results right-aligned"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RESPOS_A::VALUE2
    }
}
#[doc = "Field `RESPOS` writer - Result Position"]
pub type RESPOS_W<'a, REG> = crate::BitWriter<'a, REG, RESPOS_A>;
impl<'a, REG> RESPOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Store results left-aligned"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RESPOS_A::VALUE1)
    }
    #[doc = "Store results right-aligned"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RESPOS_A::VALUE2)
    }
}
#[doc = "Broken Wire Detection Channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BWDCH_A {
    #[doc = "0: Select VAGND"]
    VALUE1 = 0,
    #[doc = "1: Select VAREF"]
    VALUE2 = 1,
}
impl From<BWDCH_A> for u8 {
    #[inline(always)]
    fn from(variant: BWDCH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BWDCH_A {
    type Ux = u8;
}
impl crate::IsEnum for BWDCH_A {}
#[doc = "Field `BWDCH` reader - Broken Wire Detection Channel"]
pub type BWDCH_R = crate::FieldReader<BWDCH_A>;
impl BWDCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BWDCH_A> {
        match self.bits {
            0 => Some(BWDCH_A::VALUE1),
            1 => Some(BWDCH_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Select VAGND"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BWDCH_A::VALUE1
    }
    #[doc = "Select VAREF"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BWDCH_A::VALUE2
    }
}
#[doc = "Field `BWDCH` writer - Broken Wire Detection Channel"]
pub type BWDCH_W<'a, REG> = crate::FieldWriter<'a, REG, 2, BWDCH_A>;
impl<'a, REG> BWDCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select VAGND"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BWDCH_A::VALUE1)
    }
    #[doc = "Select VAREF"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BWDCH_A::VALUE2)
    }
}
#[doc = "Broken Wire Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWDEN_A {
    #[doc = "0: Normal operation"]
    VALUE1 = 0,
    #[doc = "1: Additional preparation phase is enabled"]
    VALUE2 = 1,
}
impl From<BWDEN_A> for bool {
    #[inline(always)]
    fn from(variant: BWDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWDEN` reader - Broken Wire Detection Enable"]
pub type BWDEN_R = crate::BitReader<BWDEN_A>;
impl BWDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BWDEN_A {
        match self.bits {
            false => BWDEN_A::VALUE1,
            true => BWDEN_A::VALUE2,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BWDEN_A::VALUE1
    }
    #[doc = "Additional preparation phase is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BWDEN_A::VALUE2
    }
}
#[doc = "Field `BWDEN` writer - Broken Wire Detection Enable"]
pub type BWDEN_W<'a, REG> = crate::BitWriter<'a, REG, BWDEN_A>;
impl<'a, REG> BWDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BWDEN_A::VALUE1)
    }
    #[doc = "Additional preparation phase is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BWDEN_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:1 - Input Class Select"]
    #[inline(always)]
    pub fn iclsel(&self) -> ICLSEL_R {
        ICLSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Lower Boundary Select"]
    #[inline(always)]
    pub fn bndsell(&self) -> BNDSELL_R {
        BNDSELL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Upper Boundary Select"]
    #[inline(always)]
    pub fn bndselu(&self) -> BNDSELU_R {
        BNDSELU_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Channel Event Mode"]
    #[inline(always)]
    pub fn chevmode(&self) -> CHEVMODE_R {
        CHEVMODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Synchronization Request"]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reference Input Selection"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Result Register"]
    #[inline(always)]
    pub fn resreg(&self) -> RESREG_R {
        RESREG_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Result Target for Background Source"]
    #[inline(always)]
    pub fn restbs(&self) -> RESTBS_R {
        RESTBS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Result Position"]
    #[inline(always)]
    pub fn respos(&self) -> RESPOS_R {
        RESPOS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Broken Wire Detection Channel"]
    #[inline(always)]
    pub fn bwdch(&self) -> BWDCH_R {
        BWDCH_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Broken Wire Detection Enable"]
    #[inline(always)]
    pub fn bwden(&self) -> BWDEN_R {
        BWDEN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Input Class Select"]
    #[inline(always)]
    #[must_use]
    pub fn iclsel(&mut self) -> ICLSEL_W<CHCTR_SPEC> {
        ICLSEL_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Lower Boundary Select"]
    #[inline(always)]
    #[must_use]
    pub fn bndsell(&mut self) -> BNDSELL_W<CHCTR_SPEC> {
        BNDSELL_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Upper Boundary Select"]
    #[inline(always)]
    #[must_use]
    pub fn bndselu(&mut self) -> BNDSELU_W<CHCTR_SPEC> {
        BNDSELU_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Channel Event Mode"]
    #[inline(always)]
    #[must_use]
    pub fn chevmode(&mut self) -> CHEVMODE_W<CHCTR_SPEC> {
        CHEVMODE_W::new(self, 8)
    }
    #[doc = "Bit 10 - Synchronization Request"]
    #[inline(always)]
    #[must_use]
    pub fn sync(&mut self) -> SYNC_W<CHCTR_SPEC> {
        SYNC_W::new(self, 10)
    }
    #[doc = "Bit 11 - Reference Input Selection"]
    #[inline(always)]
    #[must_use]
    pub fn refsel(&mut self) -> REFSEL_W<CHCTR_SPEC> {
        REFSEL_W::new(self, 11)
    }
    #[doc = "Bits 16:19 - Result Register"]
    #[inline(always)]
    #[must_use]
    pub fn resreg(&mut self) -> RESREG_W<CHCTR_SPEC> {
        RESREG_W::new(self, 16)
    }
    #[doc = "Bit 20 - Result Target for Background Source"]
    #[inline(always)]
    #[must_use]
    pub fn restbs(&mut self) -> RESTBS_W<CHCTR_SPEC> {
        RESTBS_W::new(self, 20)
    }
    #[doc = "Bit 21 - Result Position"]
    #[inline(always)]
    #[must_use]
    pub fn respos(&mut self) -> RESPOS_W<CHCTR_SPEC> {
        RESPOS_W::new(self, 21)
    }
    #[doc = "Bits 28:29 - Broken Wire Detection Channel"]
    #[inline(always)]
    #[must_use]
    pub fn bwdch(&mut self) -> BWDCH_W<CHCTR_SPEC> {
        BWDCH_W::new(self, 28)
    }
    #[doc = "Bit 30 - Broken Wire Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bwden(&mut self) -> BWDEN_W<CHCTR_SPEC> {
        BWDEN_W::new(self, 30)
    }
}
#[doc = "Channel Ctrl. Reg.\n\nYou can [`read`](crate::Reg::read) this register and get [`chctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHCTR_SPEC;
impl crate::RegisterSpec for CHCTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chctr::R`](R) reader structure"]
impl crate::Readable for CHCTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chctr::W`](W) writer structure"]
impl crate::Writable for CHCTR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHCTR[%s]
to value 0"]
impl crate::Resettable for CHCTR_SPEC {
    const RESET_VALUE: u32 = 0;
}
