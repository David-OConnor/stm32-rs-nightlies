#[doc = "Register `SECCFGR` reader"]
pub struct R(crate::R<SECCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECCFGR` writer"]
pub struct W(crate::W<SECCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECCFGR_SPEC>;
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
impl From<crate::W<SECCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALRASEC` reader - Alarm A and SSR underflow protection"]
pub type ALRASEC_R = crate::BitReader<bool>;
#[doc = "Field `ALRASEC` writer - Alarm A and SSR underflow protection"]
pub type ALRASEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `ALRBSEC` reader - Alarm B protection"]
pub type ALRBSEC_R = crate::BitReader<bool>;
#[doc = "Field `ALRBSEC` writer - Alarm B protection"]
pub type ALRBSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `WUTSEC` reader - Wakeup timer protection"]
pub type WUTSEC_R = crate::BitReader<bool>;
#[doc = "Field `WUTSEC` writer - Wakeup timer protection"]
pub type WUTSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `TSSEC` reader - Timestamp protection"]
pub type TSSEC_R = crate::BitReader<bool>;
#[doc = "Field `TSSEC` writer - Timestamp protection"]
pub type TSSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `CALSEC` reader - Shift register, daylight saving, calibration and reference clock protection"]
pub type CALSEC_R = crate::BitReader<bool>;
#[doc = "Field `CALSEC` writer - Shift register, daylight saving, calibration and reference clock protection"]
pub type CALSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `INITSEC` reader - Initialization protection"]
pub type INITSEC_R = crate::BitReader<bool>;
#[doc = "Field `INITSEC` writer - Initialization protection"]
pub type INITSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC` reader - RTC global protection"]
pub type SEC_R = crate::BitReader<bool>;
#[doc = "Field `SEC` writer - RTC global protection"]
pub type SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Alarm A and SSR underflow protection"]
    #[inline(always)]
    pub fn alrasec(&self) -> ALRASEC_R {
        ALRASEC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm B protection"]
    #[inline(always)]
    pub fn alrbsec(&self) -> ALRBSEC_R {
        ALRBSEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup timer protection"]
    #[inline(always)]
    pub fn wutsec(&self) -> WUTSEC_R {
        WUTSEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timestamp protection"]
    #[inline(always)]
    pub fn tssec(&self) -> TSSEC_R {
        TSSEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 13 - Shift register, daylight saving, calibration and reference clock protection"]
    #[inline(always)]
    pub fn calsec(&self) -> CALSEC_R {
        CALSEC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Initialization protection"]
    #[inline(always)]
    pub fn initsec(&self) -> INITSEC_R {
        INITSEC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RTC global protection"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alarm A and SSR underflow protection"]
    #[inline(always)]
    #[must_use]
    pub fn alrasec(&mut self) -> ALRASEC_W<0> {
        ALRASEC_W::new(self)
    }
    #[doc = "Bit 1 - Alarm B protection"]
    #[inline(always)]
    #[must_use]
    pub fn alrbsec(&mut self) -> ALRBSEC_W<1> {
        ALRBSEC_W::new(self)
    }
    #[doc = "Bit 2 - Wakeup timer protection"]
    #[inline(always)]
    #[must_use]
    pub fn wutsec(&mut self) -> WUTSEC_W<2> {
        WUTSEC_W::new(self)
    }
    #[doc = "Bit 3 - Timestamp protection"]
    #[inline(always)]
    #[must_use]
    pub fn tssec(&mut self) -> TSSEC_W<3> {
        TSSEC_W::new(self)
    }
    #[doc = "Bit 13 - Shift register, daylight saving, calibration and reference clock protection"]
    #[inline(always)]
    #[must_use]
    pub fn calsec(&mut self) -> CALSEC_W<13> {
        CALSEC_W::new(self)
    }
    #[doc = "Bit 14 - Initialization protection"]
    #[inline(always)]
    #[must_use]
    pub fn initsec(&mut self) -> INITSEC_W<14> {
        INITSEC_W::new(self)
    }
    #[doc = "Bit 15 - RTC global protection"]
    #[inline(always)]
    #[must_use]
    pub fn sec(&mut self) -> SEC_W<15> {
        SEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC secure configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seccfgr](index.html) module"]
pub struct SECCFGR_SPEC;
impl crate::RegisterSpec for SECCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seccfgr::R](R) reader structure"]
impl crate::Readable for SECCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seccfgr::W](W) writer structure"]
impl crate::Writable for SECCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SECCFGR to value 0"]
impl crate::Resettable for SECCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
