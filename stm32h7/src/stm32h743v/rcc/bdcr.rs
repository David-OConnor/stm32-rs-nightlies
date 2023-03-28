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
#[doc = "Field `LSEON` reader - LSE oscillator enabled"]
pub type LSEON_R = crate::BitReader<LSEON_A>;
#[doc = "LSE oscillator enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSEON_A {
    #[doc = "0: LSE oscillator Off"]
    Off = 0,
    #[doc = "1: LSE oscillator On"]
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
#[doc = "Field `LSEON` writer - LSE oscillator enabled"]
pub type LSEON_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, LSEON_A, O>;
impl<'a, const O: u8> LSEON_W<'a, O> {
    #[doc = "LSE oscillator Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(LSEON_A::Off)
    }
    #[doc = "LSE oscillator On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(LSEON_A::On)
    }
}
#[doc = "Field `LSERDY` reader - LSE oscillator ready"]
pub type LSERDY_R = crate::BitReader<LSERDYR_A>;
#[doc = "LSE oscillator ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSERDYR_A {
    #[doc = "0: LSE oscillator not ready"]
    NotReady = 0,
    #[doc = "1: LSE oscillator ready"]
    Ready = 1,
}
impl From<LSERDYR_A> for bool {
    #[inline(always)]
    fn from(variant: LSERDYR_A) -> Self {
        variant as u8 != 0
    }
}
impl LSERDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSERDYR_A {
        match self.bits {
            false => LSERDYR_A::NotReady,
            true => LSERDYR_A::Ready,
        }
    }
    #[doc = "Checks if the value of the field is `NotReady`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LSERDYR_A::NotReady
    }
    #[doc = "Checks if the value of the field is `Ready`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LSERDYR_A::Ready
    }
}
#[doc = "Field `LSERDY` writer - LSE oscillator ready"]
pub type LSERDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, LSERDYR_A, O>;
impl<'a, const O: u8> LSERDY_W<'a, O> {
    #[doc = "LSE oscillator not ready"]
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut W {
        self.variant(LSERDYR_A::NotReady)
    }
    #[doc = "LSE oscillator ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(LSERDYR_A::Ready)
    }
}
#[doc = "Field `LSEBYP` reader - LSE oscillator bypass"]
pub type LSEBYP_R = crate::BitReader<LSEBYP_A>;
#[doc = "LSE oscillator bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSEBYP_A {
    #[doc = "0: LSE crystal oscillator not bypassed"]
    NotBypassed = 0,
    #[doc = "1: LSE crystal oscillator bypassed with external clock"]
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
    #[doc = "LSE crystal oscillator not bypassed"]
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut W {
        self.variant(LSEBYP_A::NotBypassed)
    }
    #[doc = "LSE crystal oscillator bypassed with external clock"]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(LSEBYP_A::Bypassed)
    }
}
#[doc = "Field `LSEDRV` reader - LSE oscillator driving capability"]
pub type LSEDRV_R = crate::FieldReader<u8, LSEDRV_A>;
#[doc = "LSE oscillator driving capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LSEDRV_A {
    #[doc = "0: Lowest LSE oscillator driving capability"]
    Lowest = 0,
    #[doc = "1: Medium low LSE oscillator driving capability"]
    MediumLow = 1,
    #[doc = "2: Medium high LSE oscillator driving capability"]
    MediumHigh = 2,
    #[doc = "3: Highest LSE oscillator driving capability"]
    Highest = 3,
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
            0 => LSEDRV_A::Lowest,
            1 => LSEDRV_A::MediumLow,
            2 => LSEDRV_A::MediumHigh,
            3 => LSEDRV_A::Highest,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Lowest`"]
    #[inline(always)]
    pub fn is_lowest(&self) -> bool {
        *self == LSEDRV_A::Lowest
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
    #[doc = "Checks if the value of the field is `Highest`"]
    #[inline(always)]
    pub fn is_highest(&self) -> bool {
        *self == LSEDRV_A::Highest
    }
}
#[doc = "Field `LSEDRV` writer - LSE oscillator driving capability"]
pub type LSEDRV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, BDCR_SPEC, u8, LSEDRV_A, 2, O>;
impl<'a, const O: u8> LSEDRV_W<'a, O> {
    #[doc = "Lowest LSE oscillator driving capability"]
    #[inline(always)]
    pub fn lowest(self) -> &'a mut W {
        self.variant(LSEDRV_A::Lowest)
    }
    #[doc = "Medium low LSE oscillator driving capability"]
    #[inline(always)]
    pub fn medium_low(self) -> &'a mut W {
        self.variant(LSEDRV_A::MediumLow)
    }
    #[doc = "Medium high LSE oscillator driving capability"]
    #[inline(always)]
    pub fn medium_high(self) -> &'a mut W {
        self.variant(LSEDRV_A::MediumHigh)
    }
    #[doc = "Highest LSE oscillator driving capability"]
    #[inline(always)]
    pub fn highest(self) -> &'a mut W {
        self.variant(LSEDRV_A::Highest)
    }
}
#[doc = "Field `LSECSSON` reader - LSE clock security system enable"]
pub type LSECSSON_R = crate::BitReader<LSECSSON_A>;
#[doc = "LSE clock security system enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSON_A {
    #[doc = "0: Clock security system on 32 kHz oscillator off"]
    SecurityOff = 0,
    #[doc = "1: Clock security system on 32 kHz oscillator on"]
    SecurityOn = 1,
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
            false => LSECSSON_A::SecurityOff,
            true => LSECSSON_A::SecurityOn,
        }
    }
    #[doc = "Checks if the value of the field is `SecurityOff`"]
    #[inline(always)]
    pub fn is_security_off(&self) -> bool {
        *self == LSECSSON_A::SecurityOff
    }
    #[doc = "Checks if the value of the field is `SecurityOn`"]
    #[inline(always)]
    pub fn is_security_on(&self) -> bool {
        *self == LSECSSON_A::SecurityOn
    }
}
#[doc = "Field `LSECSSON` writer - LSE clock security system enable"]
pub type LSECSSON_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, LSECSSON_A, O>;
impl<'a, const O: u8> LSECSSON_W<'a, O> {
    #[doc = "Clock security system on 32 kHz oscillator off"]
    #[inline(always)]
    pub fn security_off(self) -> &'a mut W {
        self.variant(LSECSSON_A::SecurityOff)
    }
    #[doc = "Clock security system on 32 kHz oscillator on"]
    #[inline(always)]
    pub fn security_on(self) -> &'a mut W {
        self.variant(LSECSSON_A::SecurityOn)
    }
}
#[doc = "Field `LSECSSD` reader - LSE clock security system failure detection"]
pub type LSECSSD_R = crate::BitReader<LSECSSDR_A>;
#[doc = "LSE clock security system failure detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSDR_A {
    #[doc = "0: No failure detected on 32 kHz oscillator"]
    NoFailure = 0,
    #[doc = "1: Failure detected on 32 kHz oscillator"]
    Failure = 1,
}
impl From<LSECSSDR_A> for bool {
    #[inline(always)]
    fn from(variant: LSECSSDR_A) -> Self {
        variant as u8 != 0
    }
}
impl LSECSSD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSECSSDR_A {
        match self.bits {
            false => LSECSSDR_A::NoFailure,
            true => LSECSSDR_A::Failure,
        }
    }
    #[doc = "Checks if the value of the field is `NoFailure`"]
    #[inline(always)]
    pub fn is_no_failure(&self) -> bool {
        *self == LSECSSDR_A::NoFailure
    }
    #[doc = "Checks if the value of the field is `Failure`"]
    #[inline(always)]
    pub fn is_failure(&self) -> bool {
        *self == LSECSSDR_A::Failure
    }
}
#[doc = "Field `LSECSSD` writer - LSE clock security system failure detection"]
pub type LSECSSD_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, LSECSSDR_A, O>;
impl<'a, const O: u8> LSECSSD_W<'a, O> {
    #[doc = "No failure detected on 32 kHz oscillator"]
    #[inline(always)]
    pub fn no_failure(self) -> &'a mut W {
        self.variant(LSECSSDR_A::NoFailure)
    }
    #[doc = "Failure detected on 32 kHz oscillator"]
    #[inline(always)]
    pub fn failure(self) -> &'a mut W {
        self.variant(LSECSSDR_A::Failure)
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
    #[doc = "1: LSE oscillator clock used as RTC clock"]
    Lse = 1,
    #[doc = "2: LSI oscillator clock used as RTC clock"]
    Lsi = 2,
    #[doc = "3: HSE oscillator clock divided by a prescaler used as RTC clock"]
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
    #[doc = "LSE oscillator clock used as RTC clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(RTCSEL_A::Lse)
    }
    #[doc = "LSI oscillator clock used as RTC clock"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(RTCSEL_A::Lsi)
    }
    #[doc = "HSE oscillator clock divided by a prescaler used as RTC clock"]
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
#[doc = "Field `BDRST` reader - VSwitch domain software reset"]
pub type BDRST_R = crate::BitReader<BDRST_A>;
#[doc = "VSwitch domain software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BDRST_A {
    #[doc = "1: Resets the entire VSW domain"]
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
    pub fn variant(&self) -> Option<BDRST_A> {
        match self.bits {
            true => Some(BDRST_A::Reset),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == BDRST_A::Reset
    }
}
#[doc = "Field `BDRST` writer - VSwitch domain software reset"]
pub type BDRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, BDRST_A, O>;
impl<'a, const O: u8> BDRST_W<'a, O> {
    #[doc = "Resets the entire VSW domain"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(BDRST_A::Reset)
    }
}
impl R {
    #[doc = "Bit 0 - LSE oscillator enabled"]
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
    #[doc = "Bits 3:4 - LSE oscillator driving capability"]
    #[inline(always)]
    pub fn lsedrv(&self) -> LSEDRV_R {
        LSEDRV_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - LSE clock security system enable"]
    #[inline(always)]
    pub fn lsecsson(&self) -> LSECSSON_R {
        LSECSSON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LSE clock security system failure detection"]
    #[inline(always)]
    pub fn lsecssd(&self) -> LSECSSD_R {
        LSECSSD_R::new(((self.bits >> 6) & 1) != 0)
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
    #[doc = "Bit 16 - VSwitch domain software reset"]
    #[inline(always)]
    pub fn bdrst(&self) -> BDRST_R {
        BDRST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSE oscillator enabled"]
    #[inline(always)]
    #[must_use]
    pub fn lseon(&mut self) -> LSEON_W<0> {
        LSEON_W::new(self)
    }
    #[doc = "Bit 1 - LSE oscillator ready"]
    #[inline(always)]
    #[must_use]
    pub fn lserdy(&mut self) -> LSERDY_W<1> {
        LSERDY_W::new(self)
    }
    #[doc = "Bit 2 - LSE oscillator bypass"]
    #[inline(always)]
    #[must_use]
    pub fn lsebyp(&mut self) -> LSEBYP_W<2> {
        LSEBYP_W::new(self)
    }
    #[doc = "Bits 3:4 - LSE oscillator driving capability"]
    #[inline(always)]
    #[must_use]
    pub fn lsedrv(&mut self) -> LSEDRV_W<3> {
        LSEDRV_W::new(self)
    }
    #[doc = "Bit 5 - LSE clock security system enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsecsson(&mut self) -> LSECSSON_W<5> {
        LSECSSON_W::new(self)
    }
    #[doc = "Bit 6 - LSE clock security system failure detection"]
    #[inline(always)]
    #[must_use]
    pub fn lsecssd(&mut self) -> LSECSSD_W<6> {
        LSECSSD_W::new(self)
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
    #[doc = "Bit 16 - VSwitch domain software reset"]
    #[inline(always)]
    #[must_use]
    pub fn bdrst(&mut self) -> BDRST_W<16> {
        BDRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC Backup Domain Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdcr](index.html) module"]
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
