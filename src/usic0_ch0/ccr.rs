#[doc = "Register `CCR` reader"]
pub type R = crate::R<CCR_SPEC>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CCR_SPEC>;
#[doc = "Field `MODE` reader - Operating Mode"]
pub type MODE_R = crate::FieldReader<MODE_A>;
#[doc = "Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: The USIC channel is disabled. All protocol-related state machines are set to an idle state."]
    VALUE1 = 0,
    #[doc = "1: The SSC (SPI) protocol is selected."]
    VALUE2 = 1,
    #[doc = "2: The ASC (SCI, UART) protocol is selected."]
    VALUE3 = 2,
    #[doc = "3: The IIS protocol is selected."]
    VALUE4 = 3,
    #[doc = "4: The IIC protocol is selected."]
    VALUE5 = 4,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE_A {
    type Ux = u8;
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::VALUE1),
            1 => Some(MODE_A::VALUE2),
            2 => Some(MODE_A::VALUE3),
            3 => Some(MODE_A::VALUE4),
            4 => Some(MODE_A::VALUE5),
            _ => None,
        }
    }
    #[doc = "The USIC channel is disabled. All protocol-related state machines are set to an idle state."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MODE_A::VALUE1
    }
    #[doc = "The SSC (SPI) protocol is selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MODE_A::VALUE2
    }
    #[doc = "The ASC (SCI, UART) protocol is selected."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == MODE_A::VALUE3
    }
    #[doc = "The IIS protocol is selected."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == MODE_A::VALUE4
    }
    #[doc = "The IIC protocol is selected."]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == MODE_A::VALUE5
    }
}
#[doc = "Field `MODE` writer - Operating Mode"]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MODE_A>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The USIC channel is disabled. All protocol-related state machines are set to an idle state."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::VALUE1)
    }
    #[doc = "The SSC (SPI) protocol is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::VALUE2)
    }
    #[doc = "The ASC (SCI, UART) protocol is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::VALUE3)
    }
    #[doc = "The IIS protocol is selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::VALUE4)
    }
    #[doc = "The IIC protocol is selected."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::VALUE5)
    }
}
#[doc = "Field `HPCEN` reader - Hardware Port Control Enable"]
pub type HPCEN_R = crate::FieldReader<HPCEN_A>;
#[doc = "Hardware Port Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HPCEN_A {
    #[doc = "0: The hardware port control is disabled."]
    VALUE1 = 0,
    #[doc = "1: The hardware port control is enabled for DX0 and DOUT0."]
    VALUE2 = 1,
    #[doc = "2: The hardware port control is enabled for DX3, DX0 and DOUT\\[1:0\\]."]
    VALUE3 = 2,
    #[doc = "3: The hardware port control is enabled for DX0, DX\\[5:3\\]
and DOUT\\[3:0\\]."]
    VALUE4 = 3,
}
impl From<HPCEN_A> for u8 {
    #[inline(always)]
    fn from(variant: HPCEN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HPCEN_A {
    type Ux = u8;
}
impl HPCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HPCEN_A {
        match self.bits {
            0 => HPCEN_A::VALUE1,
            1 => HPCEN_A::VALUE2,
            2 => HPCEN_A::VALUE3,
            3 => HPCEN_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "The hardware port control is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HPCEN_A::VALUE1
    }
    #[doc = "The hardware port control is enabled for DX0 and DOUT0."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HPCEN_A::VALUE2
    }
    #[doc = "The hardware port control is enabled for DX3, DX0 and DOUT\\[1:0\\]."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == HPCEN_A::VALUE3
    }
    #[doc = "The hardware port control is enabled for DX0, DX\\[5:3\\]
and DOUT\\[3:0\\]."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == HPCEN_A::VALUE4
    }
}
#[doc = "Field `HPCEN` writer - Hardware Port Control Enable"]
pub type HPCEN_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, HPCEN_A>;
impl<'a, REG> HPCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The hardware port control is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HPCEN_A::VALUE1)
    }
    #[doc = "The hardware port control is enabled for DX0 and DOUT0."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HPCEN_A::VALUE2)
    }
    #[doc = "The hardware port control is enabled for DX3, DX0 and DOUT\\[1:0\\]."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(HPCEN_A::VALUE3)
    }
    #[doc = "The hardware port control is enabled for DX0, DX\\[5:3\\]
and DOUT\\[3:0\\]."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(HPCEN_A::VALUE4)
    }
}
#[doc = "Field `PM` reader - Parity Mode"]
pub type PM_R = crate::FieldReader<PM_A>;
#[doc = "Parity Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PM_A {
    #[doc = "0: The parity generation is disabled."]
    VALUE1 = 0,
    #[doc = "2: Even parity is selected (parity bit = 1 on odd number of 1s in data, parity bit = 0 on even number of 1s in data)."]
    VALUE3 = 2,
    #[doc = "3: Odd parity is selected (parity bit = 0 on odd number of 1s in data, parity bit = 1 on even number of 1s in data)."]
    VALUE4 = 3,
}
impl From<PM_A> for u8 {
    #[inline(always)]
    fn from(variant: PM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PM_A {
    type Ux = u8;
}
impl PM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PM_A> {
        match self.bits {
            0 => Some(PM_A::VALUE1),
            2 => Some(PM_A::VALUE3),
            3 => Some(PM_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "The parity generation is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PM_A::VALUE1
    }
    #[doc = "Even parity is selected (parity bit = 1 on odd number of 1s in data, parity bit = 0 on even number of 1s in data)."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PM_A::VALUE3
    }
    #[doc = "Odd parity is selected (parity bit = 0 on odd number of 1s in data, parity bit = 1 on even number of 1s in data)."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PM_A::VALUE4
    }
}
#[doc = "Field `PM` writer - Parity Mode"]
pub type PM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PM_A>;
impl<'a, REG> PM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The parity generation is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PM_A::VALUE1)
    }
    #[doc = "Even parity is selected (parity bit = 1 on odd number of 1s in data, parity bit = 0 on even number of 1s in data)."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(PM_A::VALUE3)
    }
    #[doc = "Odd parity is selected (parity bit = 0 on odd number of 1s in data, parity bit = 1 on even number of 1s in data)."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(PM_A::VALUE4)
    }
}
#[doc = "Field `RSIEN` reader - Receiver Start Interrupt Enable"]
pub type RSIEN_R = crate::BitReader<RSIEN_A>;
#[doc = "Receiver Start Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSIEN_A {
    #[doc = "0: The receiver start interrupt is disabled."]
    VALUE1 = 0,
    #[doc = "1: The receiver start interrupt is enabled. In case of a receiver start event, the service request output SRx indicated by INPR.TBINP is activated."]
    VALUE2 = 1,
}
impl From<RSIEN_A> for bool {
    #[inline(always)]
    fn from(variant: RSIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RSIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RSIEN_A {
        match self.bits {
            false => RSIEN_A::VALUE1,
            true => RSIEN_A::VALUE2,
        }
    }
    #[doc = "The receiver start interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RSIEN_A::VALUE1
    }
    #[doc = "The receiver start interrupt is enabled. In case of a receiver start event, the service request output SRx indicated by INPR.TBINP is activated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RSIEN_A::VALUE2
    }
}
#[doc = "Field `RSIEN` writer - Receiver Start Interrupt Enable"]
pub type RSIEN_W<'a, REG> = crate::BitWriter<'a, REG, RSIEN_A>;
impl<'a, REG> RSIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The receiver start interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RSIEN_A::VALUE1)
    }
    #[doc = "The receiver start interrupt is enabled. In case of a receiver start event, the service request output SRx indicated by INPR.TBINP is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RSIEN_A::VALUE2)
    }
}
#[doc = "Field `DLIEN` reader - Data Lost Interrupt Enable"]
pub type DLIEN_R = crate::BitReader<DLIEN_A>;
#[doc = "Data Lost Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLIEN_A {
    #[doc = "0: The data lost interrupt is disabled."]
    VALUE1 = 0,
    #[doc = "1: The data lost interrupt is enabled. In case of a data lost event, the service request output SRx indicated by INPR.PINP is activated."]
    VALUE2 = 1,
}
impl From<DLIEN_A> for bool {
    #[inline(always)]
    fn from(variant: DLIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DLIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DLIEN_A {
        match self.bits {
            false => DLIEN_A::VALUE1,
            true => DLIEN_A::VALUE2,
        }
    }
    #[doc = "The data lost interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DLIEN_A::VALUE1
    }
    #[doc = "The data lost interrupt is enabled. In case of a data lost event, the service request output SRx indicated by INPR.PINP is activated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DLIEN_A::VALUE2
    }
}
#[doc = "Field `DLIEN` writer - Data Lost Interrupt Enable"]
pub type DLIEN_W<'a, REG> = crate::BitWriter<'a, REG, DLIEN_A>;
impl<'a, REG> DLIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The data lost interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DLIEN_A::VALUE1)
    }
    #[doc = "The data lost interrupt is enabled. In case of a data lost event, the service request output SRx indicated by INPR.PINP is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DLIEN_A::VALUE2)
    }
}
#[doc = "Field `TSIEN` reader - Transmit Shift Interrupt Enable"]
pub type TSIEN_R = crate::BitReader<TSIEN_A>;
#[doc = "Transmit Shift Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSIEN_A {
    #[doc = "0: The transmit shift interrupt is disabled."]
    VALUE1 = 0,
    #[doc = "1: The transmit shift interrupt is enabled. In case of a transmit shift interrupt event, the service request output SRx indicated by INPR.TSINP is activated."]
    VALUE2 = 1,
}
impl From<TSIEN_A> for bool {
    #[inline(always)]
    fn from(variant: TSIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TSIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSIEN_A {
        match self.bits {
            false => TSIEN_A::VALUE1,
            true => TSIEN_A::VALUE2,
        }
    }
    #[doc = "The transmit shift interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TSIEN_A::VALUE1
    }
    #[doc = "The transmit shift interrupt is enabled. In case of a transmit shift interrupt event, the service request output SRx indicated by INPR.TSINP is activated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TSIEN_A::VALUE2
    }
}
#[doc = "Field `TSIEN` writer - Transmit Shift Interrupt Enable"]
pub type TSIEN_W<'a, REG> = crate::BitWriter<'a, REG, TSIEN_A>;
impl<'a, REG> TSIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The transmit shift interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TSIEN_A::VALUE1)
    }
    #[doc = "The transmit shift interrupt is enabled. In case of a transmit shift interrupt event, the service request output SRx indicated by INPR.TSINP is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TSIEN_A::VALUE2)
    }
}
#[doc = "Field `TBIEN` reader - Transmit Buffer Interrupt Enable"]
pub type TBIEN_R = crate::BitReader<TBIEN_A>;
#[doc = "Transmit Buffer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBIEN_A {
    #[doc = "0: The transmit buffer interrupt is disabled."]
    VALUE1 = 0,
    #[doc = "1: The transmit buffer interrupt is enabled. In case of a transmit buffer event, the service request output SRx indicated by INPR.TBINP is activated."]
    VALUE2 = 1,
}
impl From<TBIEN_A> for bool {
    #[inline(always)]
    fn from(variant: TBIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TBIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TBIEN_A {
        match self.bits {
            false => TBIEN_A::VALUE1,
            true => TBIEN_A::VALUE2,
        }
    }
    #[doc = "The transmit buffer interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TBIEN_A::VALUE1
    }
    #[doc = "The transmit buffer interrupt is enabled. In case of a transmit buffer event, the service request output SRx indicated by INPR.TBINP is activated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TBIEN_A::VALUE2
    }
}
#[doc = "Field `TBIEN` writer - Transmit Buffer Interrupt Enable"]
pub type TBIEN_W<'a, REG> = crate::BitWriter<'a, REG, TBIEN_A>;
impl<'a, REG> TBIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The transmit buffer interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TBIEN_A::VALUE1)
    }
    #[doc = "The transmit buffer interrupt is enabled. In case of a transmit buffer event, the service request output SRx indicated by INPR.TBINP is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TBIEN_A::VALUE2)
    }
}
#[doc = "Field `RIEN` reader - Receive Interrupt Enable"]
pub type RIEN_R = crate::BitReader<RIEN_A>;
#[doc = "Receive Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RIEN_A {
    #[doc = "0: The receive interrupt is disabled."]
    VALUE1 = 0,
    #[doc = "1: The receive interrupt is enabled. In case of a receive event, the service request output SRx indicated by INPR.RINP is activated."]
    VALUE2 = 1,
}
impl From<RIEN_A> for bool {
    #[inline(always)]
    fn from(variant: RIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RIEN_A {
        match self.bits {
            false => RIEN_A::VALUE1,
            true => RIEN_A::VALUE2,
        }
    }
    #[doc = "The receive interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RIEN_A::VALUE1
    }
    #[doc = "The receive interrupt is enabled. In case of a receive event, the service request output SRx indicated by INPR.RINP is activated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RIEN_A::VALUE2
    }
}
#[doc = "Field `RIEN` writer - Receive Interrupt Enable"]
pub type RIEN_W<'a, REG> = crate::BitWriter<'a, REG, RIEN_A>;
impl<'a, REG> RIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The receive interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RIEN_A::VALUE1)
    }
    #[doc = "The receive interrupt is enabled. In case of a receive event, the service request output SRx indicated by INPR.RINP is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RIEN_A::VALUE2)
    }
}
#[doc = "Field `AIEN` reader - Alternative Receive Interrupt Enable"]
pub type AIEN_R = crate::BitReader<AIEN_A>;
#[doc = "Alternative Receive Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AIEN_A {
    #[doc = "0: The alternative receive interrupt is disabled."]
    VALUE1 = 0,
    #[doc = "1: The alternative receive interrupt is enabled. In case of an alternative receive event, the service request output SRx indicated by INPR.AINP is activated."]
    VALUE2 = 1,
}
impl From<AIEN_A> for bool {
    #[inline(always)]
    fn from(variant: AIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl AIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AIEN_A {
        match self.bits {
            false => AIEN_A::VALUE1,
            true => AIEN_A::VALUE2,
        }
    }
    #[doc = "The alternative receive interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AIEN_A::VALUE1
    }
    #[doc = "The alternative receive interrupt is enabled. In case of an alternative receive event, the service request output SRx indicated by INPR.AINP is activated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AIEN_A::VALUE2
    }
}
#[doc = "Field `AIEN` writer - Alternative Receive Interrupt Enable"]
pub type AIEN_W<'a, REG> = crate::BitWriter<'a, REG, AIEN_A>;
impl<'a, REG> AIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The alternative receive interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(AIEN_A::VALUE1)
    }
    #[doc = "The alternative receive interrupt is enabled. In case of an alternative receive event, the service request output SRx indicated by INPR.AINP is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(AIEN_A::VALUE2)
    }
}
#[doc = "Field `BRGIEN` reader - Baud Rate Generator Interrupt Enable"]
pub type BRGIEN_R = crate::BitReader<BRGIEN_A>;
#[doc = "Baud Rate Generator Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRGIEN_A {
    #[doc = "0: The baud rate generator interrupt is disabled."]
    VALUE1 = 0,
    #[doc = "1: The baud rate generator interrupt is enabled. In case of a baud rate generator event, the service request output SRx indicated by INPR.PINP is activated."]
    VALUE2 = 1,
}
impl From<BRGIEN_A> for bool {
    #[inline(always)]
    fn from(variant: BRGIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BRGIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BRGIEN_A {
        match self.bits {
            false => BRGIEN_A::VALUE1,
            true => BRGIEN_A::VALUE2,
        }
    }
    #[doc = "The baud rate generator interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BRGIEN_A::VALUE1
    }
    #[doc = "The baud rate generator interrupt is enabled. In case of a baud rate generator event, the service request output SRx indicated by INPR.PINP is activated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BRGIEN_A::VALUE2
    }
}
#[doc = "Field `BRGIEN` writer - Baud Rate Generator Interrupt Enable"]
pub type BRGIEN_W<'a, REG> = crate::BitWriter<'a, REG, BRGIEN_A>;
impl<'a, REG> BRGIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The baud rate generator interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BRGIEN_A::VALUE1)
    }
    #[doc = "The baud rate generator interrupt is enabled. In case of a baud rate generator event, the service request output SRx indicated by INPR.PINP is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BRGIEN_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:3 - Operating Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - Hardware Port Control Enable"]
    #[inline(always)]
    pub fn hpcen(&self) -> HPCEN_R {
        HPCEN_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Parity Mode"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Receiver Start Interrupt Enable"]
    #[inline(always)]
    pub fn rsien(&self) -> RSIEN_R {
        RSIEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Data Lost Interrupt Enable"]
    #[inline(always)]
    pub fn dlien(&self) -> DLIEN_R {
        DLIEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit Shift Interrupt Enable"]
    #[inline(always)]
    pub fn tsien(&self) -> TSIEN_R {
        TSIEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmit Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn tbien(&self) -> TBIEN_R {
        TBIEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rien(&self) -> RIEN_R {
        RIEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Alternative Receive Interrupt Enable"]
    #[inline(always)]
    pub fn aien(&self) -> AIEN_R {
        AIEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Baud Rate Generator Interrupt Enable"]
    #[inline(always)]
    pub fn brgien(&self) -> BRGIEN_R {
        BRGIEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Operating Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CCR_SPEC> {
        MODE_W::new(self, 0)
    }
    #[doc = "Bits 6:7 - Hardware Port Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hpcen(&mut self) -> HPCEN_W<CCR_SPEC> {
        HPCEN_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Parity Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PM_W<CCR_SPEC> {
        PM_W::new(self, 8)
    }
    #[doc = "Bit 10 - Receiver Start Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rsien(&mut self) -> RSIEN_W<CCR_SPEC> {
        RSIEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Data Lost Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dlien(&mut self) -> DLIEN_W<CCR_SPEC> {
        DLIEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - Transmit Shift Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsien(&mut self) -> TSIEN_W<CCR_SPEC> {
        TSIEN_W::new(self, 12)
    }
    #[doc = "Bit 13 - Transmit Buffer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tbien(&mut self) -> TBIEN_W<CCR_SPEC> {
        TBIEN_W::new(self, 13)
    }
    #[doc = "Bit 14 - Receive Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rien(&mut self) -> RIEN_W<CCR_SPEC> {
        RIEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - Alternative Receive Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aien(&mut self) -> AIEN_W<CCR_SPEC> {
        AIEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - Baud Rate Generator Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn brgien(&mut self) -> BRGIEN_W<CCR_SPEC> {
        BRGIEN_W::new(self, 16)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
