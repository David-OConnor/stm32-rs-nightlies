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
#[doc = "Field `WUP1SEC` reader - WKUP1 pin security"]
pub type WUP1SEC_R = crate::BitReader<bool>;
#[doc = "Field `WUP1SEC` writer - WKUP1 pin security"]
pub type WUP1SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `WUP2SEC` reader - WKUP2 pin security"]
pub type WUP2SEC_R = crate::BitReader<bool>;
#[doc = "Field `WUP2SEC` writer - WKUP2 pin security"]
pub type WUP2SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `WUP3SEC` reader - WKUP3 pin security"]
pub type WUP3SEC_R = crate::BitReader<bool>;
#[doc = "Field `WUP3SEC` writer - WKUP3 pin security"]
pub type WUP3SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `WUP4SEC` reader - WKUP4 pin security"]
pub type WUP4SEC_R = crate::BitReader<bool>;
#[doc = "Field `WUP4SEC` writer - WKUP4 pin security"]
pub type WUP4SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `WUP5SEC` reader - WKUP5 pin security"]
pub type WUP5SEC_R = crate::BitReader<bool>;
#[doc = "Field `WUP5SEC` writer - WKUP5 pin security"]
pub type WUP5SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `LPMSEC` reader - LPMSEC"]
pub type LPMSEC_R = crate::BitReader<bool>;
#[doc = "Field `LPMSEC` writer - LPMSEC"]
pub type LPMSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `VDMSEC` reader - VDMSEC"]
pub type VDMSEC_R = crate::BitReader<bool>;
#[doc = "Field `VDMSEC` writer - VDMSEC"]
pub type VDMSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `VBSEC` reader - VBSEC"]
pub type VBSEC_R = crate::BitReader<bool>;
#[doc = "Field `VBSEC` writer - VBSEC"]
pub type VBSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `APCSEC` reader - APCSEC"]
pub type APCSEC_R = crate::BitReader<bool>;
#[doc = "Field `APCSEC` writer - APCSEC"]
pub type APCSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - WKUP1 pin security"]
    #[inline(always)]
    pub fn wup1sec(&self) -> WUP1SEC_R {
        WUP1SEC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WKUP2 pin security"]
    #[inline(always)]
    pub fn wup2sec(&self) -> WUP2SEC_R {
        WUP2SEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WKUP3 pin security"]
    #[inline(always)]
    pub fn wup3sec(&self) -> WUP3SEC_R {
        WUP3SEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WKUP4 pin security"]
    #[inline(always)]
    pub fn wup4sec(&self) -> WUP4SEC_R {
        WUP4SEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - WKUP5 pin security"]
    #[inline(always)]
    pub fn wup5sec(&self) -> WUP5SEC_R {
        WUP5SEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - LPMSEC"]
    #[inline(always)]
    pub fn lpmsec(&self) -> LPMSEC_R {
        LPMSEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - VDMSEC"]
    #[inline(always)]
    pub fn vdmsec(&self) -> VDMSEC_R {
        VDMSEC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - VBSEC"]
    #[inline(always)]
    pub fn vbsec(&self) -> VBSEC_R {
        VBSEC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - APCSEC"]
    #[inline(always)]
    pub fn apcsec(&self) -> APCSEC_R {
        APCSEC_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WKUP1 pin security"]
    #[inline(always)]
    #[must_use]
    pub fn wup1sec(&mut self) -> WUP1SEC_W<0> {
        WUP1SEC_W::new(self)
    }
    #[doc = "Bit 1 - WKUP2 pin security"]
    #[inline(always)]
    #[must_use]
    pub fn wup2sec(&mut self) -> WUP2SEC_W<1> {
        WUP2SEC_W::new(self)
    }
    #[doc = "Bit 2 - WKUP3 pin security"]
    #[inline(always)]
    #[must_use]
    pub fn wup3sec(&mut self) -> WUP3SEC_W<2> {
        WUP3SEC_W::new(self)
    }
    #[doc = "Bit 3 - WKUP4 pin security"]
    #[inline(always)]
    #[must_use]
    pub fn wup4sec(&mut self) -> WUP4SEC_W<3> {
        WUP4SEC_W::new(self)
    }
    #[doc = "Bit 4 - WKUP5 pin security"]
    #[inline(always)]
    #[must_use]
    pub fn wup5sec(&mut self) -> WUP5SEC_W<4> {
        WUP5SEC_W::new(self)
    }
    #[doc = "Bit 8 - LPMSEC"]
    #[inline(always)]
    #[must_use]
    pub fn lpmsec(&mut self) -> LPMSEC_W<8> {
        LPMSEC_W::new(self)
    }
    #[doc = "Bit 9 - VDMSEC"]
    #[inline(always)]
    #[must_use]
    pub fn vdmsec(&mut self) -> VDMSEC_W<9> {
        VDMSEC_W::new(self)
    }
    #[doc = "Bit 10 - VBSEC"]
    #[inline(always)]
    #[must_use]
    pub fn vbsec(&mut self) -> VBSEC_W<10> {
        VBSEC_W::new(self)
    }
    #[doc = "Bit 11 - APCSEC"]
    #[inline(always)]
    #[must_use]
    pub fn apcsec(&mut self) -> APCSEC_W<11> {
        APCSEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power secure configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seccfgr](index.html) module"]
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
