#[doc = "Register `STCON` reader"]
pub type R = crate::R<STCON_SPEC>;
#[doc = "Register `STCON` writer"]
pub type W = crate::W<STCON_SPEC>;
#[doc = "HW Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HWCON_A {
    #[doc = "0: Normal mode, JTAG"]
    CONST_00 = 0,
    #[doc = "1: ASC BSL enabled"]
    CONST_01 = 1,
    #[doc = "2: BMI customized boot enabled"]
    CONST_10 = 2,
    #[doc = "3: CAN BSL enabled"]
    CONST_11 = 3,
}
impl From<HWCON_A> for u8 {
    #[inline(always)]
    fn from(variant: HWCON_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HWCON_A {
    type Ux = u8;
}
impl crate::IsEnum for HWCON_A {}
#[doc = "Field `HWCON` reader - HW Configuration"]
pub type HWCON_R = crate::FieldReader<HWCON_A>;
impl HWCON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HWCON_A {
        match self.bits {
            0 => HWCON_A::CONST_00,
            1 => HWCON_A::CONST_01,
            2 => HWCON_A::CONST_10,
            3 => HWCON_A::CONST_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal mode, JTAG"]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == HWCON_A::CONST_00
    }
    #[doc = "ASC BSL enabled"]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == HWCON_A::CONST_01
    }
    #[doc = "BMI customized boot enabled"]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == HWCON_A::CONST_10
    }
    #[doc = "CAN BSL enabled"]
    #[inline(always)]
    pub fn is_const_11(&self) -> bool {
        *self == HWCON_A::CONST_11
    }
}
#[doc = "SW Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SWCON_A {
    #[doc = "0: Normal mode, boot from Boot ROM"]
    CONST_0000 = 0,
    #[doc = "1: ASC BSL enabled"]
    CONST_0001 = 1,
    #[doc = "2: BMI customized boot enabled"]
    CONST_0010 = 2,
    #[doc = "3: CAN BSL enabled"]
    CONST_0011 = 3,
    #[doc = "4: Boot from Code SRAM"]
    CONST_0100 = 4,
    #[doc = "8: Boot from alternate Flash Address 0"]
    CONST_1000 = 8,
    #[doc = "12: Boot from alternate Flash Address 1"]
    CONST_1100 = 12,
    #[doc = "14: Enable fallback Alternate Boot Mode (ABM)"]
    CONST_1110 = 14,
}
impl From<SWCON_A> for u8 {
    #[inline(always)]
    fn from(variant: SWCON_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SWCON_A {
    type Ux = u8;
}
impl crate::IsEnum for SWCON_A {}
#[doc = "Field `SWCON` reader - SW Configuration"]
pub type SWCON_R = crate::FieldReader<SWCON_A>;
impl SWCON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SWCON_A> {
        match self.bits {
            0 => Some(SWCON_A::CONST_0000),
            1 => Some(SWCON_A::CONST_0001),
            2 => Some(SWCON_A::CONST_0010),
            3 => Some(SWCON_A::CONST_0011),
            4 => Some(SWCON_A::CONST_0100),
            8 => Some(SWCON_A::CONST_1000),
            12 => Some(SWCON_A::CONST_1100),
            14 => Some(SWCON_A::CONST_1110),
            _ => None,
        }
    }
    #[doc = "Normal mode, boot from Boot ROM"]
    #[inline(always)]
    pub fn is_const_0000(&self) -> bool {
        *self == SWCON_A::CONST_0000
    }
    #[doc = "ASC BSL enabled"]
    #[inline(always)]
    pub fn is_const_0001(&self) -> bool {
        *self == SWCON_A::CONST_0001
    }
    #[doc = "BMI customized boot enabled"]
    #[inline(always)]
    pub fn is_const_0010(&self) -> bool {
        *self == SWCON_A::CONST_0010
    }
    #[doc = "CAN BSL enabled"]
    #[inline(always)]
    pub fn is_const_0011(&self) -> bool {
        *self == SWCON_A::CONST_0011
    }
    #[doc = "Boot from Code SRAM"]
    #[inline(always)]
    pub fn is_const_0100(&self) -> bool {
        *self == SWCON_A::CONST_0100
    }
    #[doc = "Boot from alternate Flash Address 0"]
    #[inline(always)]
    pub fn is_const_1000(&self) -> bool {
        *self == SWCON_A::CONST_1000
    }
    #[doc = "Boot from alternate Flash Address 1"]
    #[inline(always)]
    pub fn is_const_1100(&self) -> bool {
        *self == SWCON_A::CONST_1100
    }
    #[doc = "Enable fallback Alternate Boot Mode (ABM)"]
    #[inline(always)]
    pub fn is_const_1110(&self) -> bool {
        *self == SWCON_A::CONST_1110
    }
}
#[doc = "Field `SWCON` writer - SW Configuration"]
pub type SWCON_W<'a, REG> = crate::FieldWriter<'a, REG, 4, SWCON_A>;
impl<'a, REG> SWCON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal mode, boot from Boot ROM"]
    #[inline(always)]
    pub fn const_0000(self) -> &'a mut crate::W<REG> {
        self.variant(SWCON_A::CONST_0000)
    }
    #[doc = "ASC BSL enabled"]
    #[inline(always)]
    pub fn const_0001(self) -> &'a mut crate::W<REG> {
        self.variant(SWCON_A::CONST_0001)
    }
    #[doc = "BMI customized boot enabled"]
    #[inline(always)]
    pub fn const_0010(self) -> &'a mut crate::W<REG> {
        self.variant(SWCON_A::CONST_0010)
    }
    #[doc = "CAN BSL enabled"]
    #[inline(always)]
    pub fn const_0011(self) -> &'a mut crate::W<REG> {
        self.variant(SWCON_A::CONST_0011)
    }
    #[doc = "Boot from Code SRAM"]
    #[inline(always)]
    pub fn const_0100(self) -> &'a mut crate::W<REG> {
        self.variant(SWCON_A::CONST_0100)
    }
    #[doc = "Boot from alternate Flash Address 0"]
    #[inline(always)]
    pub fn const_1000(self) -> &'a mut crate::W<REG> {
        self.variant(SWCON_A::CONST_1000)
    }
    #[doc = "Boot from alternate Flash Address 1"]
    #[inline(always)]
    pub fn const_1100(self) -> &'a mut crate::W<REG> {
        self.variant(SWCON_A::CONST_1100)
    }
    #[doc = "Enable fallback Alternate Boot Mode (ABM)"]
    #[inline(always)]
    pub fn const_1110(self) -> &'a mut crate::W<REG> {
        self.variant(SWCON_A::CONST_1110)
    }
}
impl R {
    #[doc = "Bits 0:1 - HW Configuration"]
    #[inline(always)]
    pub fn hwcon(&self) -> HWCON_R {
        HWCON_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:11 - SW Configuration"]
    #[inline(always)]
    pub fn swcon(&self) -> SWCON_R {
        SWCON_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - SW Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn swcon(&mut self) -> SWCON_W<STCON_SPEC> {
        SWCON_W::new(self, 8)
    }
}
#[doc = "Startup Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stcon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stcon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STCON_SPEC;
impl crate::RegisterSpec for STCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stcon::R`](R) reader structure"]
impl crate::Readable for STCON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stcon::W`](W) writer structure"]
impl crate::Writable for STCON_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STCON to value 0"]
impl crate::Resettable for STCON_SPEC {
    const RESET_VALUE: u32 = 0;
}
