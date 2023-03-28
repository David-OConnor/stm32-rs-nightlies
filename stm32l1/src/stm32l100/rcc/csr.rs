#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR` writer"]
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSION` reader - Internal low-speed oscillator enable"]
pub type LSION_R = crate::BitReader<bool>;
#[doc = "Field `LSION` writer - Internal low-speed oscillator enable"]
pub type LSION_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `LSIRDY` reader - Internal low-speed oscillator ready"]
pub type LSIRDY_R = crate::BitReader<bool>;
#[doc = "Field `LSEON` reader - External low-speed oscillator enable"]
pub type LSEON_R = crate::BitReader<bool>;
#[doc = "Field `LSEON` writer - External low-speed oscillator enable"]
pub type LSEON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `LSERDY` reader - External low-speed oscillator ready"]
pub type LSERDY_R = crate::BitReader<bool>;
#[doc = "Field `LSEBYP` reader - External low-speed oscillator bypass"]
pub type LSEBYP_R = crate::BitReader<bool>;
#[doc = "Field `LSEBYP` writer - External low-speed oscillator bypass"]
pub type LSEBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `LSECSSON` reader - CSS on LSE enable"]
pub type LSECSSON_R = crate::BitReader<bool>;
#[doc = "Field `LSECSSON` writer - CSS on LSE enable"]
pub type LSECSSON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `LSECSSD` reader - CSS on LSE failure Detection"]
pub type LSECSSD_R = crate::BitReader<bool>;
#[doc = "Field `LSECSSD` writer - CSS on LSE failure Detection"]
pub type LSECSSD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `RTCSEL` reader - RTC and LCD clock source selection"]
pub type RTCSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTCSEL` writer - RTC and LCD clock source selection"]
pub type RTCSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `RTCEN` reader - RTC clock enable"]
pub type RTCEN_R = crate::BitReader<bool>;
#[doc = "Field `RTCEN` writer - RTC clock enable"]
pub type RTCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `RTCRST` reader - RTC software reset"]
pub type RTCRST_R = crate::BitReader<RTCRSTW_A>;
#[doc = "RTC software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCRSTW_A {
    #[doc = "1: Resets the RTC peripheral"]
    Reset = 1,
}
impl From<RTCRSTW_A> for bool {
    #[inline(always)]
    fn from(variant: RTCRSTW_A) -> Self {
        variant as u8 != 0
    }
}
impl RTCRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RTCRSTW_A> {
        match self.bits {
            true => Some(RTCRSTW_A::Reset),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RTCRSTW_A::Reset
    }
}
#[doc = "Field `RTCRST` writer - RTC software reset"]
pub type RTCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, RTCRSTW_A, O>;
impl<'a, const O: u8> RTCRST_W<'a, O> {
    #[doc = "Resets the RTC peripheral"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RTCRSTW_A::Reset)
    }
}
#[doc = "Field `RMVF` reader - Remove reset flag"]
pub type RMVF_R = crate::BitReader<RMVFW_A>;
#[doc = "Remove reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMVFW_A {
    #[doc = "1: Clears the reset flag"]
    Clear = 1,
}
impl From<RMVFW_A> for bool {
    #[inline(always)]
    fn from(variant: RMVFW_A) -> Self {
        variant as u8 != 0
    }
}
impl RMVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RMVFW_A> {
        match self.bits {
            true => Some(RMVFW_A::Clear),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Clear`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RMVFW_A::Clear
    }
}
#[doc = "Field `RMVF` writer - Remove reset flag"]
pub type RMVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, RMVFW_A, O>;
impl<'a, const O: u8> RMVF_W<'a, O> {
    #[doc = "Clears the reset flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RMVFW_A::Clear)
    }
}
#[doc = "Field `OBLRSTF` reader - Options bytes loading reset flag"]
pub type OBLRSTF_R = crate::BitReader<OBLRSTFR_A>;
#[doc = "Options bytes loading reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBLRSTFR_A {
    #[doc = "0: No reset has occured"]
    NoReset = 0,
    #[doc = "1: A reset has occured"]
    Reset = 1,
}
impl From<OBLRSTFR_A> for bool {
    #[inline(always)]
    fn from(variant: OBLRSTFR_A) -> Self {
        variant as u8 != 0
    }
}
impl OBLRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OBLRSTFR_A {
        match self.bits {
            false => OBLRSTFR_A::NoReset,
            true => OBLRSTFR_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NoReset`"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == OBLRSTFR_A::NoReset
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == OBLRSTFR_A::Reset
    }
}
#[doc = "Field `OBLRSTF` writer - Options bytes loading reset flag"]
pub type OBLRSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, OBLRSTFR_A, O>;
impl<'a, const O: u8> OBLRSTF_W<'a, O> {
    #[doc = "No reset has occured"]
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(OBLRSTFR_A::NoReset)
    }
    #[doc = "A reset has occured"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(OBLRSTFR_A::Reset)
    }
}
#[doc = "Field `PINRSTF` reader - PIN reset flag"]
pub use OBLRSTF_R as PINRSTF_R;
#[doc = "Field `PORRSTF` reader - POR/PDR reset flag"]
pub use OBLRSTF_R as PORRSTF_R;
#[doc = "Field `SFTRSTF` reader - Software reset flag"]
pub use OBLRSTF_R as SFTRSTF_R;
#[doc = "Field `IWDGRSTF` reader - Independent watchdog reset flag"]
pub use OBLRSTF_R as IWDGRSTF_R;
#[doc = "Field `WWDGRSTF` reader - Window watchdog reset flag"]
pub use OBLRSTF_R as WWDGRSTF_R;
#[doc = "Field `LPWRSTF` reader - Low-power reset flag"]
pub use OBLRSTF_R as LPWRSTF_R;
#[doc = "Field `PINRSTF` writer - PIN reset flag"]
pub use OBLRSTF_W as PINRSTF_W;
#[doc = "Field `PORRSTF` writer - POR/PDR reset flag"]
pub use OBLRSTF_W as PORRSTF_W;
#[doc = "Field `SFTRSTF` writer - Software reset flag"]
pub use OBLRSTF_W as SFTRSTF_W;
#[doc = "Field `IWDGRSTF` writer - Independent watchdog reset flag"]
pub use OBLRSTF_W as IWDGRSTF_W;
#[doc = "Field `WWDGRSTF` writer - Window watchdog reset flag"]
pub use OBLRSTF_W as WWDGRSTF_W;
#[doc = "Field `LPWRSTF` writer - Low-power reset flag"]
pub use OBLRSTF_W as LPWRSTF_W;
impl R {
    #[doc = "Bit 0 - Internal low-speed oscillator enable"]
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal low-speed oscillator ready"]
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - External low-speed oscillator enable"]
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - External low-speed oscillator ready"]
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - External low-speed oscillator bypass"]
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CSS on LSE enable"]
    #[inline(always)]
    pub fn lsecsson(&self) -> LSECSSON_R {
        LSECSSON_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CSS on LSE failure Detection"]
    #[inline(always)]
    pub fn lsecssd(&self) -> LSECSSD_R {
        LSECSSD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:17 - RTC and LCD clock source selection"]
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSEL_R {
        RTCSEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 22 - RTC clock enable"]
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - RTC software reset"]
    #[inline(always)]
    pub fn rtcrst(&self) -> RTCRST_R {
        RTCRST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Remove reset flag"]
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Options bytes loading reset flag"]
    #[inline(always)]
    pub fn oblrstf(&self) -> OBLRSTF_R {
        OBLRSTF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PIN reset flag"]
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - POR/PDR reset flag"]
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Independent watchdog reset flag"]
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IWDGRSTF_R {
        IWDGRSTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Window watchdog reset flag"]
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline(always)]
    pub fn lpwrstf(&self) -> LPWRSTF_R {
        LPWRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal low-speed oscillator enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsion(&mut self) -> LSION_W<0> {
        LSION_W::new(self)
    }
    #[doc = "Bit 8 - External low-speed oscillator enable"]
    #[inline(always)]
    #[must_use]
    pub fn lseon(&mut self) -> LSEON_W<8> {
        LSEON_W::new(self)
    }
    #[doc = "Bit 10 - External low-speed oscillator bypass"]
    #[inline(always)]
    #[must_use]
    pub fn lsebyp(&mut self) -> LSEBYP_W<10> {
        LSEBYP_W::new(self)
    }
    #[doc = "Bit 11 - CSS on LSE enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsecsson(&mut self) -> LSECSSON_W<11> {
        LSECSSON_W::new(self)
    }
    #[doc = "Bit 12 - CSS on LSE failure Detection"]
    #[inline(always)]
    #[must_use]
    pub fn lsecssd(&mut self) -> LSECSSD_W<12> {
        LSECSSD_W::new(self)
    }
    #[doc = "Bits 16:17 - RTC and LCD clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn rtcsel(&mut self) -> RTCSEL_W<16> {
        RTCSEL_W::new(self)
    }
    #[doc = "Bit 22 - RTC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcen(&mut self) -> RTCEN_W<22> {
        RTCEN_W::new(self)
    }
    #[doc = "Bit 23 - RTC software reset"]
    #[inline(always)]
    #[must_use]
    pub fn rtcrst(&mut self) -> RTCRST_W<23> {
        RTCRST_W::new(self)
    }
    #[doc = "Bit 24 - Remove reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn rmvf(&mut self) -> RMVF_W<24> {
        RMVF_W::new(self)
    }
    #[doc = "Bit 25 - Options bytes loading reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn oblrstf(&mut self) -> OBLRSTF_W<25> {
        OBLRSTF_W::new(self)
    }
    #[doc = "Bit 26 - PIN reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn pinrstf(&mut self) -> PINRSTF_W<26> {
        PINRSTF_W::new(self)
    }
    #[doc = "Bit 27 - POR/PDR reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn porrstf(&mut self) -> PORRSTF_W<27> {
        PORRSTF_W::new(self)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn sftrstf(&mut self) -> SFTRSTF_W<28> {
        SFTRSTF_W::new(self)
    }
    #[doc = "Bit 29 - Independent watchdog reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn iwdgrstf(&mut self) -> IWDGRSTF_W<29> {
        IWDGRSTF_W::new(self)
    }
    #[doc = "Bit 30 - Window watchdog reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgrstf(&mut self) -> WWDGRSTF_W<30> {
        WWDGRSTF_W::new(self)
    }
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn lpwrstf(&mut self) -> LPWRSTF_W<31> {
        LPWRSTF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control/status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr::W](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
