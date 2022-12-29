#[doc = "Register `FSR` reader"]
pub struct R(crate::R<FSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PBUSY` reader - Program Flash Busy"]
pub type PBUSY_R = crate::BitReader<PBUSY_A>;
#[doc = "Program Flash Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PBUSY_A {
    #[doc = "0: PFLASH ready, not busy; PFLASH in read mode."]
    VALUE1 = 0,
    #[doc = "1: PFLASH busy; PFLASH not in read mode."]
    VALUE2 = 1,
}
impl From<PBUSY_A> for bool {
    #[inline(always)]
    fn from(variant: PBUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl PBUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PBUSY_A {
        match self.bits {
            false => PBUSY_A::VALUE1,
            true => PBUSY_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PBUSY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PBUSY_A::VALUE2
    }
}
#[doc = "Field `FABUSY` reader - Flash Array Busy"]
pub type FABUSY_R = crate::BitReader<bool>;
#[doc = "Field `PROG` reader - Programming State"]
pub type PROG_R = crate::BitReader<PROG_A>;
#[doc = "Programming State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROG_A {
    #[doc = "0: There is no program operation requested or in progress or just finished."]
    VALUE1 = 0,
    #[doc = "1: Programming operation (write page) requested (from FIM) or in action or finished."]
    VALUE2 = 1,
}
impl From<PROG_A> for bool {
    #[inline(always)]
    fn from(variant: PROG_A) -> Self {
        variant as u8 != 0
    }
}
impl PROG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROG_A {
        match self.bits {
            false => PROG_A::VALUE1,
            true => PROG_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PROG_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PROG_A::VALUE2
    }
}
#[doc = "Field `ERASE` reader - Erase State"]
pub type ERASE_R = crate::BitReader<ERASE_A>;
#[doc = "Erase State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERASE_A {
    #[doc = "0: There is no erase operation requested or in progress or just finished"]
    VALUE1 = 0,
    #[doc = "1: Erase operation requested (from FIM) or in action or finished."]
    VALUE2 = 1,
}
impl From<ERASE_A> for bool {
    #[inline(always)]
    fn from(variant: ERASE_A) -> Self {
        variant as u8 != 0
    }
}
impl ERASE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERASE_A {
        match self.bits {
            false => ERASE_A::VALUE1,
            true => ERASE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ERASE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ERASE_A::VALUE2
    }
}
#[doc = "Field `PFPAGE` reader - Program Flash in Page Mode"]
pub type PFPAGE_R = crate::BitReader<PFPAGE_A>;
#[doc = "Program Flash in Page Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFPAGE_A {
    #[doc = "0: Program Flash not in page mode"]
    VALUE1 = 0,
    #[doc = "1: Program Flash in page mode; assembly buffer of PFLASH (256 byte) is in use (being filled up)"]
    VALUE2 = 1,
}
impl From<PFPAGE_A> for bool {
    #[inline(always)]
    fn from(variant: PFPAGE_A) -> Self {
        variant as u8 != 0
    }
}
impl PFPAGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFPAGE_A {
        match self.bits {
            false => PFPAGE_A::VALUE1,
            true => PFPAGE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PFPAGE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PFPAGE_A::VALUE2
    }
}
#[doc = "Field `PFOPER` reader - Program Flash Operation Error"]
pub type PFOPER_R = crate::BitReader<PFOPER_A>;
#[doc = "Program Flash Operation Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFOPER_A {
    #[doc = "0: No operation error reported by Program Flash"]
    VALUE1 = 0,
    #[doc = "1: Flash array operation aborted, because of a Flash array failure, e.g. an ECC error in microcode."]
    VALUE2 = 1,
}
impl From<PFOPER_A> for bool {
    #[inline(always)]
    fn from(variant: PFOPER_A) -> Self {
        variant as u8 != 0
    }
}
impl PFOPER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFOPER_A {
        match self.bits {
            false => PFOPER_A::VALUE1,
            true => PFOPER_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PFOPER_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PFOPER_A::VALUE2
    }
}
#[doc = "Field `SQER` reader - Command Sequence Error"]
pub type SQER_R = crate::BitReader<SQER_A>;
#[doc = "Command Sequence Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SQER_A {
    #[doc = "0: No sequence error"]
    VALUE1 = 0,
    #[doc = "1: Command state machine operation unsuccessful because of improper address or command sequence."]
    VALUE2 = 1,
}
impl From<SQER_A> for bool {
    #[inline(always)]
    fn from(variant: SQER_A) -> Self {
        variant as u8 != 0
    }
}
impl SQER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SQER_A {
        match self.bits {
            false => SQER_A::VALUE1,
            true => SQER_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SQER_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SQER_A::VALUE2
    }
}
#[doc = "Field `PROER` reader - Protection Error"]
pub type PROER_R = crate::BitReader<PROER_A>;
#[doc = "Protection Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROER_A {
    #[doc = "0: No protection error"]
    VALUE1 = 0,
    #[doc = "1: Protection error."]
    VALUE2 = 1,
}
impl From<PROER_A> for bool {
    #[inline(always)]
    fn from(variant: PROER_A) -> Self {
        variant as u8 != 0
    }
}
impl PROER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROER_A {
        match self.bits {
            false => PROER_A::VALUE1,
            true => PROER_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PROER_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PROER_A::VALUE2
    }
}
#[doc = "Field `PFSBER` reader - PFLASH Single-Bit Error and Correction"]
pub type PFSBER_R = crate::BitReader<PFSBER_A>;
#[doc = "PFLASH Single-Bit Error and Correction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFSBER_A {
    #[doc = "0: No Single-Bit Error detected during read access to PFLASH"]
    VALUE1 = 0,
    #[doc = "1: Single-Bit Error detected and corrected"]
    VALUE2 = 1,
}
impl From<PFSBER_A> for bool {
    #[inline(always)]
    fn from(variant: PFSBER_A) -> Self {
        variant as u8 != 0
    }
}
impl PFSBER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFSBER_A {
        match self.bits {
            false => PFSBER_A::VALUE1,
            true => PFSBER_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PFSBER_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PFSBER_A::VALUE2
    }
}
#[doc = "Field `PFDBER` reader - PFLASH Double-Bit Error"]
pub type PFDBER_R = crate::BitReader<PFDBER_A>;
#[doc = "PFLASH Double-Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFDBER_A {
    #[doc = "0: No Double-Bit Error detected during read access to PFLASH"]
    VALUE1 = 0,
    #[doc = "1: Double-Bit Error detected in PFLASH"]
    VALUE2 = 1,
}
impl From<PFDBER_A> for bool {
    #[inline(always)]
    fn from(variant: PFDBER_A) -> Self {
        variant as u8 != 0
    }
}
impl PFDBER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFDBER_A {
        match self.bits {
            false => PFDBER_A::VALUE1,
            true => PFDBER_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PFDBER_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PFDBER_A::VALUE2
    }
}
#[doc = "Field `PROIN` reader - Protection Installed"]
pub type PROIN_R = crate::BitReader<PROIN_A>;
#[doc = "Protection Installed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROIN_A {
    #[doc = "0: No protection is installed"]
    VALUE1 = 0,
    #[doc = "1: Read or/and write protection for one or more users is configured and correctly confirmed in the User Configuration Block(s)."]
    VALUE2 = 1,
}
impl From<PROIN_A> for bool {
    #[inline(always)]
    fn from(variant: PROIN_A) -> Self {
        variant as u8 != 0
    }
}
impl PROIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROIN_A {
        match self.bits {
            false => PROIN_A::VALUE1,
            true => PROIN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PROIN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PROIN_A::VALUE2
    }
}
#[doc = "Field `RPROIN` reader - Read Protection Installed"]
pub type RPROIN_R = crate::BitReader<RPROIN_A>;
#[doc = "Read Protection Installed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPROIN_A {
    #[doc = "0: No read protection installed"]
    VALUE1 = 0,
    #[doc = "1: Read protection and global write protection is configured and correctly confirmed in the User Configuration Block 0."]
    VALUE2 = 1,
}
impl From<RPROIN_A> for bool {
    #[inline(always)]
    fn from(variant: RPROIN_A) -> Self {
        variant as u8 != 0
    }
}
impl RPROIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPROIN_A {
        match self.bits {
            false => RPROIN_A::VALUE1,
            true => RPROIN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RPROIN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RPROIN_A::VALUE2
    }
}
#[doc = "Field `RPRODIS` reader - Read Protection Disable State"]
pub type RPRODIS_R = crate::BitReader<RPRODIS_A>;
#[doc = "Read Protection Disable State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPRODIS_A {
    #[doc = "0: Read protection (if installed) is not disabled"]
    VALUE1 = 0,
    #[doc = "1: Read and global write protection is temporarily disabled."]
    VALUE2 = 1,
}
impl From<RPRODIS_A> for bool {
    #[inline(always)]
    fn from(variant: RPRODIS_A) -> Self {
        variant as u8 != 0
    }
}
impl RPRODIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPRODIS_A {
        match self.bits {
            false => RPRODIS_A::VALUE1,
            true => RPRODIS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RPRODIS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RPRODIS_A::VALUE2
    }
}
#[doc = "Field `WPROIN0` reader - Sector Write Protection Installed for User 0"]
pub type WPROIN0_R = crate::BitReader<WPROIN0_A>;
#[doc = "Sector Write Protection Installed for User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPROIN0_A {
    #[doc = "0: No write protection installed for user 0"]
    VALUE1 = 0,
    #[doc = "1: Sector write protection for user 0 is configured and correctly confirmed in the User Configuration Block 0."]
    VALUE2 = 1,
}
impl From<WPROIN0_A> for bool {
    #[inline(always)]
    fn from(variant: WPROIN0_A) -> Self {
        variant as u8 != 0
    }
}
impl WPROIN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPROIN0_A {
        match self.bits {
            false => WPROIN0_A::VALUE1,
            true => WPROIN0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WPROIN0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WPROIN0_A::VALUE2
    }
}
#[doc = "Field `WPROIN1` reader - Sector Write Protection Installed for User 1"]
pub type WPROIN1_R = crate::BitReader<WPROIN1_A>;
#[doc = "Sector Write Protection Installed for User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPROIN1_A {
    #[doc = "0: No write protection installed for user 1"]
    VALUE1 = 0,
    #[doc = "1: Sector write protection for user 1 is configured and correctly confirmed in the User Configuration Block 1."]
    VALUE2 = 1,
}
impl From<WPROIN1_A> for bool {
    #[inline(always)]
    fn from(variant: WPROIN1_A) -> Self {
        variant as u8 != 0
    }
}
impl WPROIN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPROIN1_A {
        match self.bits {
            false => WPROIN1_A::VALUE1,
            true => WPROIN1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WPROIN1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WPROIN1_A::VALUE2
    }
}
#[doc = "Field `WPROIN2` reader - Sector OTP Protection Installed for User 2"]
pub type WPROIN2_R = crate::BitReader<WPROIN2_A>;
#[doc = "Sector OTP Protection Installed for User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPROIN2_A {
    #[doc = "0: No OTP write protection installed for user 2"]
    VALUE1 = 0,
    #[doc = "1: Sector OTP write protection with ROM functionality is configured and correctly confirmed in the UCB2. The protection is locked for ever."]
    VALUE2 = 1,
}
impl From<WPROIN2_A> for bool {
    #[inline(always)]
    fn from(variant: WPROIN2_A) -> Self {
        variant as u8 != 0
    }
}
impl WPROIN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPROIN2_A {
        match self.bits {
            false => WPROIN2_A::VALUE1,
            true => WPROIN2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WPROIN2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WPROIN2_A::VALUE2
    }
}
#[doc = "Field `WPRODIS0` reader - Sector Write Protection Disabled for User 0"]
pub type WPRODIS0_R = crate::BitReader<WPRODIS0_A>;
#[doc = "Sector Write Protection Disabled for User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPRODIS0_A {
    #[doc = "0: All protected sectors of user 0 are locked if write protection is installed"]
    VALUE1 = 0,
    #[doc = "1: All write-protected sectors of user 0 are temporarily unlocked, if not coincidently locked by user 2 or via read protection."]
    VALUE2 = 1,
}
impl From<WPRODIS0_A> for bool {
    #[inline(always)]
    fn from(variant: WPRODIS0_A) -> Self {
        variant as u8 != 0
    }
}
impl WPRODIS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPRODIS0_A {
        match self.bits {
            false => WPRODIS0_A::VALUE1,
            true => WPRODIS0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WPRODIS0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WPRODIS0_A::VALUE2
    }
}
#[doc = "Field `WPRODIS1` reader - Sector Write Protection Disabled for User 1"]
pub type WPRODIS1_R = crate::BitReader<WPRODIS1_A>;
#[doc = "Sector Write Protection Disabled for User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPRODIS1_A {
    #[doc = "0: All protected sectors of user 1 are locked if write protection is installed"]
    VALUE1 = 0,
    #[doc = "1: All write-protected sectors of user 1 are temporarily unlocked, if not coincidently locked by user 0 or user 2 or via read protection."]
    VALUE2 = 1,
}
impl From<WPRODIS1_A> for bool {
    #[inline(always)]
    fn from(variant: WPRODIS1_A) -> Self {
        variant as u8 != 0
    }
}
impl WPRODIS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPRODIS1_A {
        match self.bits {
            false => WPRODIS1_A::VALUE1,
            true => WPRODIS1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WPRODIS1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WPRODIS1_A::VALUE2
    }
}
#[doc = "Field `SLM` reader - Flash Sleep Mode"]
pub type SLM_R = crate::BitReader<SLM_A>;
#[doc = "Flash Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLM_A {
    #[doc = "0: Flash not in sleep mode"]
    VALUE1 = 0,
    #[doc = "1: Flash is in sleep or shut down mode"]
    VALUE2 = 1,
}
impl From<SLM_A> for bool {
    #[inline(always)]
    fn from(variant: SLM_A) -> Self {
        variant as u8 != 0
    }
}
impl SLM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLM_A {
        match self.bits {
            false => SLM_A::VALUE1,
            true => SLM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SLM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SLM_A::VALUE2
    }
}
#[doc = "Field `VER` reader - Verify Error"]
pub type VER_R = crate::BitReader<VER_A>;
#[doc = "Verify Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VER_A {
    #[doc = "0: The page is correctly programmed or the sector correctly erased. All programmed or erased bits have full expected quality."]
    VALUE1 = 0,
    #[doc = "1: A program verify error or an erase verify error has been detected. Full quality (retention time) of all programmed (\"1\") or erased (\"0\") bits cannot be guaranteed."]
    VALUE2 = 1,
}
impl From<VER_A> for bool {
    #[inline(always)]
    fn from(variant: VER_A) -> Self {
        variant as u8 != 0
    }
}
impl VER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VER_A {
        match self.bits {
            false => VER_A::VALUE1,
            true => VER_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VER_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VER_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Program Flash Busy"]
    #[inline(always)]
    pub fn pbusy(&self) -> PBUSY_R {
        PBUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Flash Array Busy"]
    #[inline(always)]
    pub fn fabusy(&self) -> FABUSY_R {
        FABUSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Programming State"]
    #[inline(always)]
    pub fn prog(&self) -> PROG_R {
        PROG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Erase State"]
    #[inline(always)]
    pub fn erase(&self) -> ERASE_R {
        ERASE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Program Flash in Page Mode"]
    #[inline(always)]
    pub fn pfpage(&self) -> PFPAGE_R {
        PFPAGE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Program Flash Operation Error"]
    #[inline(always)]
    pub fn pfoper(&self) -> PFOPER_R {
        PFOPER_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Command Sequence Error"]
    #[inline(always)]
    pub fn sqer(&self) -> SQER_R {
        SQER_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Protection Error"]
    #[inline(always)]
    pub fn proer(&self) -> PROER_R {
        PROER_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PFLASH Single-Bit Error and Correction"]
    #[inline(always)]
    pub fn pfsber(&self) -> PFSBER_R {
        PFSBER_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - PFLASH Double-Bit Error"]
    #[inline(always)]
    pub fn pfdber(&self) -> PFDBER_R {
        PFDBER_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Protection Installed"]
    #[inline(always)]
    pub fn proin(&self) -> PROIN_R {
        PROIN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Read Protection Installed"]
    #[inline(always)]
    pub fn rproin(&self) -> RPROIN_R {
        RPROIN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Read Protection Disable State"]
    #[inline(always)]
    pub fn rprodis(&self) -> RPRODIS_R {
        RPRODIS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Sector Write Protection Installed for User 0"]
    #[inline(always)]
    pub fn wproin0(&self) -> WPROIN0_R {
        WPROIN0_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Sector Write Protection Installed for User 1"]
    #[inline(always)]
    pub fn wproin1(&self) -> WPROIN1_R {
        WPROIN1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Sector OTP Protection Installed for User 2"]
    #[inline(always)]
    pub fn wproin2(&self) -> WPROIN2_R {
        WPROIN2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Sector Write Protection Disabled for User 0"]
    #[inline(always)]
    pub fn wprodis0(&self) -> WPRODIS0_R {
        WPRODIS0_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Sector Write Protection Disabled for User 1"]
    #[inline(always)]
    pub fn wprodis1(&self) -> WPRODIS1_R {
        WPRODIS1_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Flash Sleep Mode"]
    #[inline(always)]
    pub fn slm(&self) -> SLM_R {
        SLM_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - Verify Error"]
    #[inline(always)]
    pub fn ver(&self) -> VER_R {
        VER_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Flash Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsr](index.html) module"]
pub struct FSR_SPEC;
impl crate::RegisterSpec for FSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsr::R](R) reader structure"]
impl crate::Readable for FSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FSR to value 0"]
impl crate::Resettable for FSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
