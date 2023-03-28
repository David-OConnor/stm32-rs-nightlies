#[doc = "Register `CFGR` reader"]
pub struct R(crate::R<CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR` writer"]
pub struct W(crate::W<CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR_SPEC>;
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
impl From<crate::W<CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMONEN` reader - TMONEN"]
pub type TMONEN_R = crate::BitReader<bool>;
#[doc = "Field `TMONEN` writer - TMONEN"]
pub type TMONEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `VMONEN` reader - VMONEN"]
pub type VMONEN_R = crate::BitReader<bool>;
#[doc = "Field `VMONEN` writer - VMONEN"]
pub type VMONEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `WUTMONEN` reader - WUTMONEN"]
pub type WUTMONEN_R = crate::BitReader<bool>;
#[doc = "Field `WUTMONEN` writer - WUTMONEN"]
pub type WUTMONEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - TMONEN"]
    #[inline(always)]
    pub fn tmonen(&self) -> TMONEN_R {
        TMONEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VMONEN"]
    #[inline(always)]
    pub fn vmonen(&self) -> VMONEN_R {
        VMONEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WUTMONEN"]
    #[inline(always)]
    pub fn wutmonen(&self) -> WUTMONEN_R {
        WUTMONEN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - TMONEN"]
    #[inline(always)]
    #[must_use]
    pub fn tmonen(&mut self) -> TMONEN_W<1> {
        TMONEN_W::new(self)
    }
    #[doc = "Bit 2 - VMONEN"]
    #[inline(always)]
    #[must_use]
    pub fn vmonen(&mut self) -> VMONEN_W<2> {
        VMONEN_W::new(self)
    }
    #[doc = "Bit 3 - WUTMONEN"]
    #[inline(always)]
    #[must_use]
    pub fn wutmonen(&mut self) -> WUTMONEN_W<3> {
        WUTMONEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TAMP configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](index.html) module"]
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr::R](R) reader structure"]
impl crate::Readable for CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr::W](W) writer structure"]
impl crate::Writable for CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
