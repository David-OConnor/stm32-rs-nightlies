#[doc = "Register `PRIVCFGR` reader"]
pub struct R(crate::R<PRIVCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRIVCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRIVCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRIVCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRIVCFGR` writer"]
pub struct W(crate::W<PRIVCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRIVCFGR_SPEC>;
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
impl From<crate::W<PRIVCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRIVCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRIV` reader - PRIV"]
pub type PRIV_R = crate::BitReader<bool>;
#[doc = "Field `PRIV` writer - PRIV"]
pub type PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - PRIV"]
    #[inline(always)]
    pub fn priv_(&self) -> PRIV_R {
        PRIV_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PRIV"]
    #[inline(always)]
    #[must_use]
    pub fn priv_(&mut self) -> PRIV_W<0> {
        PRIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power privilege configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [privcfgr](index.html) module"]
pub struct PRIVCFGR_SPEC;
impl crate::RegisterSpec for PRIVCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [privcfgr::R](R) reader structure"]
impl crate::Readable for PRIVCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [privcfgr::W](W) writer structure"]
impl crate::Writable for PRIVCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRIVCFGR to value 0"]
impl crate::Resettable for PRIVCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
