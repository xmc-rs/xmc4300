#[doc = "Register `TRAPRAW` reader"]
pub type R = crate::R<TraprawSpec>;
#[doc = "OSC_HP Oscillator Watchdog Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Soscwdgt {
    #[doc = "0: No pending trap request"]
    Const0 = 0,
    #[doc = "1: Pending trap request"]
    Const1 = 1,
}
impl From<Soscwdgt> for bool {
    #[inline(always)]
    fn from(variant: Soscwdgt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOSCWDGT` reader - OSC_HP Oscillator Watchdog Trap Raw Status"]
pub type SoscwdgtR = crate::BitReader<Soscwdgt>;
impl SoscwdgtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Soscwdgt {
        match self.bits {
            false => Soscwdgt::Const0,
            true => Soscwdgt::Const1,
        }
    }
    #[doc = "No pending trap request"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Soscwdgt::Const0
    }
    #[doc = "Pending trap request"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Soscwdgt::Const1
    }
}
#[doc = "System VCO Lock Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Svcolckt {
    #[doc = "0: No pending trap request"]
    Const0 = 0,
    #[doc = "1: Pending trap request"]
    Const1 = 1,
}
impl From<Svcolckt> for bool {
    #[inline(always)]
    fn from(variant: Svcolckt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SVCOLCKT` reader - System VCO Lock Trap Raw Status"]
pub type SvcolcktR = crate::BitReader<Svcolckt>;
impl SvcolcktR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Svcolckt {
        match self.bits {
            false => Svcolckt::Const0,
            true => Svcolckt::Const1,
        }
    }
    #[doc = "No pending trap request"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Svcolckt::Const0
    }
    #[doc = "Pending trap request"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Svcolckt::Const1
    }
}
#[doc = "USB VCO Lock Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uvcolckt {
    #[doc = "0: No pending trap request"]
    Const0 = 0,
    #[doc = "1: Pending trap request"]
    Const1 = 1,
}
impl From<Uvcolckt> for bool {
    #[inline(always)]
    fn from(variant: Uvcolckt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UVCOLCKT` reader - USB VCO Lock Trap Raw Status"]
pub type UvcolcktR = crate::BitReader<Uvcolckt>;
impl UvcolcktR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uvcolckt {
        match self.bits {
            false => Uvcolckt::Const0,
            true => Uvcolckt::Const1,
        }
    }
    #[doc = "No pending trap request"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Uvcolckt::Const0
    }
    #[doc = "Pending trap request"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Uvcolckt::Const1
    }
}
#[doc = "Parity Error Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pet {
    #[doc = "0: No pending trap request"]
    Const0 = 0,
    #[doc = "1: Pending trap request"]
    Const1 = 1,
}
impl From<Pet> for bool {
    #[inline(always)]
    fn from(variant: Pet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PET` reader - Parity Error Trap Raw Status"]
pub type PetR = crate::BitReader<Pet>;
impl PetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pet {
        match self.bits {
            false => Pet::Const0,
            true => Pet::Const1,
        }
    }
    #[doc = "No pending trap request"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pet::Const0
    }
    #[doc = "Pending trap request"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pet::Const1
    }
}
#[doc = "Brown Out Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brwnt {
    #[doc = "0: No pending trap request"]
    Const0 = 0,
    #[doc = "1: Pending trap request"]
    Const1 = 1,
}
impl From<Brwnt> for bool {
    #[inline(always)]
    fn from(variant: Brwnt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRWNT` reader - Brown Out Trap Raw Status"]
pub type BrwntR = crate::BitReader<Brwnt>;
impl BrwntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Brwnt {
        match self.bits {
            false => Brwnt::Const0,
            true => Brwnt::Const1,
        }
    }
    #[doc = "No pending trap request"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Brwnt::Const0
    }
    #[doc = "Pending trap request"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Brwnt::Const1
    }
}
#[doc = "OSC_ULP Oscillator Watchdog Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ulpwdgt {
    #[doc = "0: No pending trap request"]
    Const0 = 0,
    #[doc = "1: Pending trap request"]
    Const1 = 1,
}
impl From<Ulpwdgt> for bool {
    #[inline(always)]
    fn from(variant: Ulpwdgt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULPWDGT` reader - OSC_ULP Oscillator Watchdog Trap Raw Status"]
pub type UlpwdgtR = crate::BitReader<Ulpwdgt>;
impl UlpwdgtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ulpwdgt {
        match self.bits {
            false => Ulpwdgt::Const0,
            true => Ulpwdgt::Const1,
        }
    }
    #[doc = "No pending trap request"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Ulpwdgt::Const0
    }
    #[doc = "Pending trap request"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Ulpwdgt::Const1
    }
}
#[doc = "Peripheral Bridge 0 Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bwerr0t {
    #[doc = "0: No pending trap request"]
    Const0 = 0,
    #[doc = "1: Pending trap request"]
    Const1 = 1,
}
impl From<Bwerr0t> for bool {
    #[inline(always)]
    fn from(variant: Bwerr0t) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWERR0T` reader - Peripheral Bridge 0 Trap Raw Status"]
pub type Bwerr0tR = crate::BitReader<Bwerr0t>;
impl Bwerr0tR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bwerr0t {
        match self.bits {
            false => Bwerr0t::Const0,
            true => Bwerr0t::Const1,
        }
    }
    #[doc = "No pending trap request"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Bwerr0t::Const0
    }
    #[doc = "Pending trap request"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Bwerr0t::Const1
    }
}
#[doc = "Peripheral Bridge 1 Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bwerr1t {
    #[doc = "0: No pending trap request"]
    Const0 = 0,
    #[doc = "1: Pending trap request"]
    Const1 = 1,
}
impl From<Bwerr1t> for bool {
    #[inline(always)]
    fn from(variant: Bwerr1t) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWERR1T` reader - Peripheral Bridge 1 Trap Raw Status"]
pub type Bwerr1tR = crate::BitReader<Bwerr1t>;
impl Bwerr1tR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bwerr1t {
        match self.bits {
            false => Bwerr1t::Const0,
            true => Bwerr1t::Const1,
        }
    }
    #[doc = "No pending trap request"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Bwerr1t::Const0
    }
    #[doc = "Pending trap request"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Bwerr1t::Const1
    }
}
#[doc = "EtherCat Reset 0 Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecat0rst {
    #[doc = "0: No pending trap request"]
    Const0 = 0,
    #[doc = "1: Pending trap request"]
    Const1 = 1,
}
impl From<Ecat0rst> for bool {
    #[inline(always)]
    fn from(variant: Ecat0rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECAT0RST` reader - EtherCat Reset 0 Trap Raw Status"]
pub type Ecat0rstR = crate::BitReader<Ecat0rst>;
impl Ecat0rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ecat0rst {
        match self.bits {
            false => Ecat0rst::Const0,
            true => Ecat0rst::Const1,
        }
    }
    #[doc = "No pending trap request"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Ecat0rst::Const0
    }
    #[doc = "Pending trap request"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Ecat0rst::Const1
    }
}
impl R {
    #[doc = "Bit 0 - OSC_HP Oscillator Watchdog Trap Raw Status"]
    #[inline(always)]
    pub fn soscwdgt(&self) -> SoscwdgtR {
        SoscwdgtR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - System VCO Lock Trap Raw Status"]
    #[inline(always)]
    pub fn svcolckt(&self) -> SvcolcktR {
        SvcolcktR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB VCO Lock Trap Raw Status"]
    #[inline(always)]
    pub fn uvcolckt(&self) -> UvcolcktR {
        UvcolcktR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Parity Error Trap Raw Status"]
    #[inline(always)]
    pub fn pet(&self) -> PetR {
        PetR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Brown Out Trap Raw Status"]
    #[inline(always)]
    pub fn brwnt(&self) -> BrwntR {
        BrwntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OSC_ULP Oscillator Watchdog Trap Raw Status"]
    #[inline(always)]
    pub fn ulpwdgt(&self) -> UlpwdgtR {
        UlpwdgtR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Peripheral Bridge 0 Trap Raw Status"]
    #[inline(always)]
    pub fn bwerr0t(&self) -> Bwerr0tR {
        Bwerr0tR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Peripheral Bridge 1 Trap Raw Status"]
    #[inline(always)]
    pub fn bwerr1t(&self) -> Bwerr1tR {
        Bwerr1tR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - EtherCat Reset 0 Trap Raw Status"]
    #[inline(always)]
    pub fn ecat0rst(&self) -> Ecat0rstR {
        Ecat0rstR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Trap Raw Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trapraw::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TraprawSpec;
impl crate::RegisterSpec for TraprawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trapraw::R`](R) reader structure"]
impl crate::Readable for TraprawSpec {}
#[doc = "`reset()` method sets TRAPRAW to value 0"]
impl crate::Resettable for TraprawSpec {
    const RESET_VALUE: u32 = 0;
}
