#[doc = "Register `PWRSTAT` reader"]
pub type R = crate::R<PwrstatSpec>;
#[doc = "Hibernate Domain Enable Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hiben {
    #[doc = "0: Inactive"]
    Const0 = 0,
    #[doc = "1: Active"]
    Const1 = 1,
}
impl From<Hiben> for bool {
    #[inline(always)]
    fn from(variant: Hiben) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIBEN` reader - Hibernate Domain Enable Status"]
pub type HibenR = crate::BitReader<Hiben>;
impl HibenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hiben {
        match self.bits {
            false => Hiben::Const0,
            true => Hiben::Const1,
        }
    }
    #[doc = "Inactive"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Hiben::Const0
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Hiben::Const1
    }
}
#[doc = "USB PHY Transceiver State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbphypdq {
    #[doc = "0: Power-down"]
    Const0 = 0,
    #[doc = "1: Active"]
    Const1 = 1,
}
impl From<Usbphypdq> for bool {
    #[inline(always)]
    fn from(variant: Usbphypdq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBPHYPDQ` reader - USB PHY Transceiver State"]
pub type UsbphypdqR = crate::BitReader<Usbphypdq>;
impl UsbphypdqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbphypdq {
        match self.bits {
            false => Usbphypdq::Const0,
            true => Usbphypdq::Const1,
        }
    }
    #[doc = "Power-down"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Usbphypdq::Const0
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Usbphypdq::Const1
    }
}
#[doc = "USB On-The-Go Comparators State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbotgen {
    #[doc = "0: Power-down"]
    Const0 = 0,
    #[doc = "1: Active"]
    Const1 = 1,
}
impl From<Usbotgen> for bool {
    #[inline(always)]
    fn from(variant: Usbotgen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBOTGEN` reader - USB On-The-Go Comparators State"]
pub type UsbotgenR = crate::BitReader<Usbotgen>;
impl UsbotgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbotgen {
        match self.bits {
            false => Usbotgen::Const0,
            true => Usbotgen::Const1,
        }
    }
    #[doc = "Power-down"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Usbotgen::Const0
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Usbotgen::Const1
    }
}
#[doc = "USB Weak Pull-Up at PADN State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbpuwq {
    #[doc = "0: Pull-up active"]
    Const0 = 0,
    #[doc = "1: Pull-up not active"]
    Const1 = 1,
}
impl From<Usbpuwq> for bool {
    #[inline(always)]
    fn from(variant: Usbpuwq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBPUWQ` reader - USB Weak Pull-Up at PADN State"]
pub type UsbpuwqR = crate::BitReader<Usbpuwq>;
impl UsbpuwqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbpuwq {
        match self.bits {
            false => Usbpuwq::Const0,
            true => Usbpuwq::Const1,
        }
    }
    #[doc = "Pull-up active"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Usbpuwq::Const0
    }
    #[doc = "Pull-up not active"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Usbpuwq::Const1
    }
}
impl R {
    #[doc = "Bit 0 - Hibernate Domain Enable Status"]
    #[inline(always)]
    pub fn hiben(&self) -> HibenR {
        HibenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - USB PHY Transceiver State"]
    #[inline(always)]
    pub fn usbphypdq(&self) -> UsbphypdqR {
        UsbphypdqR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - USB On-The-Go Comparators State"]
    #[inline(always)]
    pub fn usbotgen(&self) -> UsbotgenR {
        UsbotgenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USB Weak Pull-Up at PADN State"]
    #[inline(always)]
    pub fn usbpuwq(&self) -> UsbpuwqR {
        UsbpuwqR::new(((self.bits >> 18) & 1) != 0)
    }
}
#[doc = "PCU Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrstatSpec;
impl crate::RegisterSpec for PwrstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrstat::R`](R) reader structure"]
impl crate::Readable for PwrstatSpec {}
#[doc = "`reset()` method sets PWRSTAT to value 0"]
impl crate::Resettable for PwrstatSpec {
    const RESET_VALUE: u32 = 0;
}
