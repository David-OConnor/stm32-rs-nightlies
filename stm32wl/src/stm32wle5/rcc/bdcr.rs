#[doc = "Register `BDCR` reader"]
pub struct R(crate::R<BDCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BDCR` writer"]
pub struct W(crate::W<BDCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDCR_SPEC>;
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
impl From<crate::W<BDCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSEON` reader - LSE oscillator enable"]
pub type LSEON_R = crate::BitReader<LSEON_A>;
#[doc = "LSE oscillator enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSEON_A {
    #[doc = "0: LSE oscillator off"]
    Off = 0,
    #[doc = "1: LSE oscillator on"]
    On = 1,
}
impl From<LSEON_A> for bool {
    #[inline(always)]
    fn from(variant: LSEON_A) -> Self {
        variant as u8 != 0
    }
}
impl LSEON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSEON_A {
        match self.bits {
            false => LSEON_A::Off,
            true => LSEON_A::On,
        }
    }
    #[doc = "Checks if the value of the field is `Off`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == LSEON_A::Off
    }
    #[doc = "Checks if the value of the field is `On`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == LSEON_A::On
    }
}
#[doc = "Field `LSEON` writer - LSE oscillator enable"]
pub type LSEON_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, LSEON_A, O>;
impl<'a, const O: u8> LSEON_W<'a, O> {
    #[doc = "LSE oscillator off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(LSEON_A::Off)
    }
    #[doc = "LSE oscillator on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(LSEON_A::On)
    }
}
#[doc = "Field `LSERDY` reader - LSE oscillator ready"]
pub type LSERDY_R = crate::BitReader<LSERDY_A>;
#[doc = "LSE oscillator ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSERDY_A {
    #[doc = "0: LSE oscillator not ready"]
    NotReady = 0,
    #[doc = "1: LSE oscillator ready"]
    Ready = 1,
}
impl From<LSERDY_A> for bool {
    #[inline(always)]
    fn from(variant: LSERDY_A) -> Self {
        variant as u8 != 0
    }
}
impl LSERDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSERDY_A {
        match self.bits {
            false => LSERDY_A::NotReady,
            true => LSERDY_A::Ready,
        }
    }
    #[doc = "Checks if the value of the field is `NotReady`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LSERDY_A::NotReady
    }
    #[doc = "Checks if the value of the field is `Ready`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LSERDY_A::Ready
    }
}
#[doc = "Field `LSEBYP` reader - LSE oscillator bypass"]
pub type LSEBYP_R = crate::BitReader<LSEBYP_A>;
#[doc = "LSE oscillator bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSEBYP_A {
    #[doc = "0: LSE oscillator not bypassed"]
    Disabled = 0,
    #[doc = "1: LSE oscillator bypassed"]
    Enabled = 1,
}
impl From<LSEBYP_A> for bool {
    #[inline(always)]
    fn from(variant: LSEBYP_A) -> Self {
        variant as u8 != 0
    }
}
impl LSEBYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSEBYP_A {
        match self.bits {
            false => LSEBYP_A::Disabled,
            true => LSEBYP_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSEBYP_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSEBYP_A::Enabled
    }
}
#[doc = "Field `LSEBYP` writer - LSE oscillator bypass"]
pub type LSEBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, LSEBYP_A, O>;
impl<'a, const O: u8> LSEBYP_W<'a, O> {
    #[doc = "LSE oscillator not bypassed"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSEBYP_A::Disabled)
    }
    #[doc = "LSE oscillator bypassed"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSEBYP_A::Enabled)
    }
}
#[doc = "Field `LSEDRV` reader - LSE oscillator drive capability"]
pub type LSEDRV_R = crate::FieldReader<u8, LSEDRV_A>;
#[doc = "LSE oscillator drive capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LSEDRV_A {
    #[doc = "0: Xtal mode lower driving capability"]
    Low = 0,
    #[doc = "1: Xtal mode medium-low driving capability"]
    MedLow = 1,
    #[doc = "2: Xtal mode medium-high driving capability"]
    MedHigh = 2,
    #[doc = "3: Xtal mode higher driving capability"]
    High = 3,
}
impl From<LSEDRV_A> for u8 {
    #[inline(always)]
    fn from(variant: LSEDRV_A) -> Self {
        variant as _
    }
}
impl LSEDRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSEDRV_A {
        match self.bits {
            0 => LSEDRV_A::Low,
            1 => LSEDRV_A::MedLow,
            2 => LSEDRV_A::MedHigh,
            3 => LSEDRV_A::High,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Low`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == LSEDRV_A::Low
    }
    #[doc = "Checks if the value of the field is `MedLow`"]
    #[inline(always)]
    pub fn is_med_low(&self) -> bool {
        *self == LSEDRV_A::MedLow
    }
    #[doc = "Checks if the value of the field is `MedHigh`"]
    #[inline(always)]
    pub fn is_med_high(&self) -> bool {
        *self == LSEDRV_A::MedHigh
    }
    #[doc = "Checks if the value of the field is `High`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == LSEDRV_A::High
    }
}
#[doc = "Field `LSEDRV` writer - LSE oscillator drive capability"]
pub type LSEDRV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, BDCR_SPEC, u8, LSEDRV_A, 2, O>;
impl<'a, const O: u8> LSEDRV_W<'a, O> {
    #[doc = "Xtal mode lower driving capability"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(LSEDRV_A::Low)
    }
    #[doc = "Xtal mode medium-low driving capability"]
    #[inline(always)]
    pub fn med_low(self) -> &'a mut W {
        self.variant(LSEDRV_A::MedLow)
    }
    #[doc = "Xtal mode medium-high driving capability"]
    #[inline(always)]
    pub fn med_high(self) -> &'a mut W {
        self.variant(LSEDRV_A::MedHigh)
    }
    #[doc = "Xtal mode higher driving capability"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(LSEDRV_A::High)
    }
}
#[doc = "Field `LSECSSON` reader - CSS on LSE enable"]
pub type LSECSSON_R = crate::BitReader<LSECSSON_A>;
#[doc = "CSS on LSE enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSON_A {
    #[doc = "0: CSS on LSE disabled"]
    Disabled = 0,
    #[doc = "1: CSS on LSE enabled"]
    Enabled = 1,
}
impl From<LSECSSON_A> for bool {
    #[inline(always)]
    fn from(variant: LSECSSON_A) -> Self {
        variant as u8 != 0
    }
}
impl LSECSSON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSECSSON_A {
        match self.bits {
            false => LSECSSON_A::Disabled,
            true => LSECSSON_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSECSSON_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSECSSON_A::Enabled
    }
}
#[doc = "Field `LSECSSON` writer - CSS on LSE enable"]
pub type LSECSSON_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, LSECSSON_A, O>;
impl<'a, const O: u8> LSECSSON_W<'a, O> {
    #[doc = "CSS on LSE disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSECSSON_A::Disabled)
    }
    #[doc = "CSS on LSE enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSECSSON_A::Enabled)
    }
}
#[doc = "Field `LSECSSD` reader - CSS on LSE failure Detection"]
pub type LSECSSD_R = crate::BitReader<LSECSSD_A>;
#[doc = "CSS on LSE failure Detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSD_A {
    #[doc = "0: No failure detected on LSE"]
    NoFailure = 0,
    #[doc = "1: Failure detected on LSE"]
    Failure = 1,
}
impl From<LSECSSD_A> for bool {
    #[inline(always)]
    fn from(variant: LSECSSD_A) -> Self {
        variant as u8 != 0
    }
}
impl LSECSSD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSECSSD_A {
        match self.bits {
            false => LSECSSD_A::NoFailure,
            true => LSECSSD_A::Failure,
        }
    }
    #[doc = "Checks if the value of the field is `NoFailure`"]
    #[inline(always)]
    pub fn is_no_failure(&self) -> bool {
        *self == LSECSSD_A::NoFailure
    }
    #[doc = "Checks if the value of the field is `Failure`"]
    #[inline(always)]
    pub fn is_failure(&self) -> bool {
        *self == LSECSSD_A::Failure
    }
}
#[doc = "Field `LSESYSEN` reader - LSE system clock enable"]
pub type LSESYSEN_R = crate::BitReader<LSESYSEN_A>;
#[doc = "LSE system clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSESYSEN_A {
    #[doc = "0: LSE system clock disabled to USARTx, LPUARTx, LPTIMx, TIMx, RNG, system LSCO, MCO, MSI PLL mode"]
    Disabled = 0,
    #[doc = "1: LSE system clock enabled to USARTx, LPUARTx, LPTIMx, TIMx, RNG, system LSCO, MCO, MSI PLL mode"]
    Enabled = 1,
}
impl From<LSESYSEN_A> for bool {
    #[inline(always)]
    fn from(variant: LSESYSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LSESYSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSESYSEN_A {
        match self.bits {
            false => LSESYSEN_A::Disabled,
            true => LSESYSEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSESYSEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSESYSEN_A::Enabled
    }
}
#[doc = "Field `LSESYSEN` writer - LSE system clock enable"]
pub type LSESYSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, LSESYSEN_A, O>;
impl<'a, const O: u8> LSESYSEN_W<'a, O> {
    #[doc = "LSE system clock disabled to USARTx, LPUARTx, LPTIMx, TIMx, RNG, system LSCO, MCO, MSI PLL mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSESYSEN_A::Disabled)
    }
    #[doc = "LSE system clock enabled to USARTx, LPUARTx, LPTIMx, TIMx, RNG, system LSCO, MCO, MSI PLL mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSESYSEN_A::Enabled)
    }
}
#[doc = "Field `RTCSEL` reader - RTC clock source selection"]
pub type RTCSEL_R = crate::FieldReader<u8, RTCSEL_A>;
#[doc = "RTC clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTCSEL_A {
    #[doc = "0: No clock"]
    NoClock = 0,
    #[doc = "1: LSE oscillator clock selected"]
    Lse = 1,
    #[doc = "2: LSI oscillator clock selected"]
    Lsi = 2,
    #[doc = "3: HSE32 oscillator clock divided by 32 selected"]
    Hse32 = 3,
}
impl From<RTCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCSEL_A) -> Self {
        variant as _
    }
}
impl RTCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCSEL_A {
        match self.bits {
            0 => RTCSEL_A::NoClock,
            1 => RTCSEL_A::Lse,
            2 => RTCSEL_A::Lsi,
            3 => RTCSEL_A::Hse32,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NoClock`"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == RTCSEL_A::NoClock
    }
    #[doc = "Checks if the value of the field is `Lse`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == RTCSEL_A::Lse
    }
    #[doc = "Checks if the value of the field is `Lsi`"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == RTCSEL_A::Lsi
    }
    #[doc = "Checks if the value of the field is `Hse32`"]
    #[inline(always)]
    pub fn is_hse32(&self) -> bool {
        *self == RTCSEL_A::Hse32
    }
}
#[doc = "Field `RTCSEL` writer - RTC clock source selection"]
pub type RTCSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, BDCR_SPEC, u8, RTCSEL_A, 2, O>;
impl<'a, const O: u8> RTCSEL_W<'a, O> {
    #[doc = "No clock"]
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut W {
        self.variant(RTCSEL_A::NoClock)
    }
    #[doc = "LSE oscillator clock selected"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(RTCSEL_A::Lse)
    }
    #[doc = "LSI oscillator clock selected"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(RTCSEL_A::Lsi)
    }
    #[doc = "HSE32 oscillator clock divided by 32 selected"]
    #[inline(always)]
    pub fn hse32(self) -> &'a mut W {
        self.variant(RTCSEL_A::Hse32)
    }
}
#[doc = "Field `LSESYSRDY` reader - LSE system clock ready"]
pub type LSESYSRDY_R = crate::BitReader<LSESYSRDY_A>;
#[doc = "LSE system clock ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSESYSRDY_A {
    #[doc = "0: LSE system clock not ready"]
    NotReady = 0,
    #[doc = "1: LSE system clock ready"]
    Ready = 1,
}
impl From<LSESYSRDY_A> for bool {
    #[inline(always)]
    fn from(variant: LSESYSRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl LSESYSRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSESYSRDY_A {
        match self.bits {
            false => LSESYSRDY_A::NotReady,
            true => LSESYSRDY_A::Ready,
        }
    }
    #[doc = "Checks if the value of the field is `NotReady`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LSESYSRDY_A::NotReady
    }
    #[doc = "Checks if the value of the field is `Ready`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LSESYSRDY_A::Ready
    }
}
#[doc = "Field `RTCEN` reader - RTC clock enable"]
pub type RTCEN_R = crate::BitReader<RTCEN_A>;
#[doc = "RTC clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCEN_A {
    #[doc = "0: RTC kernel clock disabled"]
    Disabled = 0,
    #[doc = "1: RTC kernel clock enabled"]
    Enabled = 1,
}
impl From<RTCEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RTCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCEN_A {
        match self.bits {
            false => RTCEN_A::Disabled,
            true => RTCEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTCEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTCEN_A::Enabled
    }
}
#[doc = "Field `RTCEN` writer - RTC clock enable"]
pub type RTCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, RTCEN_A, O>;
impl<'a, const O: u8> RTCEN_W<'a, O> {
    #[doc = "RTC kernel clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTCEN_A::Disabled)
    }
    #[doc = "RTC kernel clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RTCEN_A::Enabled)
    }
}
#[doc = "Field `BDRST` reader - Backup domain software reset"]
pub type BDRST_R = crate::BitReader<BDRST_A>;
#[doc = "Backup domain software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BDRST_A {
    #[doc = "0: Reset not activated"]
    NotActive = 0,
    #[doc = "1: Entire Backup domain reset"]
    Reset = 1,
}
impl From<BDRST_A> for bool {
    #[inline(always)]
    fn from(variant: BDRST_A) -> Self {
        variant as u8 != 0
    }
}
impl BDRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BDRST_A {
        match self.bits {
            false => BDRST_A::NotActive,
            true => BDRST_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NotActive`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == BDRST_A::NotActive
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == BDRST_A::Reset
    }
}
#[doc = "Field `BDRST` writer - Backup domain software reset"]
pub type BDRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, BDRST_A, O>;
impl<'a, const O: u8> BDRST_W<'a, O> {
    #[doc = "Reset not activated"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(BDRST_A::NotActive)
    }
    #[doc = "Entire Backup domain reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(BDRST_A::Reset)
    }
}
#[doc = "Field `LSCOEN` reader - Low speed clock output enable"]
pub type LSCOEN_R = crate::BitReader<LSCOEN_A>;
#[doc = "Low speed clock output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSCOEN_A {
    #[doc = "0: LSCO disabled"]
    Disabled = 0,
    #[doc = "1: LSCO enabled"]
    Enabled = 1,
}
impl From<LSCOEN_A> for bool {
    #[inline(always)]
    fn from(variant: LSCOEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LSCOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSCOEN_A {
        match self.bits {
            false => LSCOEN_A::Disabled,
            true => LSCOEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSCOEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSCOEN_A::Enabled
    }
}
#[doc = "Field `LSCOEN` writer - Low speed clock output enable"]
pub type LSCOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, LSCOEN_A, O>;
impl<'a, const O: u8> LSCOEN_W<'a, O> {
    #[doc = "LSCO disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSCOEN_A::Disabled)
    }
    #[doc = "LSCO enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSCOEN_A::Enabled)
    }
}
#[doc = "Field `LSCOSEL` reader - Low speed clock output selection"]
pub type LSCOSEL_R = crate::BitReader<LSCOSEL_A>;
#[doc = "Low speed clock output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSCOSEL_A {
    #[doc = "0: LSI clock selected"]
    Lsi = 0,
    #[doc = "1: LSE clock selected"]
    Lse = 1,
}
impl From<LSCOSEL_A> for bool {
    #[inline(always)]
    fn from(variant: LSCOSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl LSCOSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSCOSEL_A {
        match self.bits {
            false => LSCOSEL_A::Lsi,
            true => LSCOSEL_A::Lse,
        }
    }
    #[doc = "Checks if the value of the field is `Lsi`"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == LSCOSEL_A::Lsi
    }
    #[doc = "Checks if the value of the field is `Lse`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LSCOSEL_A::Lse
    }
}
#[doc = "Field `LSCOSEL` writer - Low speed clock output selection"]
pub type LSCOSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, LSCOSEL_A, O>;
impl<'a, const O: u8> LSCOSEL_W<'a, O> {
    #[doc = "LSI clock selected"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(LSCOSEL_A::Lsi)
    }
    #[doc = "LSE clock selected"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(LSCOSEL_A::Lse)
    }
}
impl R {
    #[doc = "Bit 0 - LSE oscillator enable"]
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE oscillator ready"]
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LSE oscillator bypass"]
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - LSE oscillator drive capability"]
    #[inline(always)]
    pub fn lsedrv(&self) -> LSEDRV_R {
        LSEDRV_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - CSS on LSE enable"]
    #[inline(always)]
    pub fn lsecsson(&self) -> LSECSSON_R {
        LSECSSON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CSS on LSE failure Detection"]
    #[inline(always)]
    pub fn lsecssd(&self) -> LSECSSD_R {
        LSECSSD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LSE system clock enable"]
    #[inline(always)]
    pub fn lsesysen(&self) -> LSESYSEN_R {
        LSESYSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - RTC clock source selection"]
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSEL_R {
        RTCSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - LSE system clock ready"]
    #[inline(always)]
    pub fn lsesysrdy(&self) -> LSESYSRDY_R {
        LSESYSRDY_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Backup domain software reset"]
    #[inline(always)]
    pub fn bdrst(&self) -> BDRST_R {
        BDRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Low speed clock output enable"]
    #[inline(always)]
    pub fn lscoen(&self) -> LSCOEN_R {
        LSCOEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Low speed clock output selection"]
    #[inline(always)]
    pub fn lscosel(&self) -> LSCOSEL_R {
        LSCOSEL_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSE oscillator enable"]
    #[inline(always)]
    #[must_use]
    pub fn lseon(&mut self) -> LSEON_W<0> {
        LSEON_W::new(self)
    }
    #[doc = "Bit 2 - LSE oscillator bypass"]
    #[inline(always)]
    #[must_use]
    pub fn lsebyp(&mut self) -> LSEBYP_W<2> {
        LSEBYP_W::new(self)
    }
    #[doc = "Bits 3:4 - LSE oscillator drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn lsedrv(&mut self) -> LSEDRV_W<3> {
        LSEDRV_W::new(self)
    }
    #[doc = "Bit 5 - CSS on LSE enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsecsson(&mut self) -> LSECSSON_W<5> {
        LSECSSON_W::new(self)
    }
    #[doc = "Bit 7 - LSE system clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsesysen(&mut self) -> LSESYSEN_W<7> {
        LSESYSEN_W::new(self)
    }
    #[doc = "Bits 8:9 - RTC clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn rtcsel(&mut self) -> RTCSEL_W<8> {
        RTCSEL_W::new(self)
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcen(&mut self) -> RTCEN_W<15> {
        RTCEN_W::new(self)
    }
    #[doc = "Bit 16 - Backup domain software reset"]
    #[inline(always)]
    #[must_use]
    pub fn bdrst(&mut self) -> BDRST_W<16> {
        BDRST_W::new(self)
    }
    #[doc = "Bit 24 - Low speed clock output enable"]
    #[inline(always)]
    #[must_use]
    pub fn lscoen(&mut self) -> LSCOEN_W<24> {
        LSCOEN_W::new(self)
    }
    #[doc = "Bit 25 - Low speed clock output selection"]
    #[inline(always)]
    #[must_use]
    pub fn lscosel(&mut self) -> LSCOSEL_W<25> {
        LSCOSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup domain control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdcr](index.html) module"]
pub struct BDCR_SPEC;
impl crate::RegisterSpec for BDCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bdcr::R](R) reader structure"]
impl crate::Readable for BDCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bdcr::W](W) writer structure"]
impl crate::Writable for BDCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BDCR to value 0"]
impl crate::Resettable for BDCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
