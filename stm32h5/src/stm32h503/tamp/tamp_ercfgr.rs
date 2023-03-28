#[doc = "Register `TAMP_ERCFGR` reader"]
pub struct R(crate::R<TAMP_ERCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAMP_ERCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAMP_ERCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAMP_ERCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAMP_ERCFGR` writer"]
pub struct W(crate::W<TAMP_ERCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAMP_ERCFGR_SPEC>;
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
impl From<crate::W<TAMP_ERCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAMP_ERCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERCFG0` reader - Configurable device secrets configuration"]
pub type ERCFG0_R = crate::BitReader<bool>;
#[doc = "Field `ERCFG0` writer - Configurable device secrets configuration"]
pub type ERCFG0_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_ERCFGR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Configurable device secrets configuration"]
    #[inline(always)]
    pub fn ercfg0(&self) -> ERCFG0_R {
        ERCFG0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configurable device secrets configuration"]
    #[inline(always)]
    #[must_use]
    pub fn ercfg0(&mut self) -> ERCFG0_W<0> {
        ERCFG0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TAMP erase configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_ercfgr](index.html) module"]
pub struct TAMP_ERCFGR_SPEC;
impl crate::RegisterSpec for TAMP_ERCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tamp_ercfgr::R](R) reader structure"]
impl crate::Readable for TAMP_ERCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tamp_ercfgr::W](W) writer structure"]
impl crate::Writable for TAMP_ERCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAMP_ERCFGR to value 0"]
impl crate::Resettable for TAMP_ERCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
