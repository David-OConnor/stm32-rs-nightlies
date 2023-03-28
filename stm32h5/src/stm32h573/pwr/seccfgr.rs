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
#[doc = "Field `WUP1SEC` reader - WUPx secure protection"]
pub type WUP1SEC_R = crate::BitReader<bool>;
#[doc = "Field `WUP1SEC` writer - WUPx secure protection"]
pub type WUP1SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `WUP2SEC` reader - WUPx secure protection"]
pub type WUP2SEC_R = crate::BitReader<bool>;
#[doc = "Field `WUP2SEC` writer - WUPx secure protection"]
pub type WUP2SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `WUP3SEC` reader - WUPx secure protection"]
pub type WUP3SEC_R = crate::BitReader<bool>;
#[doc = "Field `WUP3SEC` writer - WUPx secure protection"]
pub type WUP3SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `WUP4SEC` reader - WUPx secure protection"]
pub type WUP4SEC_R = crate::BitReader<bool>;
#[doc = "Field `WUP4SEC` writer - WUPx secure protection"]
pub type WUP4SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `WUP5SEC` reader - WUPx secure protection"]
pub type WUP5SEC_R = crate::BitReader<bool>;
#[doc = "Field `WUP5SEC` writer - WUPx secure protection"]
pub type WUP5SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `WUP6SEC` reader - WUPx secure protection"]
pub type WUP6SEC_R = crate::BitReader<bool>;
#[doc = "Field `WUP6SEC` writer - WUPx secure protection"]
pub type WUP6SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `WUP7SEC` reader - WUPx secure protection"]
pub type WUP7SEC_R = crate::BitReader<bool>;
#[doc = "Field `WUP7SEC` writer - WUPx secure protection"]
pub type WUP7SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `WUP8SEC` reader - WUPx secure protection"]
pub type WUP8SEC_R = crate::BitReader<bool>;
#[doc = "Field `WUP8SEC` writer - WUPx secure protection"]
pub type WUP8SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `RETSEC` reader - retention secure protection"]
pub type RETSEC_R = crate::BitReader<bool>;
#[doc = "Field `RETSEC` writer - retention secure protection"]
pub type RETSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `LPMSEC` reader - low-power modes secure protection"]
pub type LPMSEC_R = crate::BitReader<bool>;
#[doc = "Field `LPMSEC` writer - low-power modes secure protection"]
pub type LPMSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SCMSEC` reader - supply configuration and monitoring secure protection."]
pub type SCMSEC_R = crate::BitReader<bool>;
#[doc = "Field `SCMSEC` writer - supply configuration and monitoring secure protection."]
pub type SCMSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `VBSEC` reader - backup domain secure protection"]
pub type VBSEC_R = crate::BitReader<bool>;
#[doc = "Field `VBSEC` writer - backup domain secure protection"]
pub type VBSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `VUSBSEC` reader - voltage USB secure protection"]
pub type VUSBSEC_R = crate::BitReader<bool>;
#[doc = "Field `VUSBSEC` writer - voltage USB secure protection"]
pub type VUSBSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - WUPx secure protection"]
    #[inline(always)]
    pub fn wup1sec(&self) -> WUP1SEC_R {
        WUP1SEC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WUPx secure protection"]
    #[inline(always)]
    pub fn wup2sec(&self) -> WUP2SEC_R {
        WUP2SEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WUPx secure protection"]
    #[inline(always)]
    pub fn wup3sec(&self) -> WUP3SEC_R {
        WUP3SEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WUPx secure protection"]
    #[inline(always)]
    pub fn wup4sec(&self) -> WUP4SEC_R {
        WUP4SEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - WUPx secure protection"]
    #[inline(always)]
    pub fn wup5sec(&self) -> WUP5SEC_R {
        WUP5SEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - WUPx secure protection"]
    #[inline(always)]
    pub fn wup6sec(&self) -> WUP6SEC_R {
        WUP6SEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - WUPx secure protection"]
    #[inline(always)]
    pub fn wup7sec(&self) -> WUP7SEC_R {
        WUP7SEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - WUPx secure protection"]
    #[inline(always)]
    pub fn wup8sec(&self) -> WUP8SEC_R {
        WUP8SEC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - retention secure protection"]
    #[inline(always)]
    pub fn retsec(&self) -> RETSEC_R {
        RETSEC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - low-power modes secure protection"]
    #[inline(always)]
    pub fn lpmsec(&self) -> LPMSEC_R {
        LPMSEC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - supply configuration and monitoring secure protection."]
    #[inline(always)]
    pub fn scmsec(&self) -> SCMSEC_R {
        SCMSEC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - backup domain secure protection"]
    #[inline(always)]
    pub fn vbsec(&self) -> VBSEC_R {
        VBSEC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - voltage USB secure protection"]
    #[inline(always)]
    pub fn vusbsec(&self) -> VUSBSEC_R {
        VUSBSEC_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WUPx secure protection"]
    #[inline(always)]
    #[must_use]
    pub fn wup1sec(&mut self) -> WUP1SEC_W<0> {
        WUP1SEC_W::new(self)
    }
    #[doc = "Bit 1 - WUPx secure protection"]
    #[inline(always)]
    #[must_use]
    pub fn wup2sec(&mut self) -> WUP2SEC_W<1> {
        WUP2SEC_W::new(self)
    }
    #[doc = "Bit 2 - WUPx secure protection"]
    #[inline(always)]
    #[must_use]
    pub fn wup3sec(&mut self) -> WUP3SEC_W<2> {
        WUP3SEC_W::new(self)
    }
    #[doc = "Bit 3 - WUPx secure protection"]
    #[inline(always)]
    #[must_use]
    pub fn wup4sec(&mut self) -> WUP4SEC_W<3> {
        WUP4SEC_W::new(self)
    }
    #[doc = "Bit 4 - WUPx secure protection"]
    #[inline(always)]
    #[must_use]
    pub fn wup5sec(&mut self) -> WUP5SEC_W<4> {
        WUP5SEC_W::new(self)
    }
    #[doc = "Bit 5 - WUPx secure protection"]
    #[inline(always)]
    #[must_use]
    pub fn wup6sec(&mut self) -> WUP6SEC_W<5> {
        WUP6SEC_W::new(self)
    }
    #[doc = "Bit 6 - WUPx secure protection"]
    #[inline(always)]
    #[must_use]
    pub fn wup7sec(&mut self) -> WUP7SEC_W<6> {
        WUP7SEC_W::new(self)
    }
    #[doc = "Bit 7 - WUPx secure protection"]
    #[inline(always)]
    #[must_use]
    pub fn wup8sec(&mut self) -> WUP8SEC_W<7> {
        WUP8SEC_W::new(self)
    }
    #[doc = "Bit 11 - retention secure protection"]
    #[inline(always)]
    #[must_use]
    pub fn retsec(&mut self) -> RETSEC_W<11> {
        RETSEC_W::new(self)
    }
    #[doc = "Bit 12 - low-power modes secure protection"]
    #[inline(always)]
    #[must_use]
    pub fn lpmsec(&mut self) -> LPMSEC_W<12> {
        LPMSEC_W::new(self)
    }
    #[doc = "Bit 13 - supply configuration and monitoring secure protection."]
    #[inline(always)]
    #[must_use]
    pub fn scmsec(&mut self) -> SCMSEC_W<13> {
        SCMSEC_W::new(self)
    }
    #[doc = "Bit 14 - backup domain secure protection"]
    #[inline(always)]
    #[must_use]
    pub fn vbsec(&mut self) -> VBSEC_W<14> {
        VBSEC_W::new(self)
    }
    #[doc = "Bit 15 - voltage USB secure protection"]
    #[inline(always)]
    #[must_use]
    pub fn vusbsec(&mut self) -> VUSBSEC_W<15> {
        VUSBSEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWR security configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seccfgr](index.html) module"]
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
