#[doc = "Register `SRRAW` reader"]
pub type R = crate::R<SrrawSpec>;
#[doc = "WDT pre-warning Interrupt Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prwarn {
    #[doc = "0: Inactive"]
    Const0 = 0,
    #[doc = "1: Active"]
    Const1 = 1,
}
impl From<Prwarn> for bool {
    #[inline(always)]
    fn from(variant: Prwarn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRWARN` reader - WDT pre-warning Interrupt Status Before Masking"]
pub type PrwarnR = crate::BitReader<Prwarn>;
impl PrwarnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prwarn {
        match self.bits {
            false => Prwarn::Const0,
            true => Prwarn::Const1,
        }
    }
    #[doc = "Inactive"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Prwarn::Const0
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Prwarn::Const1
    }
}
#[doc = "Field `PI` reader - RTC Raw Periodic Interrupt Status Before Masking"]
pub type PiR = crate::BitReader;
#[doc = "Field `AI` reader - RTC Raw Alarm Interrupt Status Before Masking"]
pub type AiR = crate::BitReader;
#[doc = "Field `DLROVR` reader - DLR Request Overrun Interrupt Status Before Masking"]
pub type DlrovrR = crate::BitReader;
#[doc = "HDCLR Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hdclr {
    #[doc = "0: Not updated"]
    Const0 = 0,
    #[doc = "1: Update completed"]
    Const1 = 1,
}
impl From<Hdclr> for bool {
    #[inline(always)]
    fn from(variant: Hdclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCLR` reader - HDCLR Mirror Register Update Status Before Masking"]
pub type HdclrR = crate::BitReader<Hdclr>;
impl HdclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hdclr {
        match self.bits {
            false => Hdclr::Const0,
            true => Hdclr::Const1,
        }
    }
    #[doc = "Not updated"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Hdclr::Const0
    }
    #[doc = "Update completed"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Hdclr::Const1
    }
}
#[doc = "HDSET Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hdset {
    #[doc = "0: Not updated"]
    Const0 = 0,
    #[doc = "1: Update completed"]
    Const1 = 1,
}
impl From<Hdset> for bool {
    #[inline(always)]
    fn from(variant: Hdset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDSET` reader - HDSET Mirror Register Update Status Before Masking"]
pub type HdsetR = crate::BitReader<Hdset>;
impl HdsetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hdset {
        match self.bits {
            false => Hdset::Const0,
            true => Hdset::Const1,
        }
    }
    #[doc = "Not updated"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Hdset::Const0
    }
    #[doc = "Update completed"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Hdset::Const1
    }
}
#[doc = "HDCR Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hdcr {
    #[doc = "0: Not updated"]
    Const0 = 0,
    #[doc = "1: Update completed"]
    Const1 = 1,
}
impl From<Hdcr> for bool {
    #[inline(always)]
    fn from(variant: Hdcr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCR` reader - HDCR Mirror Register Update Status Before Masking"]
pub type HdcrR = crate::BitReader<Hdcr>;
impl HdcrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hdcr {
        match self.bits {
            false => Hdcr::Const0,
            true => Hdcr::Const1,
        }
    }
    #[doc = "Not updated"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Hdcr::Const0
    }
    #[doc = "Update completed"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Hdcr::Const1
    }
}
#[doc = "OSCSICTRL Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oscsictrl {
    #[doc = "0: Not updated"]
    Const0 = 0,
    #[doc = "1: Update completed"]
    Const1 = 1,
}
impl From<Oscsictrl> for bool {
    #[inline(always)]
    fn from(variant: Oscsictrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCSICTRL` reader - OSCSICTRL Mirror Register Update Status Before Masking"]
pub type OscsictrlR = crate::BitReader<Oscsictrl>;
impl OscsictrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oscsictrl {
        match self.bits {
            false => Oscsictrl::Const0,
            true => Oscsictrl::Const1,
        }
    }
    #[doc = "Not updated"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Oscsictrl::Const0
    }
    #[doc = "Update completed"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Oscsictrl::Const1
    }
}
#[doc = "OSCULCTRL Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Osculctrl {
    #[doc = "0: Not updated"]
    Const0 = 0,
    #[doc = "1: Update completed"]
    Const1 = 1,
}
impl From<Osculctrl> for bool {
    #[inline(always)]
    fn from(variant: Osculctrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCULCTRL` reader - OSCULCTRL Mirror Register Update Status Before Masking"]
pub type OsculctrlR = crate::BitReader<Osculctrl>;
impl OsculctrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Osculctrl {
        match self.bits {
            false => Osculctrl::Const0,
            true => Osculctrl::Const1,
        }
    }
    #[doc = "Not updated"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Osculctrl::Const0
    }
    #[doc = "Update completed"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Osculctrl::Const1
    }
}
#[doc = "RTC CTR Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcCtr {
    #[doc = "0: Not updated"]
    Const0 = 0,
    #[doc = "1: Update completed"]
    Const1 = 1,
}
impl From<RtcCtr> for bool {
    #[inline(always)]
    fn from(variant: RtcCtr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_CTR` reader - RTC CTR Mirror Register Update Status Before Masking"]
pub type RtcCtrR = crate::BitReader<RtcCtr>;
impl RtcCtrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RtcCtr {
        match self.bits {
            false => RtcCtr::Const0,
            true => RtcCtr::Const1,
        }
    }
    #[doc = "Not updated"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == RtcCtr::Const0
    }
    #[doc = "Update completed"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == RtcCtr::Const1
    }
}
#[doc = "RTC ATIM0 Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcAtim0 {
    #[doc = "0: Not updated"]
    Const0 = 0,
    #[doc = "1: Update completed"]
    Const1 = 1,
}
impl From<RtcAtim0> for bool {
    #[inline(always)]
    fn from(variant: RtcAtim0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_ATIM0` reader - RTC ATIM0 Mirror Register Update Status Before Masking"]
pub type RtcAtim0R = crate::BitReader<RtcAtim0>;
impl RtcAtim0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RtcAtim0 {
        match self.bits {
            false => RtcAtim0::Const0,
            true => RtcAtim0::Const1,
        }
    }
    #[doc = "Not updated"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == RtcAtim0::Const0
    }
    #[doc = "Update completed"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == RtcAtim0::Const1
    }
}
#[doc = "RTC ATIM1 Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcAtim1 {
    #[doc = "0: Not updated"]
    Const0 = 0,
    #[doc = "1: Update completed"]
    Const1 = 1,
}
impl From<RtcAtim1> for bool {
    #[inline(always)]
    fn from(variant: RtcAtim1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_ATIM1` reader - RTC ATIM1 Mirror Register Update Status Before Masking"]
pub type RtcAtim1R = crate::BitReader<RtcAtim1>;
impl RtcAtim1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RtcAtim1 {
        match self.bits {
            false => RtcAtim1::Const0,
            true => RtcAtim1::Const1,
        }
    }
    #[doc = "Not updated"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == RtcAtim1::Const0
    }
    #[doc = "Update completed"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == RtcAtim1::Const1
    }
}
#[doc = "RTC TIM0 Mirror Register Update Before Masking Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcTim0 {
    #[doc = "0: Not updated"]
    Const0 = 0,
    #[doc = "1: Update completed"]
    Const1 = 1,
}
impl From<RtcTim0> for bool {
    #[inline(always)]
    fn from(variant: RtcTim0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_TIM0` reader - RTC TIM0 Mirror Register Update Before Masking Status"]
pub type RtcTim0R = crate::BitReader<RtcTim0>;
impl RtcTim0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RtcTim0 {
        match self.bits {
            false => RtcTim0::Const0,
            true => RtcTim0::Const1,
        }
    }
    #[doc = "Not updated"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == RtcTim0::Const0
    }
    #[doc = "Update completed"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == RtcTim0::Const1
    }
}
#[doc = "RTC TIM1 Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcTim1 {
    #[doc = "0: Not updated"]
    Const0 = 0,
    #[doc = "1: Update completed"]
    Const1 = 1,
}
impl From<RtcTim1> for bool {
    #[inline(always)]
    fn from(variant: RtcTim1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_TIM1` reader - RTC TIM1 Mirror Register Update Status Before Masking"]
pub type RtcTim1R = crate::BitReader<RtcTim1>;
impl RtcTim1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RtcTim1 {
        match self.bits {
            false => RtcTim1::Const0,
            true => RtcTim1::Const1,
        }
    }
    #[doc = "Not updated"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == RtcTim1::Const0
    }
    #[doc = "Update completed"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == RtcTim1::Const1
    }
}
#[doc = "Retention Memory Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rmx {
    #[doc = "0: Not updated"]
    Const0 = 0,
    #[doc = "1: Update completed"]
    Const1 = 1,
}
impl From<Rmx> for bool {
    #[inline(always)]
    fn from(variant: Rmx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RMX` reader - Retention Memory Mirror Register Update Status Before Masking"]
pub type RmxR = crate::BitReader<Rmx>;
impl RmxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rmx {
        match self.bits {
            false => Rmx::Const0,
            true => Rmx::Const1,
        }
    }
    #[doc = "Not updated"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Rmx::Const0
    }
    #[doc = "Update completed"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Rmx::Const1
    }
}
impl R {
    #[doc = "Bit 0 - WDT pre-warning Interrupt Status Before Masking"]
    #[inline(always)]
    pub fn prwarn(&self) -> PrwarnR {
        PrwarnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTC Raw Periodic Interrupt Status Before Masking"]
    #[inline(always)]
    pub fn pi(&self) -> PiR {
        PiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RTC Raw Alarm Interrupt Status Before Masking"]
    #[inline(always)]
    pub fn ai(&self) -> AiR {
        AiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DLR Request Overrun Interrupt Status Before Masking"]
    #[inline(always)]
    pub fn dlrovr(&self) -> DlrovrR {
        DlrovrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 17 - HDCLR Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn hdclr(&self) -> HdclrR {
        HdclrR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HDSET Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn hdset(&self) -> HdsetR {
        HdsetR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HDCR Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn hdcr(&self) -> HdcrR {
        HdcrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - OSCSICTRL Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn oscsictrl(&self) -> OscsictrlR {
        OscsictrlR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - OSCULCTRL Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn osculctrl(&self) -> OsculctrlR {
        OsculctrlR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - RTC CTR Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn rtc_ctr(&self) -> RtcCtrR {
        RtcCtrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - RTC ATIM0 Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn rtc_atim0(&self) -> RtcAtim0R {
        RtcAtim0R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - RTC ATIM1 Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn rtc_atim1(&self) -> RtcAtim1R {
        RtcAtim1R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - RTC TIM0 Mirror Register Update Before Masking Status"]
    #[inline(always)]
    pub fn rtc_tim0(&self) -> RtcTim0R {
        RtcTim0R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - RTC TIM1 Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn rtc_tim1(&self) -> RtcTim1R {
        RtcTim1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Retention Memory Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn rmx(&self) -> RmxR {
        RmxR::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "SCU Raw Service Request Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srraw::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrrawSpec;
impl crate::RegisterSpec for SrrawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srraw::R`](R) reader structure"]
impl crate::Readable for SrrawSpec {}
#[doc = "`reset()` method sets SRRAW to value 0"]
impl crate::Resettable for SrrawSpec {
    const RESET_VALUE: u32 = 0;
}
