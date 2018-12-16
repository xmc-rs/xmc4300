#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PCR_SSCMODE {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `MSLSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSLSENR {
    #[doc = "The MSLS generation is disabled (MSLS = 0). This is the setting for SSC slave mode."]
    VALUE1,
    #[doc = "The MSLS generation is enabled. This is the setting for SSC master mode."]
    VALUE2,
}
impl MSLSENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MSLSENR::VALUE1 => false,
            MSLSENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSLSENR {
        match value {
            false => MSLSENR::VALUE1,
            true => MSLSENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MSLSENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MSLSENR::VALUE2
    }
}
#[doc = "Possible values of the field `SELCTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELCTRR {
    #[doc = "The coded select mode is enabled."]
    VALUE1,
    #[doc = "The direct select mode is enabled."]
    VALUE2,
}
impl SELCTRR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SELCTRR::VALUE1 => false,
            SELCTRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SELCTRR {
        match value {
            false => SELCTRR::VALUE1,
            true => SELCTRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SELCTRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SELCTRR::VALUE2
    }
}
#[doc = "Possible values of the field `SELINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELINVR {
    #[doc = "The SELO outputs have the same polarity as the MSLS signal (active high)."]
    VALUE1,
    #[doc = "The SELO outputs have the inverted polarity to the MSLS signal (active low)."]
    VALUE2,
}
impl SELINVR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SELINVR::VALUE1 => false,
            SELINVR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SELINVR {
        match value {
            false => SELINVR::VALUE1,
            true => SELINVR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SELINVR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SELINVR::VALUE2
    }
}
#[doc = "Possible values of the field `FEM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEMR {
    #[doc = "The current data frame is considered as finished when the last bit of a data word has been sent out and the transmit buffer TBUF does not contain new data (TDV = 0)."]
    VALUE1,
    #[doc = "The MSLS signal is kept active also while no new data is available and no other end of frame condition is reached. In this case, the software can accept delays in delivering the data without automatic deactivation of MSLS in multi-word data frames."]
    VALUE2,
}
impl FEMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            FEMR::VALUE1 => false,
            FEMR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FEMR {
        match value {
            false => FEMR::VALUE1,
            true => FEMR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FEMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FEMR::VALUE2
    }
}
#[doc = "Possible values of the field `CTQSEL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTQSEL1R {
    #[doc = "fCTQIN = fPDIV"]
    VALUE1,
    #[doc = "fCTQIN = fPPP"]
    VALUE2,
    #[doc = "fCTQIN = fSCLK"]
    VALUE3,
    #[doc = "fCTQIN = fMCLK"]
    VALUE4,
}
impl CTQSEL1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CTQSEL1R::VALUE1 => 0,
            CTQSEL1R::VALUE2 => 1,
            CTQSEL1R::VALUE3 => 2,
            CTQSEL1R::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CTQSEL1R {
        match value {
            0 => CTQSEL1R::VALUE1,
            1 => CTQSEL1R::VALUE2,
            2 => CTQSEL1R::VALUE3,
            3 => CTQSEL1R::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CTQSEL1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CTQSEL1R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CTQSEL1R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CTQSEL1R::VALUE4
    }
}
#[doc = r" Value of the field"]
pub struct PCTQ1R {
    bits: u8,
}
impl PCTQ1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DCTQ1R {
    bits: u8,
}
impl DCTQ1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PARIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARIENR {
    #[doc = "A protocol interrupt is not generated with the detection of a parity error."]
    VALUE1,
    #[doc = "A protocol interrupt is generated with the detection of a parity error."]
    VALUE2,
}
impl PARIENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PARIENR::VALUE1 => false,
            PARIENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PARIENR {
        match value {
            false => PARIENR::VALUE1,
            true => PARIENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PARIENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PARIENR::VALUE2
    }
}
#[doc = "Possible values of the field `MSLSIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSLSIENR {
    #[doc = "A protocol interrupt is not generated if a change of signal MSLS is detected."]
    VALUE1,
    #[doc = "A protocol interrupt is generated if a change of signal MSLS is detected."]
    VALUE2,
}
impl MSLSIENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MSLSIENR::VALUE1 => false,
            MSLSIENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSLSIENR {
        match value {
            false => MSLSIENR::VALUE1,
            true => MSLSIENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MSLSIENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MSLSIENR::VALUE2
    }
}
#[doc = "Possible values of the field `DX2TIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DX2TIENR {
    #[doc = "A protocol interrupt is not generated if DX2T is activated."]
    VALUE1,
    #[doc = "A protocol interrupt is generated if DX2T is activated."]
    VALUE2,
}
impl DX2TIENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DX2TIENR::VALUE1 => false,
            DX2TIENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DX2TIENR {
        match value {
            false => DX2TIENR::VALUE1,
            true => DX2TIENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DX2TIENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DX2TIENR::VALUE2
    }
}
#[doc = "Possible values of the field `SELO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELOR {
    #[doc = "The corresponding SELOx line cannot be activated."]
    VALUE1,
    #[doc = "The corresponding SELOx line can be activated (according to the mode selected by SELCTR)."]
    VALUE2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SELOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SELOR::VALUE1 => 0,
            SELOR::VALUE2 => 1,
            SELOR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SELOR {
        match value {
            0 => SELOR::VALUE1,
            1 => SELOR::VALUE2,
            i => SELOR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SELOR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SELOR::VALUE2
    }
}
#[doc = "Possible values of the field `TIWEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIWENR {
    #[doc = "No delay between data words of the same frame."]
    VALUE1,
    #[doc = "The inter-word delay Tiw is enabled and introduced between data words of the same frame."]
    VALUE2,
}
impl TIWENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TIWENR::VALUE1 => false,
            TIWENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIWENR {
        match value {
            false => TIWENR::VALUE1,
            true => TIWENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TIWENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TIWENR::VALUE2
    }
}
#[doc = "Possible values of the field `SLPHSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLPHSELR {
    #[doc = "Data bits are shifted out with the leading edge of the shift clock signal and latched in with the trailing edge."]
    VALUE1,
    #[doc = "The first data bit is shifted out when the data shift unit receives a low to high transition from the DX2 stage. Subsequent bits are shifted out with the trailing edge of the shift clock signal. Data bits are always latched in with the leading edge."]
    VALUE2,
}
impl SLPHSELR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SLPHSELR::VALUE1 => false,
            SLPHSELR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLPHSELR {
        match value {
            false => SLPHSELR::VALUE1,
            true => SLPHSELR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SLPHSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SLPHSELR::VALUE2
    }
}
#[doc = "Possible values of the field `MCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCLKR {
    #[doc = "The MCLK generation is disabled and output MCLK = 0."]
    VALUE1,
    #[doc = "The MCLK generation is enabled."]
    VALUE2,
}
impl MCLKR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MCLKR::VALUE1 => false,
            MCLKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MCLKR {
        match value {
            false => MCLKR::VALUE1,
            true => MCLKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MCLKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MCLKR::VALUE2
    }
}
#[doc = "Values that can be written to the field `MSLSEN`"]
pub enum MSLSENW {
    #[doc = "The MSLS generation is disabled (MSLS = 0). This is the setting for SSC slave mode."]
    VALUE1,
    #[doc = "The MSLS generation is enabled. This is the setting for SSC master mode."]
    VALUE2,
}
impl MSLSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSLSENW::VALUE1 => false,
            MSLSENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSLSENW<'a> {
    w: &'a mut W,
}
impl<'a> _MSLSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSLSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The MSLS generation is disabled (MSLS = 0). This is the setting for SSC slave mode."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSLSENW::VALUE1)
    }
    #[doc = "The MSLS generation is enabled. This is the setting for SSC master mode."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSLSENW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SELCTR`"]
pub enum SELCTRW {
    #[doc = "The coded select mode is enabled."]
    VALUE1,
    #[doc = "The direct select mode is enabled."]
    VALUE2,
}
impl SELCTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SELCTRW::VALUE1 => false,
            SELCTRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELCTRW<'a> {
    w: &'a mut W,
}
impl<'a> _SELCTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELCTRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The coded select mode is enabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SELCTRW::VALUE1)
    }
    #[doc = "The direct select mode is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SELCTRW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SELINV`"]
pub enum SELINVW {
    #[doc = "The SELO outputs have the same polarity as the MSLS signal (active high)."]
    VALUE1,
    #[doc = "The SELO outputs have the inverted polarity to the MSLS signal (active low)."]
    VALUE2,
}
impl SELINVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SELINVW::VALUE1 => false,
            SELINVW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELINVW<'a> {
    w: &'a mut W,
}
impl<'a> _SELINVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELINVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The SELO outputs have the same polarity as the MSLS signal (active high)."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SELINVW::VALUE1)
    }
    #[doc = "The SELO outputs have the inverted polarity to the MSLS signal (active low)."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SELINVW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FEM`"]
pub enum FEMW {
    #[doc = "The current data frame is considered as finished when the last bit of a data word has been sent out and the transmit buffer TBUF does not contain new data (TDV = 0)."]
    VALUE1,
    #[doc = "The MSLS signal is kept active also while no new data is available and no other end of frame condition is reached. In this case, the software can accept delays in delivering the data without automatic deactivation of MSLS in multi-word data frames."]
    VALUE2,
}
impl FEMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FEMW::VALUE1 => false,
            FEMW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FEMW<'a> {
    w: &'a mut W,
}
impl<'a> _FEMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FEMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The current data frame is considered as finished when the last bit of a data word has been sent out and the transmit buffer TBUF does not contain new data (TDV = 0)."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FEMW::VALUE1)
    }
    #[doc = "The MSLS signal is kept active also while no new data is available and no other end of frame condition is reached. In this case, the software can accept delays in delivering the data without automatic deactivation of MSLS in multi-word data frames."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FEMW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CTQSEL1`"]
pub enum CTQSEL1W {
    #[doc = "fCTQIN = fPDIV"]
    VALUE1,
    #[doc = "fCTQIN = fPPP"]
    VALUE2,
    #[doc = "fCTQIN = fSCLK"]
    VALUE3,
    #[doc = "fCTQIN = fMCLK"]
    VALUE4,
}
impl CTQSEL1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CTQSEL1W::VALUE1 => 0,
            CTQSEL1W::VALUE2 => 1,
            CTQSEL1W::VALUE3 => 2,
            CTQSEL1W::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTQSEL1W<'a> {
    w: &'a mut W,
}
impl<'a> _CTQSEL1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTQSEL1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "fCTQIN = fPDIV"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CTQSEL1W::VALUE1)
    }
    #[doc = "fCTQIN = fPPP"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CTQSEL1W::VALUE2)
    }
    #[doc = "fCTQIN = fSCLK"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CTQSEL1W::VALUE3)
    }
    #[doc = "fCTQIN = fMCLK"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CTQSEL1W::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PCTQ1W<'a> {
    w: &'a mut W,
}
impl<'a> _PCTQ1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DCTQ1W<'a> {
    w: &'a mut W,
}
impl<'a> _DCTQ1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PARIEN`"]
pub enum PARIENW {
    #[doc = "A protocol interrupt is not generated with the detection of a parity error."]
    VALUE1,
    #[doc = "A protocol interrupt is generated with the detection of a parity error."]
    VALUE2,
}
impl PARIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PARIENW::VALUE1 => false,
            PARIENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PARIENW<'a> {
    w: &'a mut W,
}
impl<'a> _PARIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PARIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A protocol interrupt is not generated with the detection of a parity error."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PARIENW::VALUE1)
    }
    #[doc = "A protocol interrupt is generated with the detection of a parity error."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PARIENW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MSLSIEN`"]
pub enum MSLSIENW {
    #[doc = "A protocol interrupt is not generated if a change of signal MSLS is detected."]
    VALUE1,
    #[doc = "A protocol interrupt is generated if a change of signal MSLS is detected."]
    VALUE2,
}
impl MSLSIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSLSIENW::VALUE1 => false,
            MSLSIENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSLSIENW<'a> {
    w: &'a mut W,
}
impl<'a> _MSLSIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSLSIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A protocol interrupt is not generated if a change of signal MSLS is detected."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSLSIENW::VALUE1)
    }
    #[doc = "A protocol interrupt is generated if a change of signal MSLS is detected."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSLSIENW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DX2TIEN`"]
pub enum DX2TIENW {
    #[doc = "A protocol interrupt is not generated if DX2T is activated."]
    VALUE1,
    #[doc = "A protocol interrupt is generated if DX2T is activated."]
    VALUE2,
}
impl DX2TIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DX2TIENW::VALUE1 => false,
            DX2TIENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DX2TIENW<'a> {
    w: &'a mut W,
}
impl<'a> _DX2TIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DX2TIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A protocol interrupt is not generated if DX2T is activated."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DX2TIENW::VALUE1)
    }
    #[doc = "A protocol interrupt is generated if DX2T is activated."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DX2TIENW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SELO`"]
pub enum SELOW {
    #[doc = "The corresponding SELOx line cannot be activated."]
    VALUE1,
    #[doc = "The corresponding SELOx line can be activated (according to the mode selected by SELCTR)."]
    VALUE2,
}
impl SELOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SELOW::VALUE1 => 0,
            SELOW::VALUE2 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELOW<'a> {
    w: &'a mut W,
}
impl<'a> _SELOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELOW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The corresponding SELOx line cannot be activated."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SELOW::VALUE1)
    }
    #[doc = "The corresponding SELOx line can be activated (according to the mode selected by SELCTR)."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SELOW::VALUE2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIWEN`"]
pub enum TIWENW {
    #[doc = "No delay between data words of the same frame."]
    VALUE1,
    #[doc = "The inter-word delay Tiw is enabled and introduced between data words of the same frame."]
    VALUE2,
}
impl TIWENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIWENW::VALUE1 => false,
            TIWENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIWENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIWENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIWENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No delay between data words of the same frame."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TIWENW::VALUE1)
    }
    #[doc = "The inter-word delay Tiw is enabled and introduced between data words of the same frame."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TIWENW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SLPHSEL`"]
pub enum SLPHSELW {
    #[doc = "Data bits are shifted out with the leading edge of the shift clock signal and latched in with the trailing edge."]
    VALUE1,
    #[doc = "The first data bit is shifted out when the data shift unit receives a low to high transition from the DX2 stage. Subsequent bits are shifted out with the trailing edge of the shift clock signal. Data bits are always latched in with the leading edge."]
    VALUE2,
}
impl SLPHSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLPHSELW::VALUE1 => false,
            SLPHSELW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLPHSELW<'a> {
    w: &'a mut W,
}
impl<'a> _SLPHSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLPHSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data bits are shifted out with the leading edge of the shift clock signal and latched in with the trailing edge."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SLPHSELW::VALUE1)
    }
    #[doc = "The first data bit is shifted out when the data shift unit receives a low to high transition from the DX2 stage. Subsequent bits are shifted out with the trailing edge of the shift clock signal. Data bits are always latched in with the leading edge."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SLPHSELW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MCLK`"]
pub enum MCLKW {
    #[doc = "The MCLK generation is disabled and output MCLK = 0."]
    VALUE1,
    #[doc = "The MCLK generation is enabled."]
    VALUE2,
}
impl MCLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MCLKW::VALUE1 => false,
            MCLKW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _MCLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCLKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The MCLK generation is disabled and output MCLK = 0."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MCLKW::VALUE1)
    }
    #[doc = "The MCLK generation is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MCLKW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 31;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - MSLS Enable"]
    #[inline]
    pub fn mslsen(&self) -> MSLSENR {
        MSLSENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Select Control"]
    #[inline]
    pub fn selctr(&self) -> SELCTRR {
        SELCTRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Select Inversion"]
    #[inline]
    pub fn selinv(&self) -> SELINVR {
        SELINVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Frame End Mode"]
    #[inline]
    pub fn fem(&self) -> FEMR {
        FEMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:5 - Input Frequency Selection"]
    #[inline]
    pub fn ctqsel1(&self) -> CTQSEL1R {
        CTQSEL1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Divider Factor PCTQ1 for Tiw and Tnf"]
    #[inline]
    pub fn pctq1(&self) -> PCTQ1R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PCTQ1R { bits }
    }
    #[doc = "Bits 8:12 - Divider Factor DCTQ1 for Tiw and Tnf"]
    #[inline]
    pub fn dctq1(&self) -> DCTQ1R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DCTQ1R { bits }
    }
    #[doc = "Bit 13 - Parity Error Interrupt Enable"]
    #[inline]
    pub fn parien(&self) -> PARIENR {
        PARIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - MSLS Interrupt Enable"]
    #[inline]
    pub fn mslsien(&self) -> MSLSIENR {
        MSLSIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - DX2T Interrupt Enable"]
    #[inline]
    pub fn dx2tien(&self) -> DX2TIENR {
        DX2TIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:23 - Select Output"]
    #[inline]
    pub fn selo(&self) -> SELOR {
        SELOR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 24 - Enable Inter-Word Delay Tiw"]
    #[inline]
    pub fn tiwen(&self) -> TIWENR {
        TIWENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Slave Mode Clock Phase Select"]
    #[inline]
    pub fn slphsel(&self) -> SLPHSELR {
        SLPHSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Master Clock Enable"]
    #[inline]
    pub fn mclk(&self) -> MCLKR {
        MCLKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - MSLS Enable"]
    #[inline]
    pub fn mslsen(&mut self) -> _MSLSENW {
        _MSLSENW { w: self }
    }
    #[doc = "Bit 1 - Select Control"]
    #[inline]
    pub fn selctr(&mut self) -> _SELCTRW {
        _SELCTRW { w: self }
    }
    #[doc = "Bit 2 - Select Inversion"]
    #[inline]
    pub fn selinv(&mut self) -> _SELINVW {
        _SELINVW { w: self }
    }
    #[doc = "Bit 3 - Frame End Mode"]
    #[inline]
    pub fn fem(&mut self) -> _FEMW {
        _FEMW { w: self }
    }
    #[doc = "Bits 4:5 - Input Frequency Selection"]
    #[inline]
    pub fn ctqsel1(&mut self) -> _CTQSEL1W {
        _CTQSEL1W { w: self }
    }
    #[doc = "Bits 6:7 - Divider Factor PCTQ1 for Tiw and Tnf"]
    #[inline]
    pub fn pctq1(&mut self) -> _PCTQ1W {
        _PCTQ1W { w: self }
    }
    #[doc = "Bits 8:12 - Divider Factor DCTQ1 for Tiw and Tnf"]
    #[inline]
    pub fn dctq1(&mut self) -> _DCTQ1W {
        _DCTQ1W { w: self }
    }
    #[doc = "Bit 13 - Parity Error Interrupt Enable"]
    #[inline]
    pub fn parien(&mut self) -> _PARIENW {
        _PARIENW { w: self }
    }
    #[doc = "Bit 14 - MSLS Interrupt Enable"]
    #[inline]
    pub fn mslsien(&mut self) -> _MSLSIENW {
        _MSLSIENW { w: self }
    }
    #[doc = "Bit 15 - DX2T Interrupt Enable"]
    #[inline]
    pub fn dx2tien(&mut self) -> _DX2TIENW {
        _DX2TIENW { w: self }
    }
    #[doc = "Bits 16:23 - Select Output"]
    #[inline]
    pub fn selo(&mut self) -> _SELOW {
        _SELOW { w: self }
    }
    #[doc = "Bit 24 - Enable Inter-Word Delay Tiw"]
    #[inline]
    pub fn tiwen(&mut self) -> _TIWENW {
        _TIWENW { w: self }
    }
    #[doc = "Bit 25 - Slave Mode Clock Phase Select"]
    #[inline]
    pub fn slphsel(&mut self) -> _SLPHSELW {
        _SLPHSELW { w: self }
    }
    #[doc = "Bit 31 - Master Clock Enable"]
    #[inline]
    pub fn mclk(&mut self) -> _MCLKW {
        _MCLKW { w: self }
    }
}
