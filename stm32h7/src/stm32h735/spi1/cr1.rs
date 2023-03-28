#[doc = "Register `CR1` reader"]
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR1` writer"]
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPE` reader - Serial Peripheral Enable"]
pub type SPE_R = crate::BitReader<SPE_A>;
#[doc = "Serial Peripheral Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPE_A {
    #[doc = "0: Peripheral disabled"]
    Disabled = 0,
    #[doc = "1: Peripheral enabled"]
    Enabled = 1,
}
impl From<SPE_A> for bool {
    #[inline(always)]
    fn from(variant: SPE_A) -> Self {
        variant as u8 != 0
    }
}
impl SPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPE_A {
        match self.bits {
            false => SPE_A::Disabled,
            true => SPE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SPE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SPE_A::Enabled
    }
}
#[doc = "Field `SPE` writer - Serial Peripheral Enable"]
pub type SPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, SPE_A, O>;
impl<'a, const O: u8> SPE_W<'a, O> {
    #[doc = "Peripheral disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SPE_A::Disabled)
    }
    #[doc = "Peripheral enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SPE_A::Enabled)
    }
}
#[doc = "Field `MASRX` reader - Master automatic SUSP in Receive mode"]
pub type MASRX_R = crate::BitReader<MASRX_A>;
#[doc = "Master automatic SUSP in Receive mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASRX_A {
    #[doc = "0: Automatic suspend in master receive-only mode disabled"]
    Disabled = 0,
    #[doc = "1: Automatic suspend in master receive-only mode enabled"]
    Enabled = 1,
}
impl From<MASRX_A> for bool {
    #[inline(always)]
    fn from(variant: MASRX_A) -> Self {
        variant as u8 != 0
    }
}
impl MASRX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASRX_A {
        match self.bits {
            false => MASRX_A::Disabled,
            true => MASRX_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MASRX_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MASRX_A::Enabled
    }
}
#[doc = "Field `MASRX` writer - Master automatic SUSP in Receive mode"]
pub type MASRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, MASRX_A, O>;
impl<'a, const O: u8> MASRX_W<'a, O> {
    #[doc = "Automatic suspend in master receive-only mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MASRX_A::Disabled)
    }
    #[doc = "Automatic suspend in master receive-only mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MASRX_A::Enabled)
    }
}
#[doc = "Field `CSTART` reader - Master transfer start"]
pub type CSTART_R = crate::BitReader<CSTART_A>;
#[doc = "Master transfer start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTART_A {
    #[doc = "0: Do not start master transfer"]
    NotStarted = 0,
    #[doc = "1: Start master transfer"]
    Started = 1,
}
impl From<CSTART_A> for bool {
    #[inline(always)]
    fn from(variant: CSTART_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTART_A {
        match self.bits {
            false => CSTART_A::NotStarted,
            true => CSTART_A::Started,
        }
    }
    #[doc = "Checks if the value of the field is `NotStarted`"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == CSTART_A::NotStarted
    }
    #[doc = "Checks if the value of the field is `Started`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == CSTART_A::Started
    }
}
#[doc = "Field `CSTART` writer - Master transfer start"]
pub type CSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, CSTART_A, O>;
impl<'a, const O: u8> CSTART_W<'a, O> {
    #[doc = "Do not start master transfer"]
    #[inline(always)]
    pub fn not_started(self) -> &'a mut W {
        self.variant(CSTART_A::NotStarted)
    }
    #[doc = "Start master transfer"]
    #[inline(always)]
    pub fn started(self) -> &'a mut W {
        self.variant(CSTART_A::Started)
    }
}
#[doc = "Master SUSPend request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSUSPW_AW {
    #[doc = "0: Do not request master suspend"]
    NotRequested = 0,
    #[doc = "1: Request master suspend"]
    Requested = 1,
}
impl From<CSUSPW_AW> for bool {
    #[inline(always)]
    fn from(variant: CSUSPW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSUSP` writer - Master SUSPend request"]
pub type CSUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, CSUSPW_AW, O>;
impl<'a, const O: u8> CSUSP_W<'a, O> {
    #[doc = "Do not request master suspend"]
    #[inline(always)]
    pub fn not_requested(self) -> &'a mut W {
        self.variant(CSUSPW_AW::NotRequested)
    }
    #[doc = "Request master suspend"]
    #[inline(always)]
    pub fn requested(self) -> &'a mut W {
        self.variant(CSUSPW_AW::Requested)
    }
}
#[doc = "Field `HDDIR` reader - Rx/Tx direction at Half-duplex mode"]
pub type HDDIR_R = crate::BitReader<HDDIR_A>;
#[doc = "Rx/Tx direction at Half-duplex mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDDIR_A {
    #[doc = "0: Receiver in half duplex mode"]
    Receiver = 0,
    #[doc = "1: Transmitter in half duplex mode"]
    Transmitter = 1,
}
impl From<HDDIR_A> for bool {
    #[inline(always)]
    fn from(variant: HDDIR_A) -> Self {
        variant as u8 != 0
    }
}
impl HDDIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDDIR_A {
        match self.bits {
            false => HDDIR_A::Receiver,
            true => HDDIR_A::Transmitter,
        }
    }
    #[doc = "Checks if the value of the field is `Receiver`"]
    #[inline(always)]
    pub fn is_receiver(&self) -> bool {
        *self == HDDIR_A::Receiver
    }
    #[doc = "Checks if the value of the field is `Transmitter`"]
    #[inline(always)]
    pub fn is_transmitter(&self) -> bool {
        *self == HDDIR_A::Transmitter
    }
}
#[doc = "Field `HDDIR` writer - Rx/Tx direction at Half-duplex mode"]
pub type HDDIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, HDDIR_A, O>;
impl<'a, const O: u8> HDDIR_W<'a, O> {
    #[doc = "Receiver in half duplex mode"]
    #[inline(always)]
    pub fn receiver(self) -> &'a mut W {
        self.variant(HDDIR_A::Receiver)
    }
    #[doc = "Transmitter in half duplex mode"]
    #[inline(always)]
    pub fn transmitter(self) -> &'a mut W {
        self.variant(HDDIR_A::Transmitter)
    }
}
#[doc = "Field `SSI` reader - Internal SS signal input level"]
pub type SSI_R = crate::BitReader<SSI_A>;
#[doc = "Internal SS signal input level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSI_A {
    #[doc = "0: 0 is forced onto the SS signal and the I/O value of the SS pin is ignored"]
    SlaveSelected = 0,
    #[doc = "1: 1 is forced onto the SS signal and the I/O value of the SS pin is ignored"]
    SlaveNotSelected = 1,
}
impl From<SSI_A> for bool {
    #[inline(always)]
    fn from(variant: SSI_A) -> Self {
        variant as u8 != 0
    }
}
impl SSI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSI_A {
        match self.bits {
            false => SSI_A::SlaveSelected,
            true => SSI_A::SlaveNotSelected,
        }
    }
    #[doc = "Checks if the value of the field is `SlaveSelected`"]
    #[inline(always)]
    pub fn is_slave_selected(&self) -> bool {
        *self == SSI_A::SlaveSelected
    }
    #[doc = "Checks if the value of the field is `SlaveNotSelected`"]
    #[inline(always)]
    pub fn is_slave_not_selected(&self) -> bool {
        *self == SSI_A::SlaveNotSelected
    }
}
#[doc = "Field `SSI` writer - Internal SS signal input level"]
pub type SSI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, SSI_A, O>;
impl<'a, const O: u8> SSI_W<'a, O> {
    #[doc = "0 is forced onto the SS signal and the I/O value of the SS pin is ignored"]
    #[inline(always)]
    pub fn slave_selected(self) -> &'a mut W {
        self.variant(SSI_A::SlaveSelected)
    }
    #[doc = "1 is forced onto the SS signal and the I/O value of the SS pin is ignored"]
    #[inline(always)]
    pub fn slave_not_selected(self) -> &'a mut W {
        self.variant(SSI_A::SlaveNotSelected)
    }
}
#[doc = "Field `CRC33_17` reader - 32-bit CRC polynomial configuration"]
pub type CRC33_17_R = crate::BitReader<CRC33_17_A>;
#[doc = "32-bit CRC polynomial configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRC33_17_A {
    #[doc = "0: Full size (33/17 bit) CRC polynomial is not used"]
    Disabled = 0,
    #[doc = "1: Full size (33/17 bit) CRC polynomial is used"]
    Enabled = 1,
}
impl From<CRC33_17_A> for bool {
    #[inline(always)]
    fn from(variant: CRC33_17_A) -> Self {
        variant as u8 != 0
    }
}
impl CRC33_17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC33_17_A {
        match self.bits {
            false => CRC33_17_A::Disabled,
            true => CRC33_17_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRC33_17_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRC33_17_A::Enabled
    }
}
#[doc = "Field `CRC33_17` writer - 32-bit CRC polynomial configuration"]
pub type CRC33_17_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, CRC33_17_A, O>;
impl<'a, const O: u8> CRC33_17_W<'a, O> {
    #[doc = "Full size (33/17 bit) CRC polynomial is not used"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRC33_17_A::Disabled)
    }
    #[doc = "Full size (33/17 bit) CRC polynomial is used"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRC33_17_A::Enabled)
    }
}
#[doc = "Field `RCRCINI` reader - CRC calculation initialization pattern control for receiver"]
pub type RCRCINI_R = crate::BitReader<RCRCINI_A>;
#[doc = "CRC calculation initialization pattern control for receiver\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCRCINI_A {
    #[doc = "0: All zeros RX CRC initialization pattern"]
    AllZeros = 0,
    #[doc = "1: All ones RX CRC initialization pattern"]
    AllOnes = 1,
}
impl From<RCRCINI_A> for bool {
    #[inline(always)]
    fn from(variant: RCRCINI_A) -> Self {
        variant as u8 != 0
    }
}
impl RCRCINI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCRCINI_A {
        match self.bits {
            false => RCRCINI_A::AllZeros,
            true => RCRCINI_A::AllOnes,
        }
    }
    #[doc = "Checks if the value of the field is `AllZeros`"]
    #[inline(always)]
    pub fn is_all_zeros(&self) -> bool {
        *self == RCRCINI_A::AllZeros
    }
    #[doc = "Checks if the value of the field is `AllOnes`"]
    #[inline(always)]
    pub fn is_all_ones(&self) -> bool {
        *self == RCRCINI_A::AllOnes
    }
}
#[doc = "Field `RCRCINI` writer - CRC calculation initialization pattern control for receiver"]
pub type RCRCINI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, RCRCINI_A, O>;
impl<'a, const O: u8> RCRCINI_W<'a, O> {
    #[doc = "All zeros RX CRC initialization pattern"]
    #[inline(always)]
    pub fn all_zeros(self) -> &'a mut W {
        self.variant(RCRCINI_A::AllZeros)
    }
    #[doc = "All ones RX CRC initialization pattern"]
    #[inline(always)]
    pub fn all_ones(self) -> &'a mut W {
        self.variant(RCRCINI_A::AllOnes)
    }
}
#[doc = "Field `TCRCINI` reader - CRC calculation initialization pattern control for transmitter"]
pub type TCRCINI_R = crate::BitReader<TCRCINI_A>;
#[doc = "CRC calculation initialization pattern control for transmitter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCRCINI_A {
    #[doc = "0: All zeros TX CRC initialization pattern"]
    AllZeros = 0,
    #[doc = "1: All ones TX CRC initialization pattern"]
    AllOnes = 1,
}
impl From<TCRCINI_A> for bool {
    #[inline(always)]
    fn from(variant: TCRCINI_A) -> Self {
        variant as u8 != 0
    }
}
impl TCRCINI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCRCINI_A {
        match self.bits {
            false => TCRCINI_A::AllZeros,
            true => TCRCINI_A::AllOnes,
        }
    }
    #[doc = "Checks if the value of the field is `AllZeros`"]
    #[inline(always)]
    pub fn is_all_zeros(&self) -> bool {
        *self == TCRCINI_A::AllZeros
    }
    #[doc = "Checks if the value of the field is `AllOnes`"]
    #[inline(always)]
    pub fn is_all_ones(&self) -> bool {
        *self == TCRCINI_A::AllOnes
    }
}
#[doc = "Field `TCRCINI` writer - CRC calculation initialization pattern control for transmitter"]
pub type TCRCINI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, TCRCINI_A, O>;
impl<'a, const O: u8> TCRCINI_W<'a, O> {
    #[doc = "All zeros TX CRC initialization pattern"]
    #[inline(always)]
    pub fn all_zeros(self) -> &'a mut W {
        self.variant(TCRCINI_A::AllZeros)
    }
    #[doc = "All ones TX CRC initialization pattern"]
    #[inline(always)]
    pub fn all_ones(self) -> &'a mut W {
        self.variant(TCRCINI_A::AllOnes)
    }
}
#[doc = "Field `IOLOCK` reader - Locking the AF configuration of associated IOs"]
pub type IOLOCK_R = crate::BitReader<IOLOCK_A>;
#[doc = "Locking the AF configuration of associated IOs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOLOCK_A {
    #[doc = "0: IO configuration unlocked"]
    Unlocked = 0,
    #[doc = "1: IO configuration locked"]
    Locked = 1,
}
impl From<IOLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: IOLOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl IOLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOLOCK_A {
        match self.bits {
            false => IOLOCK_A::Unlocked,
            true => IOLOCK_A::Locked,
        }
    }
    #[doc = "Checks if the value of the field is `Unlocked`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == IOLOCK_A::Unlocked
    }
    #[doc = "Checks if the value of the field is `Locked`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == IOLOCK_A::Locked
    }
}
impl R {
    #[doc = "Bit 0 - Serial Peripheral Enable"]
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Master automatic SUSP in Receive mode"]
    #[inline(always)]
    pub fn masrx(&self) -> MASRX_R {
        MASRX_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Master transfer start"]
    #[inline(always)]
    pub fn cstart(&self) -> CSTART_R {
        CSTART_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Rx/Tx direction at Half-duplex mode"]
    #[inline(always)]
    pub fn hddir(&self) -> HDDIR_R {
        HDDIR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Internal SS signal input level"]
    #[inline(always)]
    pub fn ssi(&self) -> SSI_R {
        SSI_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 32-bit CRC polynomial configuration"]
    #[inline(always)]
    pub fn crc33_17(&self) -> CRC33_17_R {
        CRC33_17_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CRC calculation initialization pattern control for receiver"]
    #[inline(always)]
    pub fn rcrcini(&self) -> RCRCINI_R {
        RCRCINI_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CRC calculation initialization pattern control for transmitter"]
    #[inline(always)]
    pub fn tcrcini(&self) -> TCRCINI_R {
        TCRCINI_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Locking the AF configuration of associated IOs"]
    #[inline(always)]
    pub fn iolock(&self) -> IOLOCK_R {
        IOLOCK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Serial Peripheral Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spe(&mut self) -> SPE_W<0> {
        SPE_W::new(self)
    }
    #[doc = "Bit 8 - Master automatic SUSP in Receive mode"]
    #[inline(always)]
    #[must_use]
    pub fn masrx(&mut self) -> MASRX_W<8> {
        MASRX_W::new(self)
    }
    #[doc = "Bit 9 - Master transfer start"]
    #[inline(always)]
    #[must_use]
    pub fn cstart(&mut self) -> CSTART_W<9> {
        CSTART_W::new(self)
    }
    #[doc = "Bit 10 - Master SUSPend request"]
    #[inline(always)]
    #[must_use]
    pub fn csusp(&mut self) -> CSUSP_W<10> {
        CSUSP_W::new(self)
    }
    #[doc = "Bit 11 - Rx/Tx direction at Half-duplex mode"]
    #[inline(always)]
    #[must_use]
    pub fn hddir(&mut self) -> HDDIR_W<11> {
        HDDIR_W::new(self)
    }
    #[doc = "Bit 12 - Internal SS signal input level"]
    #[inline(always)]
    #[must_use]
    pub fn ssi(&mut self) -> SSI_W<12> {
        SSI_W::new(self)
    }
    #[doc = "Bit 13 - 32-bit CRC polynomial configuration"]
    #[inline(always)]
    #[must_use]
    pub fn crc33_17(&mut self) -> CRC33_17_W<13> {
        CRC33_17_W::new(self)
    }
    #[doc = "Bit 14 - CRC calculation initialization pattern control for receiver"]
    #[inline(always)]
    #[must_use]
    pub fn rcrcini(&mut self) -> RCRCINI_W<14> {
        RCRCINI_W::new(self)
    }
    #[doc = "Bit 15 - CRC calculation initialization pattern control for transmitter"]
    #[inline(always)]
    #[must_use]
    pub fn tcrcini(&mut self) -> TCRCINI_W<15> {
        TCRCINI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr1::R](R) reader structure"]
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
