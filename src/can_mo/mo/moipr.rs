#[doc = "Register `MOIPR` reader"]
pub type R = crate::R<MoiprSpec>;
#[doc = "Register `MOIPR` writer"]
pub type W = crate::W<MoiprSpec>;
#[doc = "Receive Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxinp {
    #[doc = "0: Interrupt output line INT_O0 is selected."]
    Value1 = 0,
    #[doc = "1: Interrupt output line INT_O1 is selected."]
    Value2 = 1,
    #[doc = "14: Interrupt output line INT_O14 is selected."]
    Value3 = 14,
    #[doc = "15: Interrupt output line INT_O15 is selected."]
    Value4 = 15,
}
impl From<Rxinp> for u8 {
    #[inline(always)]
    fn from(variant: Rxinp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rxinp {
    type Ux = u8;
}
#[doc = "Field `RXINP` reader - Receive Interrupt Node Pointer"]
pub type RxinpR = crate::FieldReader<Rxinp>;
impl RxinpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rxinp> {
        match self.bits {
            0 => Some(Rxinp::Value1),
            1 => Some(Rxinp::Value2),
            14 => Some(Rxinp::Value3),
            15 => Some(Rxinp::Value4),
            _ => None,
        }
    }
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rxinp::Value1
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rxinp::Value2
    }
    #[doc = "Interrupt output line INT_O14 is selected."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Rxinp::Value3
    }
    #[doc = "Interrupt output line INT_O15 is selected."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Rxinp::Value4
    }
}
#[doc = "Field `RXINP` writer - Receive Interrupt Node Pointer"]
pub type RxinpW<'a, REG> = crate::FieldWriter<'a, REG, 4, Rxinp>;
impl<'a, REG> RxinpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxinp::Value1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rxinp::Value2)
    }
    #[doc = "Interrupt output line INT_O14 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Rxinp::Value3)
    }
    #[doc = "Interrupt output line INT_O15 is selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Rxinp::Value4)
    }
}
#[doc = "Transmit Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txinp {
    #[doc = "0: Interrupt output line INT_O0 is selected."]
    Value1 = 0,
    #[doc = "1: Interrupt output line INT_O1 is selected."]
    Value2 = 1,
    #[doc = "14: Interrupt output line INT_O14 is selected."]
    Value3 = 14,
    #[doc = "15: Interrupt output line INT_O15 is selected."]
    Value4 = 15,
}
impl From<Txinp> for u8 {
    #[inline(always)]
    fn from(variant: Txinp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txinp {
    type Ux = u8;
}
#[doc = "Field `TXINP` reader - Transmit Interrupt Node Pointer"]
pub type TxinpR = crate::FieldReader<Txinp>;
impl TxinpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Txinp> {
        match self.bits {
            0 => Some(Txinp::Value1),
            1 => Some(Txinp::Value2),
            14 => Some(Txinp::Value3),
            15 => Some(Txinp::Value4),
            _ => None,
        }
    }
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Txinp::Value1
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Txinp::Value2
    }
    #[doc = "Interrupt output line INT_O14 is selected."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Txinp::Value3
    }
    #[doc = "Interrupt output line INT_O15 is selected."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Txinp::Value4
    }
}
#[doc = "Field `TXINP` writer - Transmit Interrupt Node Pointer"]
pub type TxinpW<'a, REG> = crate::FieldWriter<'a, REG, 4, Txinp>;
impl<'a, REG> TxinpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Txinp::Value1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Txinp::Value2)
    }
    #[doc = "Interrupt output line INT_O14 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Txinp::Value3)
    }
    #[doc = "Interrupt output line INT_O15 is selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Txinp::Value4)
    }
}
#[doc = "Field `MPN` reader - Message Pending Number"]
pub type MpnR = crate::FieldReader;
#[doc = "Field `MPN` writer - Message Pending Number"]
pub type MpnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CFCVAL` reader - CAN Frame Counter Value"]
pub type CfcvalR = crate::FieldReader<u16>;
#[doc = "Field `CFCVAL` writer - CAN Frame Counter Value"]
pub type CfcvalW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - Receive Interrupt Node Pointer"]
    #[inline(always)]
    pub fn rxinp(&self) -> RxinpR {
        RxinpR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Transmit Interrupt Node Pointer"]
    #[inline(always)]
    pub fn txinp(&self) -> TxinpR {
        TxinpR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Message Pending Number"]
    #[inline(always)]
    pub fn mpn(&self) -> MpnR {
        MpnR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - CAN Frame Counter Value"]
    #[inline(always)]
    pub fn cfcval(&self) -> CfcvalR {
        CfcvalR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Receive Interrupt Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn rxinp(&mut self) -> RxinpW<MoiprSpec> {
        RxinpW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Transmit Interrupt Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn txinp(&mut self) -> TxinpW<MoiprSpec> {
        TxinpW::new(self, 4)
    }
    #[doc = "Bits 8:15 - Message Pending Number"]
    #[inline(always)]
    #[must_use]
    pub fn mpn(&mut self) -> MpnW<MoiprSpec> {
        MpnW::new(self, 8)
    }
    #[doc = "Bits 16:31 - CAN Frame Counter Value"]
    #[inline(always)]
    #[must_use]
    pub fn cfcval(&mut self) -> CfcvalW<MoiprSpec> {
        CfcvalW::new(self, 16)
    }
}
#[doc = "Message Object Interrupt Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`moipr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`moipr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MoiprSpec;
impl crate::RegisterSpec for MoiprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`moipr::R`](R) reader structure"]
impl crate::Readable for MoiprSpec {}
#[doc = "`write(|w| ..)` method takes [`moipr::W`](W) writer structure"]
impl crate::Writable for MoiprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MOIPR to value 0"]
impl crate::Resettable for MoiprSpec {
    const RESET_VALUE: u32 = 0;
}
