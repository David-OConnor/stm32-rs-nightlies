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
    #[doc = "0: LSE oscillator OFF"]
    Disabled = 0,
    #[doc = "1: LSE oscillator ON"]
    Enabled = 1,
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
            false => LSEON_A::Disabled,
            true => LSEON_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSEON_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSEON_A::Enabled
    }
}
#[doc = "Field `LSEON` writer - LSE oscillator enable"]
pub type LSEON_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, LSEON_A, O>;
impl<'a, const O: u8> LSEON_W<'a, O> {
    #[doc = "LSE oscillator OFF"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSEON_A::Disabled)
    }
    #[doc = "LSE oscillator ON"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSEON_A::Enabled)
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
    NotBypassed = 0,
    #[doc = "1: LSE oscillator bypassed"]
    Bypassed = 1,
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
            false => LSEBYP_A::NotBypassed,
            true => LSEBYP_A::Bypassed,
        }
    }
    #[doc = "Checks if the value of the field is `NotBypassed`"]
    #[inline(always)]
    pub fn is_not_bypassed(&self) -> bool {
        *self == LSEBYP_A::NotBypassed
    }
    #[doc = "Checks if the value of the field is `Bypassed`"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == LSEBYP_A::Bypassed
    }
}
#[doc = "Field `LSEBYP` writer - LSE oscillator bypass"]
pub type LSEBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, LSEBYP_A, O>;
impl<'a, const O: u8> LSEBYP_W<'a, O> {
    #[doc = "LSE oscillator not bypassed"]
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut W {
        self.variant(LSEBYP_A::NotBypassed)
    }
    #[doc = "LSE oscillator bypassed"]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(LSEBYP_A::Bypassed)
    }
}
#[doc = "Field `LSEDRV` reader - SE oscillator drive capability"]
pub type LSEDRV_R = crate::FieldReader<u8, LSEDRV_A>;
#[doc = "SE oscillator drive capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LSEDRV_A {
    #[doc = "0: ‘Xtal mode’ lower driving capability"]
    Low = 0,
    #[doc = "1: ‘Xtal mode’ medium low driving capability"]
    MediumLow = 1,
    #[doc = "2: ‘Xtal mode’ medium high driving capability"]
    MediumHigh = 2,
    #[doc = "3: ‘Xtal mode’ higher driving capability"]
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
            1 => LSEDRV_A::MediumLow,
            2 => LSEDRV_A::MediumHigh,
            3 => LSEDRV_A::High,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Low`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == LSEDRV_A::Low
    }
    #[doc = "Checks if the value of the field is `MediumLow`"]
    #[inline(always)]
    pub fn is_medium_low(&self) -> bool {
        *self == LSEDRV_A::MediumLow
    }
    #[doc = "Checks if the value of the field is `MediumHigh`"]
    #[inline(always)]
    pub fn is_medium_high(&self) -> bool {
        *self == LSEDRV_A::MediumHigh
    }
    #[doc = "Checks if the value of the field is `High`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == LSEDRV_A::High
    }
}
#[doc = "Field `LSEDRV` writer - SE oscillator drive capability"]
pub type LSEDRV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, BDCR_SPEC, u8, LSEDRV_A, 2, O>;
impl<'a, const O: u8> LSEDRV_W<'a, O> {
    #[doc = "‘Xtal mode’ lower driving capability"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(LSEDRV_A::Low)
    }
    #[doc = "‘Xtal mode’ medium low driving capability"]
    #[inline(always)]
    pub fn medium_low(self) -> &'a mut W {
        self.variant(LSEDRV_A::MediumLow)
    }
    #[doc = "‘Xtal mode’ medium high driving capability"]
    #[inline(always)]
    pub fn medium_high(self) -> &'a mut W {
        self.variant(LSEDRV_A::MediumHigh)
    }
    #[doc = "‘Xtal mode’ higher driving capability"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(LSEDRV_A::High)
    }
}
#[doc = "Field `LSECSSON` reader - LSECSSON"]
pub type LSECSSON_R = crate::BitReader<LSECSSON_A>;
#[doc = "LSECSSON\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSON_A {
    #[doc = "0: CSS on LSE (32 kHz external oscillator) OFF"]
    Disabled = 0,
    #[doc = "1: CSS on LSE (32 kHz external oscillator) ON"]
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
#[doc = "Field `LSECSSON` writer - LSECSSON"]
pub type LSECSSON_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, LSECSSON_A, O>;
impl<'a, const O: u8> LSECSSON_W<'a, O> {
    #[doc = "CSS on LSE (32 kHz external oscillator) OFF"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSECSSON_A::Disabled)
    }
    #[doc = "CSS on LSE (32 kHz external oscillator) ON"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSECSSON_A::Enabled)
    }
}
#[doc = "Field `LSECSSD` reader - LSECSSD"]
pub type LSECSSD_R = crate::BitReader<LSECSSD_A>;
#[doc = "LSECSSD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSD_A {
    #[doc = "0: No failure detected on LSE (32 kHz oscillator)"]
    NoFailure = 0,
    #[doc = "1: Failure detected on LSE (32 kHz oscillator)"]
    FailureDetected = 1,
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
            true => LSECSSD_A::FailureDetected,
        }
    }
    #[doc = "Checks if the value of the field is `NoFailure`"]
    #[inline(always)]
    pub fn is_no_failure(&self) -> bool {
        *self == LSECSSD_A::NoFailure
    }
    #[doc = "Checks if the value of the field is `FailureDetected`"]
    #[inline(always)]
    pub fn is_failure_detected(&self) -> bool {
        *self == LSECSSD_A::FailureDetected
    }
}
#[doc = "Field `LSESYSDIS` reader - Disable the Clock LSE propagation to the system"]
pub type LSESYSDIS_R = crate::BitReader<LSESYSDIS_A>;
#[doc = "Disable the Clock LSE propagation to the system\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSESYSDIS_A {
    #[doc = "0: No clock LSE propagation"]
    Disabled = 0,
    #[doc = "1: Clock LSE propagation enabled"]
    Enabled = 1,
}
impl From<LSESYSDIS_A> for bool {
    #[inline(always)]
    fn from(variant: LSESYSDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl LSESYSDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSESYSDIS_A {
        match self.bits {
            false => LSESYSDIS_A::Disabled,
            true => LSESYSDIS_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSESYSDIS_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSESYSDIS_A::Enabled
    }
}
#[doc = "Field `LSESYSDIS` writer - Disable the Clock LSE propagation to the system"]
pub type LSESYSDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, LSESYSDIS_A, O>;
impl<'a, const O: u8> LSESYSDIS_W<'a, O> {
    #[doc = "No clock LSE propagation"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSESYSDIS_A::Disabled)
    }
    #[doc = "Clock LSE propagation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSESYSDIS_A::Enabled)
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
    #[doc = "3: HSE oscillator clock divided by 32 selected"]
    Hse = 3,
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
            3 => RTCSEL_A::Hse,
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
    #[doc = "Checks if the value of the field is `Hse`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == RTCSEL_A::Hse
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
    #[doc = "HSE oscillator clock divided by 32 selected"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(RTCSEL_A::Hse)
    }
}
#[doc = "Field `RTCEN` reader - RTC clock enable"]
pub type RTCEN_R = crate::BitReader<RTCEN_A>;
#[doc = "RTC clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCEN_A {
    #[doc = "0: RTC clock disabled"]
    Disabled = 0,
    #[doc = "1: RTC clock enabled"]
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
    #[doc = "RTC clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTCEN_A::Disabled)
    }
    #[doc = "RTC clock enabled"]
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
    NoReset = 0,
    #[doc = "1: Reset the entire Backup domain"]
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
            false => BDRST_A::NoReset,
            true => BDRST_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NoReset`"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == BDRST_A::NoReset
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
    pub fn no_reset(self) -> &'a mut W {
        self.variant(BDRST_A::NoReset)
    }
    #[doc = "Reset the entire Backup domain"]
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
    #[doc = "0: Low speed clock output (LSCO) disabled"]
    Disabled = 0,
    #[doc = "1: Low speed clock output (LSCO) enabled"]
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
    #[doc = "Low speed clock output (LSCO) disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSCOEN_A::Disabled)
    }
    #[doc = "Low speed clock output (LSCO) enabled"]
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
    #[doc = "Bits 3:4 - SE oscillator drive capability"]
    #[inline(always)]
    pub fn lsedrv(&self) -> LSEDRV_R {
        LSEDRV_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - LSECSSON"]
    #[inline(always)]
    pub fn lsecsson(&self) -> LSECSSON_R {
        LSECSSON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LSECSSD"]
    #[inline(always)]
    pub fn lsecssd(&self) -> LSECSSD_R {
        LSECSSD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Disable the Clock LSE propagation to the system"]
    #[inline(always)]
    pub fn lsesysdis(&self) -> LSESYSDIS_R {
        LSESYSDIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - RTC clock source selection"]
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSEL_R {
        RTCSEL_R::new(((self.bits >> 8) & 3) as u8)
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
    #[doc = "Bits 3:4 - SE oscillator drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn lsedrv(&mut self) -> LSEDRV_W<3> {
        LSEDRV_W::new(self)
    }
    #[doc = "Bit 5 - LSECSSON"]
    #[inline(always)]
    #[must_use]
    pub fn lsecsson(&mut self) -> LSECSSON_W<5> {
        LSECSSON_W::new(self)
    }
    #[doc = "Bit 7 - Disable the Clock LSE propagation to the system"]
    #[inline(always)]
    #[must_use]
    pub fn lsesysdis(&mut self) -> LSESYSDIS_W<7> {
        LSESYSDIS_W::new(self)
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
#[doc = "BDCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdcr](index.html) module"]
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
