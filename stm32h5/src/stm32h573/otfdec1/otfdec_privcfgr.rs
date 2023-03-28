#[doc = "Register `OTFDEC_PRIVCFGR` reader"]
pub struct R(crate::R<OTFDEC_PRIVCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTFDEC_PRIVCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTFDEC_PRIVCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTFDEC_PRIVCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTFDEC_PRIVCFGR` writer"]
pub struct W(crate::W<OTFDEC_PRIVCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTFDEC_PRIVCFGR_SPEC>;
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
impl From<crate::W<OTFDEC_PRIVCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTFDEC_PRIVCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRIV` reader - Privileged access protection. Unprivileged read accesses to registers return zeros Unprivileged write accesses to registers are ignored. Note: This bit can only be written in privileged mode. There is no limitations on reads."]
pub type PRIV_R = crate::BitReader<bool>;
#[doc = "Field `PRIV` writer - Privileged access protection. Unprivileged read accesses to registers return zeros Unprivileged write accesses to registers are ignored. Note: This bit can only be written in privileged mode. There is no limitations on reads."]
pub type PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTFDEC_PRIVCFGR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Privileged access protection. Unprivileged read accesses to registers return zeros Unprivileged write accesses to registers are ignored. Note: This bit can only be written in privileged mode. There is no limitations on reads."]
    #[inline(always)]
    pub fn priv_(&self) -> PRIV_R {
        PRIV_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Privileged access protection. Unprivileged read accesses to registers return zeros Unprivileged write accesses to registers are ignored. Note: This bit can only be written in privileged mode. There is no limitations on reads."]
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
#[doc = "OTFDEC_PRIVCFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otfdec_privcfgr](index.html) module"]
pub struct OTFDEC_PRIVCFGR_SPEC;
impl crate::RegisterSpec for OTFDEC_PRIVCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otfdec_privcfgr::R](R) reader structure"]
impl crate::Readable for OTFDEC_PRIVCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otfdec_privcfgr::W](W) writer structure"]
impl crate::Writable for OTFDEC_PRIVCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTFDEC_PRIVCFGR to value 0"]
impl crate::Resettable for OTFDEC_PRIVCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
