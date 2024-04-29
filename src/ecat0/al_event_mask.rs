#[doc = "Register `AL_EVENT_MASK` reader"]
pub type R = crate::R<AL_EVENT_MASK_SPEC>;
#[doc = "Register `AL_EVENT_MASK` writer"]
pub type W = crate::W<AL_EVENT_MASK_SPEC>;
#[doc = "AL Control event\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AL_CE_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<AL_CE_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: AL_CE_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AL_CE_MASK` reader - AL Control event"]
pub type AL_CE_MASK_R = crate::BitReader<AL_CE_MASK_A>;
impl AL_CE_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AL_CE_MASK_A {
        match self.bits {
            false => AL_CE_MASK_A::VALUE1,
            true => AL_CE_MASK_A::VALUE2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AL_CE_MASK_A::VALUE1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AL_CE_MASK_A::VALUE2
    }
}
#[doc = "Field `AL_CE_MASK` writer - AL Control event"]
pub type AL_CE_MASK_W<'a, REG> = crate::BitWriter<'a, REG, AL_CE_MASK_A>;
impl<'a, REG> AL_CE_MASK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(AL_CE_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(AL_CE_MASK_A::VALUE2)
    }
}
#[doc = "DC Latch event\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DC_LE_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<DC_LE_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: DC_LE_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DC_LE_MASK` reader - DC Latch event"]
pub type DC_LE_MASK_R = crate::BitReader<DC_LE_MASK_A>;
impl DC_LE_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DC_LE_MASK_A {
        match self.bits {
            false => DC_LE_MASK_A::VALUE1,
            true => DC_LE_MASK_A::VALUE2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DC_LE_MASK_A::VALUE1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DC_LE_MASK_A::VALUE2
    }
}
#[doc = "Field `DC_LE_MASK` writer - DC Latch event"]
pub type DC_LE_MASK_W<'a, REG> = crate::BitWriter<'a, REG, DC_LE_MASK_A>;
impl<'a, REG> DC_LE_MASK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DC_LE_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DC_LE_MASK_A::VALUE2)
    }
}
#[doc = "State of DC SYNC0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ST_S0_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<ST_S0_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: ST_S0_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ST_S0_MASK` reader - State of DC SYNC0"]
pub type ST_S0_MASK_R = crate::BitReader<ST_S0_MASK_A>;
impl ST_S0_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ST_S0_MASK_A {
        match self.bits {
            false => ST_S0_MASK_A::VALUE1,
            true => ST_S0_MASK_A::VALUE2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ST_S0_MASK_A::VALUE1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ST_S0_MASK_A::VALUE2
    }
}
#[doc = "Field `ST_S0_MASK` writer - State of DC SYNC0"]
pub type ST_S0_MASK_W<'a, REG> = crate::BitWriter<'a, REG, ST_S0_MASK_A>;
impl<'a, REG> ST_S0_MASK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ST_S0_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ST_S0_MASK_A::VALUE2)
    }
}
#[doc = "State of DC SYNC1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ST_S1_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<ST_S1_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: ST_S1_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ST_S1_MASK` reader - State of DC SYNC1"]
pub type ST_S1_MASK_R = crate::BitReader<ST_S1_MASK_A>;
impl ST_S1_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ST_S1_MASK_A {
        match self.bits {
            false => ST_S1_MASK_A::VALUE1,
            true => ST_S1_MASK_A::VALUE2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ST_S1_MASK_A::VALUE1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ST_S1_MASK_A::VALUE2
    }
}
#[doc = "Field `ST_S1_MASK` writer - State of DC SYNC1"]
pub type ST_S1_MASK_W<'a, REG> = crate::BitWriter<'a, REG, ST_S1_MASK_A>;
impl<'a, REG> ST_S1_MASK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ST_S1_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ST_S1_MASK_A::VALUE2)
    }
}
#[doc = "SyncManager activation register changed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SM_A_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<SM_A_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: SM_A_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SM_A_MASK` reader - SyncManager activation register changed"]
pub type SM_A_MASK_R = crate::BitReader<SM_A_MASK_A>;
impl SM_A_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SM_A_MASK_A {
        match self.bits {
            false => SM_A_MASK_A::VALUE1,
            true => SM_A_MASK_A::VALUE2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SM_A_MASK_A::VALUE1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SM_A_MASK_A::VALUE2
    }
}
#[doc = "Field `SM_A_MASK` writer - SyncManager activation register changed"]
pub type SM_A_MASK_W<'a, REG> = crate::BitWriter<'a, REG, SM_A_MASK_A>;
impl<'a, REG> SM_A_MASK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SM_A_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SM_A_MASK_A::VALUE2)
    }
}
#[doc = "EEPROM Emulation\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEP_E_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<EEP_E_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: EEP_E_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEP_E_MASK` reader - EEPROM Emulation"]
pub type EEP_E_MASK_R = crate::BitReader<EEP_E_MASK_A>;
impl EEP_E_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EEP_E_MASK_A {
        match self.bits {
            false => EEP_E_MASK_A::VALUE1,
            true => EEP_E_MASK_A::VALUE2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EEP_E_MASK_A::VALUE1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EEP_E_MASK_A::VALUE2
    }
}
#[doc = "Field `EEP_E_MASK` writer - EEPROM Emulation"]
pub type EEP_E_MASK_W<'a, REG> = crate::BitWriter<'a, REG, EEP_E_MASK_A>;
impl<'a, REG> EEP_E_MASK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EEP_E_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EEP_E_MASK_A::VALUE2)
    }
}
#[doc = "Watchdog Process Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP_D_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<WP_D_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: WP_D_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP_D_MASK` reader - Watchdog Process Data"]
pub type WP_D_MASK_R = crate::BitReader<WP_D_MASK_A>;
impl WP_D_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WP_D_MASK_A {
        match self.bits {
            false => WP_D_MASK_A::VALUE1,
            true => WP_D_MASK_A::VALUE2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WP_D_MASK_A::VALUE1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WP_D_MASK_A::VALUE2
    }
}
#[doc = "Field `WP_D_MASK` writer - Watchdog Process Data"]
pub type WP_D_MASK_W<'a, REG> = crate::BitWriter<'a, REG, WP_D_MASK_A>;
impl<'a, REG> WP_D_MASK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WP_D_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WP_D_MASK_A::VALUE2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMI_0_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<SMI_0_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_0_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_0_MASK` reader - SyncManager interrupt"]
pub type SMI_0_MASK_R = crate::BitReader<SMI_0_MASK_A>;
impl SMI_0_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMI_0_MASK_A {
        match self.bits {
            false => SMI_0_MASK_A::VALUE1,
            true => SMI_0_MASK_A::VALUE2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_0_MASK_A::VALUE1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_0_MASK_A::VALUE2
    }
}
#[doc = "Field `SMI_0_MASK` writer - SyncManager interrupt"]
pub type SMI_0_MASK_W<'a, REG> = crate::BitWriter<'a, REG, SMI_0_MASK_A>;
impl<'a, REG> SMI_0_MASK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_0_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_0_MASK_A::VALUE2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMI_1_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<SMI_1_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_1_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_1_MASK` reader - SyncManager interrupt"]
pub type SMI_1_MASK_R = crate::BitReader<SMI_1_MASK_A>;
impl SMI_1_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMI_1_MASK_A {
        match self.bits {
            false => SMI_1_MASK_A::VALUE1,
            true => SMI_1_MASK_A::VALUE2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_1_MASK_A::VALUE1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_1_MASK_A::VALUE2
    }
}
#[doc = "Field `SMI_1_MASK` writer - SyncManager interrupt"]
pub type SMI_1_MASK_W<'a, REG> = crate::BitWriter<'a, REG, SMI_1_MASK_A>;
impl<'a, REG> SMI_1_MASK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_1_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_1_MASK_A::VALUE2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMI_2_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<SMI_2_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_2_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_2_MASK` reader - SyncManager interrupt"]
pub type SMI_2_MASK_R = crate::BitReader<SMI_2_MASK_A>;
impl SMI_2_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMI_2_MASK_A {
        match self.bits {
            false => SMI_2_MASK_A::VALUE1,
            true => SMI_2_MASK_A::VALUE2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_2_MASK_A::VALUE1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_2_MASK_A::VALUE2
    }
}
#[doc = "Field `SMI_2_MASK` writer - SyncManager interrupt"]
pub type SMI_2_MASK_W<'a, REG> = crate::BitWriter<'a, REG, SMI_2_MASK_A>;
impl<'a, REG> SMI_2_MASK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_2_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_2_MASK_A::VALUE2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMI_3_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<SMI_3_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_3_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_3_MASK` reader - SyncManager interrupt"]
pub type SMI_3_MASK_R = crate::BitReader<SMI_3_MASK_A>;
impl SMI_3_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMI_3_MASK_A {
        match self.bits {
            false => SMI_3_MASK_A::VALUE1,
            true => SMI_3_MASK_A::VALUE2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_3_MASK_A::VALUE1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_3_MASK_A::VALUE2
    }
}
#[doc = "Field `SMI_3_MASK` writer - SyncManager interrupt"]
pub type SMI_3_MASK_W<'a, REG> = crate::BitWriter<'a, REG, SMI_3_MASK_A>;
impl<'a, REG> SMI_3_MASK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_3_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_3_MASK_A::VALUE2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMI_4_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<SMI_4_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_4_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_4_MASK` reader - SyncManager interrupt"]
pub type SMI_4_MASK_R = crate::BitReader<SMI_4_MASK_A>;
impl SMI_4_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMI_4_MASK_A {
        match self.bits {
            false => SMI_4_MASK_A::VALUE1,
            true => SMI_4_MASK_A::VALUE2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_4_MASK_A::VALUE1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_4_MASK_A::VALUE2
    }
}
#[doc = "Field `SMI_4_MASK` writer - SyncManager interrupt"]
pub type SMI_4_MASK_W<'a, REG> = crate::BitWriter<'a, REG, SMI_4_MASK_A>;
impl<'a, REG> SMI_4_MASK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_4_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_4_MASK_A::VALUE2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMI_5_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<SMI_5_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_5_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_5_MASK` reader - SyncManager interrupt"]
pub type SMI_5_MASK_R = crate::BitReader<SMI_5_MASK_A>;
impl SMI_5_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMI_5_MASK_A {
        match self.bits {
            false => SMI_5_MASK_A::VALUE1,
            true => SMI_5_MASK_A::VALUE2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_5_MASK_A::VALUE1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_5_MASK_A::VALUE2
    }
}
#[doc = "Field `SMI_5_MASK` writer - SyncManager interrupt"]
pub type SMI_5_MASK_W<'a, REG> = crate::BitWriter<'a, REG, SMI_5_MASK_A>;
impl<'a, REG> SMI_5_MASK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_5_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_5_MASK_A::VALUE2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMI_6_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<SMI_6_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_6_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_6_MASK` reader - SyncManager interrupt"]
pub type SMI_6_MASK_R = crate::BitReader<SMI_6_MASK_A>;
impl SMI_6_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMI_6_MASK_A {
        match self.bits {
            false => SMI_6_MASK_A::VALUE1,
            true => SMI_6_MASK_A::VALUE2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_6_MASK_A::VALUE1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_6_MASK_A::VALUE2
    }
}
#[doc = "Field `SMI_6_MASK` writer - SyncManager interrupt"]
pub type SMI_6_MASK_W<'a, REG> = crate::BitWriter<'a, REG, SMI_6_MASK_A>;
impl<'a, REG> SMI_6_MASK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_6_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_6_MASK_A::VALUE2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMI_7_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<SMI_7_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_7_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_7_MASK` reader - SyncManager interrupt"]
pub type SMI_7_MASK_R = crate::BitReader<SMI_7_MASK_A>;
impl SMI_7_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMI_7_MASK_A {
        match self.bits {
            false => SMI_7_MASK_A::VALUE1,
            true => SMI_7_MASK_A::VALUE2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_7_MASK_A::VALUE1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_7_MASK_A::VALUE2
    }
}
#[doc = "Field `SMI_7_MASK` writer - SyncManager interrupt"]
pub type SMI_7_MASK_W<'a, REG> = crate::BitWriter<'a, REG, SMI_7_MASK_A>;
impl<'a, REG> SMI_7_MASK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_7_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_7_MASK_A::VALUE2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMI_8_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<SMI_8_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_8_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_8_MASK` reader - SyncManager interrupt"]
pub type SMI_8_MASK_R = crate::BitReader<SMI_8_MASK_A>;
impl SMI_8_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMI_8_MASK_A {
        match self.bits {
            false => SMI_8_MASK_A::VALUE1,
            true => SMI_8_MASK_A::VALUE2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_8_MASK_A::VALUE1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_8_MASK_A::VALUE2
    }
}
#[doc = "Field `SMI_8_MASK` writer - SyncManager interrupt"]
pub type SMI_8_MASK_W<'a, REG> = crate::BitWriter<'a, REG, SMI_8_MASK_A>;
impl<'a, REG> SMI_8_MASK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_8_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_8_MASK_A::VALUE2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMI_9_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<SMI_9_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_9_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_9_MASK` reader - SyncManager interrupt"]
pub type SMI_9_MASK_R = crate::BitReader<SMI_9_MASK_A>;
impl SMI_9_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMI_9_MASK_A {
        match self.bits {
            false => SMI_9_MASK_A::VALUE1,
            true => SMI_9_MASK_A::VALUE2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_9_MASK_A::VALUE1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_9_MASK_A::VALUE2
    }
}
#[doc = "Field `SMI_9_MASK` writer - SyncManager interrupt"]
pub type SMI_9_MASK_W<'a, REG> = crate::BitWriter<'a, REG, SMI_9_MASK_A>;
impl<'a, REG> SMI_9_MASK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_9_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_9_MASK_A::VALUE2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMI_10_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<SMI_10_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_10_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_10_MASK` reader - SyncManager interrupt"]
pub type SMI_10_MASK_R = crate::BitReader<SMI_10_MASK_A>;
impl SMI_10_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMI_10_MASK_A {
        match self.bits {
            false => SMI_10_MASK_A::VALUE1,
            true => SMI_10_MASK_A::VALUE2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_10_MASK_A::VALUE1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_10_MASK_A::VALUE2
    }
}
#[doc = "Field `SMI_10_MASK` writer - SyncManager interrupt"]
pub type SMI_10_MASK_W<'a, REG> = crate::BitWriter<'a, REG, SMI_10_MASK_A>;
impl<'a, REG> SMI_10_MASK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_10_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_10_MASK_A::VALUE2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMI_11_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<SMI_11_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_11_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_11_MASK` reader - SyncManager interrupt"]
pub type SMI_11_MASK_R = crate::BitReader<SMI_11_MASK_A>;
impl SMI_11_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMI_11_MASK_A {
        match self.bits {
            false => SMI_11_MASK_A::VALUE1,
            true => SMI_11_MASK_A::VALUE2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_11_MASK_A::VALUE1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_11_MASK_A::VALUE2
    }
}
#[doc = "Field `SMI_11_MASK` writer - SyncManager interrupt"]
pub type SMI_11_MASK_W<'a, REG> = crate::BitWriter<'a, REG, SMI_11_MASK_A>;
impl<'a, REG> SMI_11_MASK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_11_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_11_MASK_A::VALUE2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMI_12_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<SMI_12_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_12_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_12_MASK` reader - SyncManager interrupt"]
pub type SMI_12_MASK_R = crate::BitReader<SMI_12_MASK_A>;
impl SMI_12_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMI_12_MASK_A {
        match self.bits {
            false => SMI_12_MASK_A::VALUE1,
            true => SMI_12_MASK_A::VALUE2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_12_MASK_A::VALUE1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_12_MASK_A::VALUE2
    }
}
#[doc = "Field `SMI_12_MASK` writer - SyncManager interrupt"]
pub type SMI_12_MASK_W<'a, REG> = crate::BitWriter<'a, REG, SMI_12_MASK_A>;
impl<'a, REG> SMI_12_MASK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_12_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_12_MASK_A::VALUE2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMI_13_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<SMI_13_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_13_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_13_MASK` reader - SyncManager interrupt"]
pub type SMI_13_MASK_R = crate::BitReader<SMI_13_MASK_A>;
impl SMI_13_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMI_13_MASK_A {
        match self.bits {
            false => SMI_13_MASK_A::VALUE1,
            true => SMI_13_MASK_A::VALUE2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_13_MASK_A::VALUE1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_13_MASK_A::VALUE2
    }
}
#[doc = "Field `SMI_13_MASK` writer - SyncManager interrupt"]
pub type SMI_13_MASK_W<'a, REG> = crate::BitWriter<'a, REG, SMI_13_MASK_A>;
impl<'a, REG> SMI_13_MASK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_13_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_13_MASK_A::VALUE2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMI_14_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<SMI_14_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_14_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_14_MASK` reader - SyncManager interrupt"]
pub type SMI_14_MASK_R = crate::BitReader<SMI_14_MASK_A>;
impl SMI_14_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMI_14_MASK_A {
        match self.bits {
            false => SMI_14_MASK_A::VALUE1,
            true => SMI_14_MASK_A::VALUE2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_14_MASK_A::VALUE1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_14_MASK_A::VALUE2
    }
}
#[doc = "Field `SMI_14_MASK` writer - SyncManager interrupt"]
pub type SMI_14_MASK_W<'a, REG> = crate::BitWriter<'a, REG, SMI_14_MASK_A>;
impl<'a, REG> SMI_14_MASK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_14_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_14_MASK_A::VALUE2)
    }
}
#[doc = "SyncManager interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMI_15_MASK_A {
    #[doc = "0: Corresponding AL Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding AL Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<SMI_15_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_15_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI_15_MASK` reader - SyncManager interrupt"]
pub type SMI_15_MASK_R = crate::BitReader<SMI_15_MASK_A>;
impl SMI_15_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMI_15_MASK_A {
        match self.bits {
            false => SMI_15_MASK_A::VALUE1,
            true => SMI_15_MASK_A::VALUE2,
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SMI_15_MASK_A::VALUE1
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SMI_15_MASK_A::VALUE2
    }
}
#[doc = "Field `SMI_15_MASK` writer - SyncManager interrupt"]
pub type SMI_15_MASK_W<'a, REG> = crate::BitWriter<'a, REG, SMI_15_MASK_A>;
impl<'a, REG> SMI_15_MASK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_15_MASK_A::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_15_MASK_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - AL Control event"]
    #[inline(always)]
    pub fn al_ce_mask(&self) -> AL_CE_MASK_R {
        AL_CE_MASK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DC Latch event"]
    #[inline(always)]
    pub fn dc_le_mask(&self) -> DC_LE_MASK_R {
        DC_LE_MASK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - State of DC SYNC0"]
    #[inline(always)]
    pub fn st_s0_mask(&self) -> ST_S0_MASK_R {
        ST_S0_MASK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - State of DC SYNC1"]
    #[inline(always)]
    pub fn st_s1_mask(&self) -> ST_S1_MASK_R {
        ST_S1_MASK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SyncManager activation register changed"]
    #[inline(always)]
    pub fn sm_a_mask(&self) -> SM_A_MASK_R {
        SM_A_MASK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EEPROM Emulation"]
    #[inline(always)]
    pub fn eep_e_mask(&self) -> EEP_E_MASK_R {
        EEP_E_MASK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Watchdog Process Data"]
    #[inline(always)]
    pub fn wp_d_mask(&self) -> WP_D_MASK_R {
        WP_D_MASK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_0_mask(&self) -> SMI_0_MASK_R {
        SMI_0_MASK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_1_mask(&self) -> SMI_1_MASK_R {
        SMI_1_MASK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_2_mask(&self) -> SMI_2_MASK_R {
        SMI_2_MASK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_3_mask(&self) -> SMI_3_MASK_R {
        SMI_3_MASK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_4_mask(&self) -> SMI_4_MASK_R {
        SMI_4_MASK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_5_mask(&self) -> SMI_5_MASK_R {
        SMI_5_MASK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_6_mask(&self) -> SMI_6_MASK_R {
        SMI_6_MASK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_7_mask(&self) -> SMI_7_MASK_R {
        SMI_7_MASK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_8_mask(&self) -> SMI_8_MASK_R {
        SMI_8_MASK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_9_mask(&self) -> SMI_9_MASK_R {
        SMI_9_MASK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_10_mask(&self) -> SMI_10_MASK_R {
        SMI_10_MASK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_11_mask(&self) -> SMI_11_MASK_R {
        SMI_11_MASK_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_12_mask(&self) -> SMI_12_MASK_R {
        SMI_12_MASK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_13_mask(&self) -> SMI_13_MASK_R {
        SMI_13_MASK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_14_mask(&self) -> SMI_14_MASK_R {
        SMI_14_MASK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SyncManager interrupt"]
    #[inline(always)]
    pub fn smi_15_mask(&self) -> SMI_15_MASK_R {
        SMI_15_MASK_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AL Control event"]
    #[inline(always)]
    #[must_use]
    pub fn al_ce_mask(&mut self) -> AL_CE_MASK_W<AL_EVENT_MASK_SPEC> {
        AL_CE_MASK_W::new(self, 0)
    }
    #[doc = "Bit 1 - DC Latch event"]
    #[inline(always)]
    #[must_use]
    pub fn dc_le_mask(&mut self) -> DC_LE_MASK_W<AL_EVENT_MASK_SPEC> {
        DC_LE_MASK_W::new(self, 1)
    }
    #[doc = "Bit 2 - State of DC SYNC0"]
    #[inline(always)]
    #[must_use]
    pub fn st_s0_mask(&mut self) -> ST_S0_MASK_W<AL_EVENT_MASK_SPEC> {
        ST_S0_MASK_W::new(self, 2)
    }
    #[doc = "Bit 3 - State of DC SYNC1"]
    #[inline(always)]
    #[must_use]
    pub fn st_s1_mask(&mut self) -> ST_S1_MASK_W<AL_EVENT_MASK_SPEC> {
        ST_S1_MASK_W::new(self, 3)
    }
    #[doc = "Bit 4 - SyncManager activation register changed"]
    #[inline(always)]
    #[must_use]
    pub fn sm_a_mask(&mut self) -> SM_A_MASK_W<AL_EVENT_MASK_SPEC> {
        SM_A_MASK_W::new(self, 4)
    }
    #[doc = "Bit 5 - EEPROM Emulation"]
    #[inline(always)]
    #[must_use]
    pub fn eep_e_mask(&mut self) -> EEP_E_MASK_W<AL_EVENT_MASK_SPEC> {
        EEP_E_MASK_W::new(self, 5)
    }
    #[doc = "Bit 6 - Watchdog Process Data"]
    #[inline(always)]
    #[must_use]
    pub fn wp_d_mask(&mut self) -> WP_D_MASK_W<AL_EVENT_MASK_SPEC> {
        WP_D_MASK_W::new(self, 6)
    }
    #[doc = "Bit 8 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_0_mask(&mut self) -> SMI_0_MASK_W<AL_EVENT_MASK_SPEC> {
        SMI_0_MASK_W::new(self, 8)
    }
    #[doc = "Bit 9 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_1_mask(&mut self) -> SMI_1_MASK_W<AL_EVENT_MASK_SPEC> {
        SMI_1_MASK_W::new(self, 9)
    }
    #[doc = "Bit 10 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_2_mask(&mut self) -> SMI_2_MASK_W<AL_EVENT_MASK_SPEC> {
        SMI_2_MASK_W::new(self, 10)
    }
    #[doc = "Bit 11 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_3_mask(&mut self) -> SMI_3_MASK_W<AL_EVENT_MASK_SPEC> {
        SMI_3_MASK_W::new(self, 11)
    }
    #[doc = "Bit 12 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_4_mask(&mut self) -> SMI_4_MASK_W<AL_EVENT_MASK_SPEC> {
        SMI_4_MASK_W::new(self, 12)
    }
    #[doc = "Bit 13 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_5_mask(&mut self) -> SMI_5_MASK_W<AL_EVENT_MASK_SPEC> {
        SMI_5_MASK_W::new(self, 13)
    }
    #[doc = "Bit 14 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_6_mask(&mut self) -> SMI_6_MASK_W<AL_EVENT_MASK_SPEC> {
        SMI_6_MASK_W::new(self, 14)
    }
    #[doc = "Bit 15 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_7_mask(&mut self) -> SMI_7_MASK_W<AL_EVENT_MASK_SPEC> {
        SMI_7_MASK_W::new(self, 15)
    }
    #[doc = "Bit 16 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_8_mask(&mut self) -> SMI_8_MASK_W<AL_EVENT_MASK_SPEC> {
        SMI_8_MASK_W::new(self, 16)
    }
    #[doc = "Bit 17 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_9_mask(&mut self) -> SMI_9_MASK_W<AL_EVENT_MASK_SPEC> {
        SMI_9_MASK_W::new(self, 17)
    }
    #[doc = "Bit 18 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_10_mask(&mut self) -> SMI_10_MASK_W<AL_EVENT_MASK_SPEC> {
        SMI_10_MASK_W::new(self, 18)
    }
    #[doc = "Bit 19 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_11_mask(&mut self) -> SMI_11_MASK_W<AL_EVENT_MASK_SPEC> {
        SMI_11_MASK_W::new(self, 19)
    }
    #[doc = "Bit 20 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_12_mask(&mut self) -> SMI_12_MASK_W<AL_EVENT_MASK_SPEC> {
        SMI_12_MASK_W::new(self, 20)
    }
    #[doc = "Bit 21 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_13_mask(&mut self) -> SMI_13_MASK_W<AL_EVENT_MASK_SPEC> {
        SMI_13_MASK_W::new(self, 21)
    }
    #[doc = "Bit 22 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_14_mask(&mut self) -> SMI_14_MASK_W<AL_EVENT_MASK_SPEC> {
        SMI_14_MASK_W::new(self, 22)
    }
    #[doc = "Bit 23 - SyncManager interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn smi_15_mask(&mut self) -> SMI_15_MASK_W<AL_EVENT_MASK_SPEC> {
        SMI_15_MASK_W::new(self, 23)
    }
}
#[doc = "PDI AL Event Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`al_event_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`al_event_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AL_EVENT_MASK_SPEC;
impl crate::RegisterSpec for AL_EVENT_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`al_event_mask::R`](R) reader structure"]
impl crate::Readable for AL_EVENT_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`al_event_mask::W`](W) writer structure"]
impl crate::Writable for AL_EVENT_MASK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AL_EVENT_MASK to value 0x00ff_ff2f"]
impl crate::Resettable for AL_EVENT_MASK_SPEC {
    const RESET_VALUE: u32 = 0x00ff_ff2f;
}
