#[doc = "Register `FSR` reader"]
pub type R = crate::R<FsrSpec>;
#[doc = "Program Flash Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbusy {
    #[doc = "0: PFLASH ready, not busy; PFLASH in read mode."]
    Value1 = 0,
    #[doc = "1: PFLASH busy; PFLASH not in read mode."]
    Value2 = 1,
}
impl From<Pbusy> for bool {
    #[inline(always)]
    fn from(variant: Pbusy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBUSY` reader - Program Flash Busy"]
pub type PbusyR = crate::BitReader<Pbusy>;
impl PbusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pbusy {
        match self.bits {
            false => Pbusy::Value1,
            true => Pbusy::Value2,
        }
    }
    #[doc = "PFLASH ready, not busy; PFLASH in read mode."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pbusy::Value1
    }
    #[doc = "PFLASH busy; PFLASH not in read mode."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pbusy::Value2
    }
}
#[doc = "Field `FABUSY` reader - Flash Array Busy"]
pub type FabusyR = crate::BitReader;
#[doc = "Programming State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prog {
    #[doc = "0: There is no program operation requested or in progress or just finished."]
    Value1 = 0,
    #[doc = "1: Programming operation (write page) requested (from FIM) or in action or finished."]
    Value2 = 1,
}
impl From<Prog> for bool {
    #[inline(always)]
    fn from(variant: Prog) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROG` reader - Programming State"]
pub type ProgR = crate::BitReader<Prog>;
impl ProgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prog {
        match self.bits {
            false => Prog::Value1,
            true => Prog::Value2,
        }
    }
    #[doc = "There is no program operation requested or in progress or just finished."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Prog::Value1
    }
    #[doc = "Programming operation (write page) requested (from FIM) or in action or finished."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Prog::Value2
    }
}
#[doc = "Erase State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erase {
    #[doc = "0: There is no erase operation requested or in progress or just finished"]
    Value1 = 0,
    #[doc = "1: Erase operation requested (from FIM) or in action or finished."]
    Value2 = 1,
}
impl From<Erase> for bool {
    #[inline(always)]
    fn from(variant: Erase) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERASE` reader - Erase State"]
pub type EraseR = crate::BitReader<Erase>;
impl EraseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Erase {
        match self.bits {
            false => Erase::Value1,
            true => Erase::Value2,
        }
    }
    #[doc = "There is no erase operation requested or in progress or just finished"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Erase::Value1
    }
    #[doc = "Erase operation requested (from FIM) or in action or finished."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Erase::Value2
    }
}
#[doc = "Program Flash in Page Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pfpage {
    #[doc = "0: Program Flash not in page mode"]
    Value1 = 0,
    #[doc = "1: Program Flash in page mode; assembly buffer of PFLASH (256 byte) is in use (being filled up)"]
    Value2 = 1,
}
impl From<Pfpage> for bool {
    #[inline(always)]
    fn from(variant: Pfpage) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PFPAGE` reader - Program Flash in Page Mode"]
pub type PfpageR = crate::BitReader<Pfpage>;
impl PfpageR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pfpage {
        match self.bits {
            false => Pfpage::Value1,
            true => Pfpage::Value2,
        }
    }
    #[doc = "Program Flash not in page mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pfpage::Value1
    }
    #[doc = "Program Flash in page mode; assembly buffer of PFLASH (256 byte) is in use (being filled up)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pfpage::Value2
    }
}
#[doc = "Program Flash Operation Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pfoper {
    #[doc = "0: No operation error reported by Program Flash"]
    Value1 = 0,
    #[doc = "1: Flash array operation aborted, because of a Flash array failure, e.g. an ECC error in microcode."]
    Value2 = 1,
}
impl From<Pfoper> for bool {
    #[inline(always)]
    fn from(variant: Pfoper) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PFOPER` reader - Program Flash Operation Error"]
pub type PfoperR = crate::BitReader<Pfoper>;
impl PfoperR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pfoper {
        match self.bits {
            false => Pfoper::Value1,
            true => Pfoper::Value2,
        }
    }
    #[doc = "No operation error reported by Program Flash"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pfoper::Value1
    }
    #[doc = "Flash array operation aborted, because of a Flash array failure, e.g. an ECC error in microcode."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pfoper::Value2
    }
}
#[doc = "Command Sequence Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sqer {
    #[doc = "0: No sequence error"]
    Value1 = 0,
    #[doc = "1: Command state machine operation unsuccessful because of improper address or command sequence."]
    Value2 = 1,
}
impl From<Sqer> for bool {
    #[inline(always)]
    fn from(variant: Sqer) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SQER` reader - Command Sequence Error"]
pub type SqerR = crate::BitReader<Sqer>;
impl SqerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sqer {
        match self.bits {
            false => Sqer::Value1,
            true => Sqer::Value2,
        }
    }
    #[doc = "No sequence error"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sqer::Value1
    }
    #[doc = "Command state machine operation unsuccessful because of improper address or command sequence."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sqer::Value2
    }
}
#[doc = "Protection Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Proer {
    #[doc = "0: No protection error"]
    Value1 = 0,
    #[doc = "1: Protection error."]
    Value2 = 1,
}
impl From<Proer> for bool {
    #[inline(always)]
    fn from(variant: Proer) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROER` reader - Protection Error"]
pub type ProerR = crate::BitReader<Proer>;
impl ProerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Proer {
        match self.bits {
            false => Proer::Value1,
            true => Proer::Value2,
        }
    }
    #[doc = "No protection error"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Proer::Value1
    }
    #[doc = "Protection error."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Proer::Value2
    }
}
#[doc = "PFLASH Single-Bit Error and Correction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pfsber {
    #[doc = "0: No Single-Bit Error detected during read access to PFLASH"]
    Value1 = 0,
    #[doc = "1: Single-Bit Error detected and corrected"]
    Value2 = 1,
}
impl From<Pfsber> for bool {
    #[inline(always)]
    fn from(variant: Pfsber) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PFSBER` reader - PFLASH Single-Bit Error and Correction"]
pub type PfsberR = crate::BitReader<Pfsber>;
impl PfsberR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pfsber {
        match self.bits {
            false => Pfsber::Value1,
            true => Pfsber::Value2,
        }
    }
    #[doc = "No Single-Bit Error detected during read access to PFLASH"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pfsber::Value1
    }
    #[doc = "Single-Bit Error detected and corrected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pfsber::Value2
    }
}
#[doc = "PFLASH Double-Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pfdber {
    #[doc = "0: No Double-Bit Error detected during read access to PFLASH"]
    Value1 = 0,
    #[doc = "1: Double-Bit Error detected in PFLASH"]
    Value2 = 1,
}
impl From<Pfdber> for bool {
    #[inline(always)]
    fn from(variant: Pfdber) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PFDBER` reader - PFLASH Double-Bit Error"]
pub type PfdberR = crate::BitReader<Pfdber>;
impl PfdberR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pfdber {
        match self.bits {
            false => Pfdber::Value1,
            true => Pfdber::Value2,
        }
    }
    #[doc = "No Double-Bit Error detected during read access to PFLASH"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pfdber::Value1
    }
    #[doc = "Double-Bit Error detected in PFLASH"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pfdber::Value2
    }
}
#[doc = "Protection Installed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Proin {
    #[doc = "0: No protection is installed"]
    Value1 = 0,
    #[doc = "1: Read or/and write protection for one or more users is configured and correctly confirmed in the User Configuration Block(s)."]
    Value2 = 1,
}
impl From<Proin> for bool {
    #[inline(always)]
    fn from(variant: Proin) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROIN` reader - Protection Installed"]
pub type ProinR = crate::BitReader<Proin>;
impl ProinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Proin {
        match self.bits {
            false => Proin::Value1,
            true => Proin::Value2,
        }
    }
    #[doc = "No protection is installed"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Proin::Value1
    }
    #[doc = "Read or/and write protection for one or more users is configured and correctly confirmed in the User Configuration Block(s)."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Proin::Value2
    }
}
#[doc = "Read Protection Installed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rproin {
    #[doc = "0: No read protection installed"]
    Value1 = 0,
    #[doc = "1: Read protection and global write protection is configured and correctly confirmed in the User Configuration Block 0."]
    Value2 = 1,
}
impl From<Rproin> for bool {
    #[inline(always)]
    fn from(variant: Rproin) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPROIN` reader - Read Protection Installed"]
pub type RproinR = crate::BitReader<Rproin>;
impl RproinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rproin {
        match self.bits {
            false => Rproin::Value1,
            true => Rproin::Value2,
        }
    }
    #[doc = "No read protection installed"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rproin::Value1
    }
    #[doc = "Read protection and global write protection is configured and correctly confirmed in the User Configuration Block 0."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rproin::Value2
    }
}
#[doc = "Read Protection Disable State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rprodis {
    #[doc = "0: Read protection (if installed) is not disabled"]
    Value1 = 0,
    #[doc = "1: Read and global write protection is temporarily disabled."]
    Value2 = 1,
}
impl From<Rprodis> for bool {
    #[inline(always)]
    fn from(variant: Rprodis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPRODIS` reader - Read Protection Disable State"]
pub type RprodisR = crate::BitReader<Rprodis>;
impl RprodisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rprodis {
        match self.bits {
            false => Rprodis::Value1,
            true => Rprodis::Value2,
        }
    }
    #[doc = "Read protection (if installed) is not disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rprodis::Value1
    }
    #[doc = "Read and global write protection is temporarily disabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rprodis::Value2
    }
}
#[doc = "Sector Write Protection Installed for User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wproin0 {
    #[doc = "0: No write protection installed for user 0"]
    Value1 = 0,
    #[doc = "1: Sector write protection for user 0 is configured and correctly confirmed in the User Configuration Block 0."]
    Value2 = 1,
}
impl From<Wproin0> for bool {
    #[inline(always)]
    fn from(variant: Wproin0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPROIN0` reader - Sector Write Protection Installed for User 0"]
pub type Wproin0R = crate::BitReader<Wproin0>;
impl Wproin0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wproin0 {
        match self.bits {
            false => Wproin0::Value1,
            true => Wproin0::Value2,
        }
    }
    #[doc = "No write protection installed for user 0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Wproin0::Value1
    }
    #[doc = "Sector write protection for user 0 is configured and correctly confirmed in the User Configuration Block 0."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Wproin0::Value2
    }
}
#[doc = "Sector Write Protection Installed for User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wproin1 {
    #[doc = "0: No write protection installed for user 1"]
    Value1 = 0,
    #[doc = "1: Sector write protection for user 1 is configured and correctly confirmed in the User Configuration Block 1."]
    Value2 = 1,
}
impl From<Wproin1> for bool {
    #[inline(always)]
    fn from(variant: Wproin1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPROIN1` reader - Sector Write Protection Installed for User 1"]
pub type Wproin1R = crate::BitReader<Wproin1>;
impl Wproin1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wproin1 {
        match self.bits {
            false => Wproin1::Value1,
            true => Wproin1::Value2,
        }
    }
    #[doc = "No write protection installed for user 1"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Wproin1::Value1
    }
    #[doc = "Sector write protection for user 1 is configured and correctly confirmed in the User Configuration Block 1."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Wproin1::Value2
    }
}
#[doc = "Sector OTP Protection Installed for User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wproin2 {
    #[doc = "0: No OTP write protection installed for user 2"]
    Value1 = 0,
    #[doc = "1: Sector OTP write protection with ROM functionality is configured and correctly confirmed in the UCB2. The protection is locked for ever."]
    Value2 = 1,
}
impl From<Wproin2> for bool {
    #[inline(always)]
    fn from(variant: Wproin2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPROIN2` reader - Sector OTP Protection Installed for User 2"]
pub type Wproin2R = crate::BitReader<Wproin2>;
impl Wproin2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wproin2 {
        match self.bits {
            false => Wproin2::Value1,
            true => Wproin2::Value2,
        }
    }
    #[doc = "No OTP write protection installed for user 2"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Wproin2::Value1
    }
    #[doc = "Sector OTP write protection with ROM functionality is configured and correctly confirmed in the UCB2. The protection is locked for ever."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Wproin2::Value2
    }
}
#[doc = "Sector Write Protection Disabled for User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wprodis0 {
    #[doc = "0: All protected sectors of user 0 are locked if write protection is installed"]
    Value1 = 0,
    #[doc = "1: All write-protected sectors of user 0 are temporarily unlocked, if not coincidently locked by user 2 or via read protection."]
    Value2 = 1,
}
impl From<Wprodis0> for bool {
    #[inline(always)]
    fn from(variant: Wprodis0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPRODIS0` reader - Sector Write Protection Disabled for User 0"]
pub type Wprodis0R = crate::BitReader<Wprodis0>;
impl Wprodis0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wprodis0 {
        match self.bits {
            false => Wprodis0::Value1,
            true => Wprodis0::Value2,
        }
    }
    #[doc = "All protected sectors of user 0 are locked if write protection is installed"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Wprodis0::Value1
    }
    #[doc = "All write-protected sectors of user 0 are temporarily unlocked, if not coincidently locked by user 2 or via read protection."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Wprodis0::Value2
    }
}
#[doc = "Sector Write Protection Disabled for User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wprodis1 {
    #[doc = "0: All protected sectors of user 1 are locked if write protection is installed"]
    Value1 = 0,
    #[doc = "1: All write-protected sectors of user 1 are temporarily unlocked, if not coincidently locked by user 0 or user 2 or via read protection."]
    Value2 = 1,
}
impl From<Wprodis1> for bool {
    #[inline(always)]
    fn from(variant: Wprodis1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPRODIS1` reader - Sector Write Protection Disabled for User 1"]
pub type Wprodis1R = crate::BitReader<Wprodis1>;
impl Wprodis1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wprodis1 {
        match self.bits {
            false => Wprodis1::Value1,
            true => Wprodis1::Value2,
        }
    }
    #[doc = "All protected sectors of user 1 are locked if write protection is installed"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Wprodis1::Value1
    }
    #[doc = "All write-protected sectors of user 1 are temporarily unlocked, if not coincidently locked by user 0 or user 2 or via read protection."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Wprodis1::Value2
    }
}
#[doc = "Flash Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slm {
    #[doc = "0: Flash not in sleep mode"]
    Value1 = 0,
    #[doc = "1: Flash is in sleep or shut down mode"]
    Value2 = 1,
}
impl From<Slm> for bool {
    #[inline(always)]
    fn from(variant: Slm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLM` reader - Flash Sleep Mode"]
pub type SlmR = crate::BitReader<Slm>;
impl SlmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slm {
        match self.bits {
            false => Slm::Value1,
            true => Slm::Value2,
        }
    }
    #[doc = "Flash not in sleep mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Slm::Value1
    }
    #[doc = "Flash is in sleep or shut down mode"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Slm::Value2
    }
}
#[doc = "Verify Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ver {
    #[doc = "0: The page is correctly programmed or the sector correctly erased. All programmed or erased bits have full expected quality."]
    Value1 = 0,
    #[doc = "1: A program verify error or an erase verify error has been detected. Full quality (retention time) of all programmed (\"1\") or erased (\"0\") bits cannot be guaranteed."]
    Value2 = 1,
}
impl From<Ver> for bool {
    #[inline(always)]
    fn from(variant: Ver) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VER` reader - Verify Error"]
pub type VerR = crate::BitReader<Ver>;
impl VerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ver {
        match self.bits {
            false => Ver::Value1,
            true => Ver::Value2,
        }
    }
    #[doc = "The page is correctly programmed or the sector correctly erased. All programmed or erased bits have full expected quality."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ver::Value1
    }
    #[doc = "A program verify error or an erase verify error has been detected. Full quality (retention time) of all programmed (\"1\") or erased (\"0\") bits cannot be guaranteed."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ver::Value2
    }
}
impl R {
    #[doc = "Bit 0 - Program Flash Busy"]
    #[inline(always)]
    pub fn pbusy(&self) -> PbusyR {
        PbusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Flash Array Busy"]
    #[inline(always)]
    pub fn fabusy(&self) -> FabusyR {
        FabusyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Programming State"]
    #[inline(always)]
    pub fn prog(&self) -> ProgR {
        ProgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Erase State"]
    #[inline(always)]
    pub fn erase(&self) -> EraseR {
        EraseR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Program Flash in Page Mode"]
    #[inline(always)]
    pub fn pfpage(&self) -> PfpageR {
        PfpageR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Program Flash Operation Error"]
    #[inline(always)]
    pub fn pfoper(&self) -> PfoperR {
        PfoperR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Command Sequence Error"]
    #[inline(always)]
    pub fn sqer(&self) -> SqerR {
        SqerR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Protection Error"]
    #[inline(always)]
    pub fn proer(&self) -> ProerR {
        ProerR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PFLASH Single-Bit Error and Correction"]
    #[inline(always)]
    pub fn pfsber(&self) -> PfsberR {
        PfsberR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - PFLASH Double-Bit Error"]
    #[inline(always)]
    pub fn pfdber(&self) -> PfdberR {
        PfdberR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Protection Installed"]
    #[inline(always)]
    pub fn proin(&self) -> ProinR {
        ProinR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Read Protection Installed"]
    #[inline(always)]
    pub fn rproin(&self) -> RproinR {
        RproinR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Read Protection Disable State"]
    #[inline(always)]
    pub fn rprodis(&self) -> RprodisR {
        RprodisR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Sector Write Protection Installed for User 0"]
    #[inline(always)]
    pub fn wproin0(&self) -> Wproin0R {
        Wproin0R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Sector Write Protection Installed for User 1"]
    #[inline(always)]
    pub fn wproin1(&self) -> Wproin1R {
        Wproin1R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Sector OTP Protection Installed for User 2"]
    #[inline(always)]
    pub fn wproin2(&self) -> Wproin2R {
        Wproin2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Sector Write Protection Disabled for User 0"]
    #[inline(always)]
    pub fn wprodis0(&self) -> Wprodis0R {
        Wprodis0R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Sector Write Protection Disabled for User 1"]
    #[inline(always)]
    pub fn wprodis1(&self) -> Wprodis1R {
        Wprodis1R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Flash Sleep Mode"]
    #[inline(always)]
    pub fn slm(&self) -> SlmR {
        SlmR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - Verify Error"]
    #[inline(always)]
    pub fn ver(&self) -> VerR {
        VerR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Flash Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsrSpec;
impl crate::RegisterSpec for FsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsr::R`](R) reader structure"]
impl crate::Readable for FsrSpec {}
#[doc = "`reset()` method sets FSR to value 0"]
impl crate::Resettable for FsrSpec {
    const RESET_VALUE: u32 = 0;
}
