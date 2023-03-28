#[doc = "Register `GPIOZ_SECCFGR` writer"]
pub struct W(crate::W<GPIOZ_SECCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOZ_SECCFGR_SPEC>;
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
impl From<crate::W<GPIOZ_SECCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOZ_SECCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEC0` writer - SEC0"]
pub type SEC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOZ_SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC1` writer - SEC1"]
pub type SEC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOZ_SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC2` writer - SEC2"]
pub type SEC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOZ_SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC3` writer - SEC3"]
pub type SEC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOZ_SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC4` writer - SEC4"]
pub type SEC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOZ_SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC5` writer - SEC5"]
pub type SEC5_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOZ_SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC6` writer - SEC6"]
pub type SEC6_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOZ_SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC7` writer - SEC7"]
pub type SEC7_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOZ_SECCFGR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - SEC0"]
    #[inline(always)]
    #[must_use]
    pub fn sec0(&mut self) -> SEC0_W<0> {
        SEC0_W::new(self)
    }
    #[doc = "Bit 1 - SEC1"]
    #[inline(always)]
    #[must_use]
    pub fn sec1(&mut self) -> SEC1_W<1> {
        SEC1_W::new(self)
    }
    #[doc = "Bit 2 - SEC2"]
    #[inline(always)]
    #[must_use]
    pub fn sec2(&mut self) -> SEC2_W<2> {
        SEC2_W::new(self)
    }
    #[doc = "Bit 3 - SEC3"]
    #[inline(always)]
    #[must_use]
    pub fn sec3(&mut self) -> SEC3_W<3> {
        SEC3_W::new(self)
    }
    #[doc = "Bit 4 - SEC4"]
    #[inline(always)]
    #[must_use]
    pub fn sec4(&mut self) -> SEC4_W<4> {
        SEC4_W::new(self)
    }
    #[doc = "Bit 5 - SEC5"]
    #[inline(always)]
    #[must_use]
    pub fn sec5(&mut self) -> SEC5_W<5> {
        SEC5_W::new(self)
    }
    #[doc = "Bit 6 - SEC6"]
    #[inline(always)]
    #[must_use]
    pub fn sec6(&mut self) -> SEC6_W<6> {
        SEC6_W::new(self)
    }
    #[doc = "Bit 7 - SEC7"]
    #[inline(always)]
    #[must_use]
    pub fn sec7(&mut self) -> SEC7_W<7> {
        SEC7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register provides write access security and can be written only by a secure access. It is used to configure a selected I/O as secure. A non-secure write access to this register is discarded.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioz_seccfgr](index.html) module"]
pub struct GPIOZ_SECCFGR_SPEC;
impl crate::RegisterSpec for GPIOZ_SECCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gpioz_seccfgr::W](W) writer structure"]
impl crate::Writable for GPIOZ_SECCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIOZ_SECCFGR to value 0xff"]
impl crate::Resettable for GPIOZ_SECCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
