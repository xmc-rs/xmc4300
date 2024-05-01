#[doc = "Register `AL_EVENT_REQ` reader"]
pub type R = crate::R<AL_EVENT_REQ_SPEC>;
#[doc = "Register `AL_EVENT_REQ` writer"]
pub type W = crate::W<AL_EVENT_REQ_SPEC>;
#[doc = "AL Control event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AL_CE_A {
    #[doc = "0: No AL Control Register change"]
    VALUE1 = 0,
    #[doc = "1: AL Control Register has been written"]
    VALUE2 = 1,
}
impl From<AL_CE_A> for bool {
    #[inline(always)]
    fn from(variant: AL_CE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AL_CE` reader - AL Control event"]
pub type AL_CE_R = crate::BitReader<AL_CE_A>;
impl AL_CE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AL_CE_A {
        match self.bits {
            false => AL_CE_A::VALUE1,
            true => AL_CE_A::VALUE2,
        }
    }
    #[doc = "No AL Control Register change"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AL_CE_A::VALUE1
    }
    #[doc = "AL Control Register has been written"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AL_CE_A::VALUE2
    }
}
#[doc = "DC Latch event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DC_LE_A {
    #[doc = "0: No change on DC Latch Inputs"]
    VALUE1 = 0,
    #[doc = "1: At least one change on DC Latch Inputs"]
    VALUE2 = 1,
}
impl From<DC_LE_A> for bool {
    #[inline(always)]
    fn from(variant: DC_LE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DC_LE` reader - DC Latch event"]
pub type DC_LE_R = crate::BitReader<DC_LE_A>;
impl DC_LE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DC_LE_A {
        match self.bits {
            false => DC_LE_A::VALUE1,
            true => DC_LE_A::VALUE2,
        }
    }
    #[doc = "No change on DC Latch Inputs"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DC_LE_A::VALUE1
    }
    #[doc = "At least one change on DC Latch Inputs"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DC_LE_A::VALUE2
    }
}
#[doc = "Field `ST_S0` reader - State of DC SYNC0"]
pub type ST_S0_R = crate::BitReader;
#[doc = "Field `ST_S1` reader - State of DC SYNC1"]
pub type ST_S1_R = crate::BitReader;
#[doc = "SyncManager activation register changed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SM_A_A {
    #[doc = "0: No change in any SyncManager"]
    VALUE1 = 0,
    #[doc = "1: At least one change on DC Latch Inputs"]
    VALUE2 = 1,
}
impl From<SM_A_A> for bool {
    #[inline(always)]
    fn from(variant: SM_A_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SM_A` reader - SyncManager activation register changed"]
pub type SM_A_R = crate::BitReader<SM_A_A>;
impl SM_A_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SM_A_A {
        match self.bits {
            false => SM_A_A::VALUE1,
            true => SM_A_A::VALUE2,
        }
    }
    #[doc = "No change in any SyncManager"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SM_A_A::VALUE1
    }
    #[doc = "At least one change on DC Latch Inputs"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SM_A_A::VALUE2
    }
}
#[doc = "EEPROM Emulation\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEP_E_A {
    #[doc = "0: No command pending"]
    VALUE1 = 0,
    #[doc = "1: EEPROM command pending"]
    VALUE2 = 1,
}
impl From<EEP_E_A> for bool {
    #[inline(always)]
    fn from(variant: EEP_E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEP_E` reader - EEPROM Emulation"]
pub type EEP_E_R = crate::BitReader<EEP_E_A>;
impl EEP_E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EEP_E_A {
        match self.bits {
            false => EEP_E_A::VALUE1,
            true => EEP_E_A::VALUE2,
        }
    }
    #[doc = "No command pending"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EEP_E_A::VALUE1
    }
    #[doc = "EEPROM command pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EEP_E_A::VALUE2
    }
}
#[doc = "Watchdog Process Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP_D_A {
    #[doc = "0: Has not expired"]
    VALUE1 = 0,
    #[doc = "1: Has expired"]
    VALUE2 = 1,
}
impl From<WP_D_A> for bool {
    #[inline(always)]
    fn from(variant: WP_D_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_D` reader - Watchdog Process Data"]
pub type WP_D_R = crate::BitReader<WP_D_A>;
impl WP_D_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WP_D_A {
        match self.bits {
            false => WP_D_A::VALUE1,
            true => WP_D_A::VALUE2,
        }
    }
    #[doc = "Has not expired"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WP_D_A::VALUE1
    }
    #[doc = "Has expired"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WP_D_A::VALUE2
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMI_0_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2 = 1,
}
impl From<SMI_0_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_0` reader - SyncManager interrupt"]
pub type SMI_0_R = crate::BitReader<SMI_0_A>;
impl SMI_0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMI_0_A {
        match self.bits {
            false => SMI_0_A::VALUE1,
            true => SMI_0_A::VALUE2,
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_0_A::VALUE1
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_0_A::VALUE2
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMI_1_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2 = 1,
}
impl From<SMI_1_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_1` reader - SyncManager interrupt"]
pub type SMI_1_R = crate::BitReader<SMI_1_A>;
impl SMI_1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMI_1_A {
        match self.bits {
            false => SMI_1_A::VALUE1,
            true => SMI_1_A::VALUE2,
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_1_A::VALUE1
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_1_A::VALUE2
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMI_2_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2 = 1,
}
impl From<SMI_2_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_2` reader - SyncManager interrupt"]
pub type SMI_2_R = crate::BitReader<SMI_2_A>;
impl SMI_2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMI_2_A {
        match self.bits {
            false => SMI_2_A::VALUE1,
            true => SMI_2_A::VALUE2,
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_2_A::VALUE1
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_2_A::VALUE2
    }
}
#[doc = "Field `SMI_2` writer - SyncManager interrupt"]
pub type SMI_2_W<'a, REG> = crate::BitWriter<'a, REG, SMI_2_A>;
impl<'a, REG> SMI_2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_2_A::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_2_A::VALUE2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMI_3_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2 = 1,
}
impl From<SMI_3_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_3` reader - SyncManager interrupt"]
pub type SMI_3_R = crate::BitReader<SMI_3_A>;
impl SMI_3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMI_3_A {
        match self.bits {
            false => SMI_3_A::VALUE1,
            true => SMI_3_A::VALUE2,
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_3_A::VALUE1
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_3_A::VALUE2
    }
}
#[doc = "Field `SMI_3` writer - SyncManager interrupt"]
pub type SMI_3_W<'a, REG> = crate::BitWriter<'a, REG, SMI_3_A>;
impl<'a, REG> SMI_3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_3_A::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_3_A::VALUE2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMI_4_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2 = 1,
}
impl From<SMI_4_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_4` reader - SyncManager interrupt"]
pub type SMI_4_R = crate::BitReader<SMI_4_A>;
impl SMI_4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMI_4_A {
        match self.bits {
            false => SMI_4_A::VALUE1,
            true => SMI_4_A::VALUE2,
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_4_A::VALUE1
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_4_A::VALUE2
    }
}
#[doc = "Field `SMI_4` writer - SyncManager interrupt"]
pub type SMI_4_W<'a, REG> = crate::BitWriter<'a, REG, SMI_4_A>;
impl<'a, REG> SMI_4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_4_A::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_4_A::VALUE2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMI_5_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2 = 1,
}
impl From<SMI_5_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_5` reader - SyncManager interrupt"]
pub type SMI_5_R = crate::BitReader<SMI_5_A>;
impl SMI_5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMI_5_A {
        match self.bits {
            false => SMI_5_A::VALUE1,
            true => SMI_5_A::VALUE2,
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_5_A::VALUE1
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_5_A::VALUE2
    }
}
#[doc = "Field `SMI_5` writer - SyncManager interrupt"]
pub type SMI_5_W<'a, REG> = crate::BitWriter<'a, REG, SMI_5_A>;
impl<'a, REG> SMI_5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_5_A::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_5_A::VALUE2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMI_6_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2 = 1,
}
impl From<SMI_6_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_6` reader - SyncManager interrupt"]
pub type SMI_6_R = crate::BitReader<SMI_6_A>;
impl SMI_6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMI_6_A {
        match self.bits {
            false => SMI_6_A::VALUE1,
            true => SMI_6_A::VALUE2,
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_6_A::VALUE1
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_6_A::VALUE2
    }
}
#[doc = "Field `SMI_6` writer - SyncManager interrupt"]
pub type SMI_6_W<'a, REG> = crate::BitWriter<'a, REG, SMI_6_A>;
impl<'a, REG> SMI_6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_6_A::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_6_A::VALUE2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMI_7_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2 = 1,
}
impl From<SMI_7_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_7` reader - SyncManager interrupt"]
pub type SMI_7_R = crate::BitReader<SMI_7_A>;
impl SMI_7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMI_7_A {
        match self.bits {
            false => SMI_7_A::VALUE1,
            true => SMI_7_A::VALUE2,
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_7_A::VALUE1
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_7_A::VALUE2
    }
}
#[doc = "Field `SMI_7` writer - SyncManager interrupt"]
pub type SMI_7_W<'a, REG> = crate::BitWriter<'a, REG, SMI_7_A>;
impl<'a, REG> SMI_7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_7_A::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_7_A::VALUE2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMI_8_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2 = 1,
}
impl From<SMI_8_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_8` reader - SyncManager interrupt"]
pub type SMI_8_R = crate::BitReader<SMI_8_A>;
impl SMI_8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMI_8_A {
        match self.bits {
            false => SMI_8_A::VALUE1,
            true => SMI_8_A::VALUE2,
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_8_A::VALUE1
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_8_A::VALUE2
    }
}
#[doc = "Field `SMI_8` writer - SyncManager interrupt"]
pub type SMI_8_W<'a, REG> = crate::BitWriter<'a, REG, SMI_8_A>;
impl<'a, REG> SMI_8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_8_A::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_8_A::VALUE2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMI_9_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2 = 1,
}
impl From<SMI_9_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_9` reader - SyncManager interrupt"]
pub type SMI_9_R = crate::BitReader<SMI_9_A>;
impl SMI_9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMI_9_A {
        match self.bits {
            false => SMI_9_A::VALUE1,
            true => SMI_9_A::VALUE2,
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_9_A::VALUE1
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_9_A::VALUE2
    }
}
#[doc = "Field `SMI_9` writer - SyncManager interrupt"]
pub type SMI_9_W<'a, REG> = crate::BitWriter<'a, REG, SMI_9_A>;
impl<'a, REG> SMI_9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_9_A::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_9_A::VALUE2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMI_10_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2 = 1,
}
impl From<SMI_10_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_10` reader - SyncManager interrupt"]
pub type SMI_10_R = crate::BitReader<SMI_10_A>;
impl SMI_10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMI_10_A {
        match self.bits {
            false => SMI_10_A::VALUE1,
            true => SMI_10_A::VALUE2,
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_10_A::VALUE1
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_10_A::VALUE2
    }
}
#[doc = "Field `SMI_10` writer - SyncManager interrupt"]
pub type SMI_10_W<'a, REG> = crate::BitWriter<'a, REG, SMI_10_A>;
impl<'a, REG> SMI_10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_10_A::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_10_A::VALUE2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMI_11_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2 = 1,
}
impl From<SMI_11_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_11` reader - SyncManager interrupt"]
pub type SMI_11_R = crate::BitReader<SMI_11_A>;
impl SMI_11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMI_11_A {
        match self.bits {
            false => SMI_11_A::VALUE1,
            true => SMI_11_A::VALUE2,
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_11_A::VALUE1
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_11_A::VALUE2
    }
}
#[doc = "Field `SMI_11` writer - SyncManager interrupt"]
pub type SMI_11_W<'a, REG> = crate::BitWriter<'a, REG, SMI_11_A>;
impl<'a, REG> SMI_11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_11_A::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_11_A::VALUE2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMI_12_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2 = 1,
}
impl From<SMI_12_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_12` reader - SyncManager interrupt"]
pub type SMI_12_R = crate::BitReader<SMI_12_A>;
impl SMI_12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMI_12_A {
        match self.bits {
            false => SMI_12_A::VALUE1,
            true => SMI_12_A::VALUE2,
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_12_A::VALUE1
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_12_A::VALUE2
    }
}
#[doc = "Field `SMI_12` writer - SyncManager interrupt"]
pub type SMI_12_W<'a, REG> = crate::BitWriter<'a, REG, SMI_12_A>;
impl<'a, REG> SMI_12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_12_A::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_12_A::VALUE2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMI_13_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2 = 1,
}
impl From<SMI_13_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_13` reader - SyncManager interrupt"]
pub type SMI_13_R = crate::BitReader<SMI_13_A>;
impl SMI_13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMI_13_A {
        match self.bits {
            false => SMI_13_A::VALUE1,
            true => SMI_13_A::VALUE2,
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_13_A::VALUE1
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_13_A::VALUE2
    }
}
#[doc = "Field `SMI_13` writer - SyncManager interrupt"]
pub type SMI_13_W<'a, REG> = crate::BitWriter<'a, REG, SMI_13_A>;
impl<'a, REG> SMI_13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_13_A::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_13_A::VALUE2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMI_14_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2 = 1,
}
impl From<SMI_14_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_14` reader - SyncManager interrupt"]
pub type SMI_14_R = crate::BitReader<SMI_14_A>;
impl SMI_14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMI_14_A {
        match self.bits {
            false => SMI_14_A::VALUE1,
            true => SMI_14_A::VALUE2,
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_14_A::VALUE1
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_14_A::VALUE2
    }
}
#[doc = "Field `SMI_14` writer - SyncManager interrupt"]
pub type SMI_14_W<'a, REG> = crate::BitWriter<'a, REG, SMI_14_A>;
impl<'a, REG> SMI_14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_14_A::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_14_A::VALUE2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMI_15_A {
    #[doc = "0: No SyncManager 0 interrupt"]
    VALUE1 = 0,
    #[doc = "1: SyncManager 0 interrupt pending"]
    VALUE2 = 1,
}
impl From<SMI_15_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_15` reader - SyncManager interrupt"]
pub type SMI_15_R = crate::BitReader<SMI_15_A>;
impl SMI_15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMI_15_A {
        match self.bits {
            false => SMI_15_A::VALUE1,
            true => SMI_15_A::VALUE2,
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_15_A::VALUE1
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_15_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - AL Control event"]
    #[inline(always)]
    pub fn al_ce(&self) -> AL_CE_R {
        AL_CE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DC Latch event"]
    #[inline(always)]
    pub fn dc_le(&self) -> DC_LE_R {
        DC_LE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - State of DC SYNC0"]
    #[inline(always)]
    pub fn st_s0(&self) -> ST_S0_R {
        ST_S0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - State of DC SYNC1"]
    #[inline(always)]
    pub fn st_s1(&self) -> ST_S1_R {
        ST_S1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SyncManager activation register changed"]
    #[inline(always)]
    pub fn sm_a(&self) -> SM_A_R {
        SM_A_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EEPROM Emulation"]
    #[inline(always)]
    pub fn eep_e(&self) -> EEP_E_R {
        EEP_E_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Watchdog Process Data"]
    #[inline(always)]
    pub fn wp_d(&self) -> WP_D_R {
        WP_D_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_0(&self) -> SMI_0_R {
        SMI_0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_1(&self) -> SMI_1_R {
        SMI_1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_2(&self) -> SMI_2_R {
        SMI_2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_3(&self) -> SMI_3_R {
        SMI_3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_4(&self) -> SMI_4_R {
        SMI_4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_5(&self) -> SMI_5_R {
        SMI_5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_6(&self) -> SMI_6_R {
        SMI_6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_7(&self) -> SMI_7_R {
        SMI_7_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_8(&self) -> SMI_8_R {
        SMI_8_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_9(&self) -> SMI_9_R {
        SMI_9_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_10(&self) -> SMI_10_R {
        SMI_10_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_11(&self) -> SMI_11_R {
        SMI_11_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_12(&self) -> SMI_12_R {
        SMI_12_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_13(&self) -> SMI_13_R {
        SMI_13_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_14(&self) -> SMI_14_R {
        SMI_14_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_15(&self) -> SMI_15_R {
        SMI_15_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_2(&mut self) -> SMI_2_W<AL_EVENT_REQ_SPEC> {
        SMI_2_W::new(self, 10)
    }
    #[doc = "Bit 11 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_3(&mut self) -> SMI_3_W<AL_EVENT_REQ_SPEC> {
        SMI_3_W::new(self, 11)
    }
    #[doc = "Bit 12 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_4(&mut self) -> SMI_4_W<AL_EVENT_REQ_SPEC> {
        SMI_4_W::new(self, 12)
    }
    #[doc = "Bit 13 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_5(&mut self) -> SMI_5_W<AL_EVENT_REQ_SPEC> {
        SMI_5_W::new(self, 13)
    }
    #[doc = "Bit 14 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_6(&mut self) -> SMI_6_W<AL_EVENT_REQ_SPEC> {
        SMI_6_W::new(self, 14)
    }
    #[doc = "Bit 15 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_7(&mut self) -> SMI_7_W<AL_EVENT_REQ_SPEC> {
        SMI_7_W::new(self, 15)
    }
    #[doc = "Bit 16 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_8(&mut self) -> SMI_8_W<AL_EVENT_REQ_SPEC> {
        SMI_8_W::new(self, 16)
    }
    #[doc = "Bit 17 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_9(&mut self) -> SMI_9_W<AL_EVENT_REQ_SPEC> {
        SMI_9_W::new(self, 17)
    }
    #[doc = "Bit 18 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_10(&mut self) -> SMI_10_W<AL_EVENT_REQ_SPEC> {
        SMI_10_W::new(self, 18)
    }
    #[doc = "Bit 19 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_11(&mut self) -> SMI_11_W<AL_EVENT_REQ_SPEC> {
        SMI_11_W::new(self, 19)
    }
    #[doc = "Bit 20 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_12(&mut self) -> SMI_12_W<AL_EVENT_REQ_SPEC> {
        SMI_12_W::new(self, 20)
    }
    #[doc = "Bit 21 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_13(&mut self) -> SMI_13_W<AL_EVENT_REQ_SPEC> {
        SMI_13_W::new(self, 21)
    }
    #[doc = "Bit 22 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_14(&mut self) -> SMI_14_W<AL_EVENT_REQ_SPEC> {
        SMI_14_W::new(self, 22)
    }
}
#[doc = "AL Event Request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`al_event_req::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`al_event_req::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AL_EVENT_REQ_SPEC;
impl crate::RegisterSpec for AL_EVENT_REQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`al_event_req::R`](R) reader structure"]
impl crate::Readable for AL_EVENT_REQ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`al_event_req::W`](W) writer structure"]
impl crate::Writable for AL_EVENT_REQ_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AL_EVENT_REQ to value 0x20"]
impl crate::Resettable for AL_EVENT_REQ_SPEC {
    const RESET_VALUE: u32 = 0x20;
}
