#[doc = "Register `STCON` reader"]
pub type R = crate::R<StconSpec>;
#[doc = "Register `STCON` writer"]
pub type W = crate::W<StconSpec>;
#[doc = "HW Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hwcon {
    #[doc = "0: Normal mode, JTAG"]
    Const00 = 0,
    #[doc = "1: ASC BSL enabled"]
    Const01 = 1,
    #[doc = "2: BMI customized boot enabled"]
    Const10 = 2,
    #[doc = "3: CAN BSL enabled"]
    Const11 = 3,
}
impl From<Hwcon> for u8 {
    #[inline(always)]
    fn from(variant: Hwcon) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hwcon {
    type Ux = u8;
}
#[doc = "Field `HWCON` reader - HW Configuration"]
pub type HwconR = crate::FieldReader<Hwcon>;
impl HwconR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hwcon {
        match self.bits {
            0 => Hwcon::Const00,
            1 => Hwcon::Const01,
            2 => Hwcon::Const10,
            3 => Hwcon::Const11,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal mode, JTAG"]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == Hwcon::Const00
    }
    #[doc = "ASC BSL enabled"]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == Hwcon::Const01
    }
    #[doc = "BMI customized boot enabled"]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == Hwcon::Const10
    }
    #[doc = "CAN BSL enabled"]
    #[inline(always)]
    pub fn is_const_11(&self) -> bool {
        *self == Hwcon::Const11
    }
}
#[doc = "SW Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Swcon {
    #[doc = "0: Normal mode, boot from Boot ROM"]
    Const0000 = 0,
    #[doc = "1: ASC BSL enabled"]
    Const0001 = 1,
    #[doc = "2: BMI customized boot enabled"]
    Const0010 = 2,
    #[doc = "3: CAN BSL enabled"]
    Const0011 = 3,
    #[doc = "4: Boot from Code SRAM"]
    Const0100 = 4,
    #[doc = "8: Boot from alternate Flash Address 0"]
    Const1000 = 8,
    #[doc = "12: Boot from alternate Flash Address 1"]
    Const1100 = 12,
    #[doc = "14: Enable fallback Alternate Boot Mode (ABM)"]
    Const1110 = 14,
}
impl From<Swcon> for u8 {
    #[inline(always)]
    fn from(variant: Swcon) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Swcon {
    type Ux = u8;
}
#[doc = "Field `SWCON` reader - SW Configuration"]
pub type SwconR = crate::FieldReader<Swcon>;
impl SwconR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Swcon> {
        match self.bits {
            0 => Some(Swcon::Const0000),
            1 => Some(Swcon::Const0001),
            2 => Some(Swcon::Const0010),
            3 => Some(Swcon::Const0011),
            4 => Some(Swcon::Const0100),
            8 => Some(Swcon::Const1000),
            12 => Some(Swcon::Const1100),
            14 => Some(Swcon::Const1110),
            _ => None,
        }
    }
    #[doc = "Normal mode, boot from Boot ROM"]
    #[inline(always)]
    pub fn is_const_0000(&self) -> bool {
        *self == Swcon::Const0000
    }
    #[doc = "ASC BSL enabled"]
    #[inline(always)]
    pub fn is_const_0001(&self) -> bool {
        *self == Swcon::Const0001
    }
    #[doc = "BMI customized boot enabled"]
    #[inline(always)]
    pub fn is_const_0010(&self) -> bool {
        *self == Swcon::Const0010
    }
    #[doc = "CAN BSL enabled"]
    #[inline(always)]
    pub fn is_const_0011(&self) -> bool {
        *self == Swcon::Const0011
    }
    #[doc = "Boot from Code SRAM"]
    #[inline(always)]
    pub fn is_const_0100(&self) -> bool {
        *self == Swcon::Const0100
    }
    #[doc = "Boot from alternate Flash Address 0"]
    #[inline(always)]
    pub fn is_const_1000(&self) -> bool {
        *self == Swcon::Const1000
    }
    #[doc = "Boot from alternate Flash Address 1"]
    #[inline(always)]
    pub fn is_const_1100(&self) -> bool {
        *self == Swcon::Const1100
    }
    #[doc = "Enable fallback Alternate Boot Mode (ABM)"]
    #[inline(always)]
    pub fn is_const_1110(&self) -> bool {
        *self == Swcon::Const1110
    }
}
#[doc = "Field `SWCON` writer - SW Configuration"]
pub type SwconW<'a, REG> = crate::FieldWriter<'a, REG, 4, Swcon>;
impl<'a, REG> SwconW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal mode, boot from Boot ROM"]
    #[inline(always)]
    pub fn const_0000(self) -> &'a mut crate::W<REG> {
        self.variant(Swcon::Const0000)
    }
    #[doc = "ASC BSL enabled"]
    #[inline(always)]
    pub fn const_0001(self) -> &'a mut crate::W<REG> {
        self.variant(Swcon::Const0001)
    }
    #[doc = "BMI customized boot enabled"]
    #[inline(always)]
    pub fn const_0010(self) -> &'a mut crate::W<REG> {
        self.variant(Swcon::Const0010)
    }
    #[doc = "CAN BSL enabled"]
    #[inline(always)]
    pub fn const_0011(self) -> &'a mut crate::W<REG> {
        self.variant(Swcon::Const0011)
    }
    #[doc = "Boot from Code SRAM"]
    #[inline(always)]
    pub fn const_0100(self) -> &'a mut crate::W<REG> {
        self.variant(Swcon::Const0100)
    }
    #[doc = "Boot from alternate Flash Address 0"]
    #[inline(always)]
    pub fn const_1000(self) -> &'a mut crate::W<REG> {
        self.variant(Swcon::Const1000)
    }
    #[doc = "Boot from alternate Flash Address 1"]
    #[inline(always)]
    pub fn const_1100(self) -> &'a mut crate::W<REG> {
        self.variant(Swcon::Const1100)
    }
    #[doc = "Enable fallback Alternate Boot Mode (ABM)"]
    #[inline(always)]
    pub fn const_1110(self) -> &'a mut crate::W<REG> {
        self.variant(Swcon::Const1110)
    }
}
impl R {
    #[doc = "Bits 0:1 - HW Configuration"]
    #[inline(always)]
    pub fn hwcon(&self) -> HwconR {
        HwconR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:11 - SW Configuration"]
    #[inline(always)]
    pub fn swcon(&self) -> SwconR {
        SwconR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - SW Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn swcon(&mut self) -> SwconW<StconSpec> {
        SwconW::new(self, 8)
    }
}
#[doc = "Startup Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stcon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stcon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StconSpec;
impl crate::RegisterSpec for StconSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stcon::R`](R) reader structure"]
impl crate::Readable for StconSpec {}
#[doc = "`write(|w| ..)` method takes [`stcon::W`](W) writer structure"]
impl crate::Writable for StconSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STCON to value 0"]
impl crate::Resettable for StconSpec {
    const RESET_VALUE: u32 = 0;
}
