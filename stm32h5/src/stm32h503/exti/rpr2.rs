#[doc = "Register `RPR2` reader"]
pub struct R(crate::R<RPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RPR2` writer"]
pub struct W(crate::W<RPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RPR2_SPEC>;
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
impl From<crate::W<RPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RPIF50` reader - configurable event inputs x rising edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF50_R = crate::BitReader<bool>;
#[doc = "Field `RPIF50` writer - configurable event inputs x rising edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF50_W<'a, const O: u8> = crate::BitWriter<'a, u32, RPR2_SPEC, bool, O>;
#[doc = "Field `RPIF53` reader - configurable event inputs x rising edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF53_R = crate::BitReader<bool>;
#[doc = "Field `RPIF53` writer - configurable event inputs x rising edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF53_W<'a, const O: u8> = crate::BitWriter<'a, u32, RPR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 18 - configurable event inputs x rising edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn rpif50(&self) -> RPIF50_R {
        RPIF50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - configurable event inputs x rising edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn rpif53(&self) -> RPIF53_R {
        RPIF53_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - configurable event inputs x rising edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    #[must_use]
    pub fn rpif50(&mut self) -> RPIF50_W<18> {
        RPIF50_W::new(self)
    }
    #[doc = "Bit 21 - configurable event inputs x rising edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    #[must_use]
    pub fn rpif53(&mut self) -> RPIF53_W<21> {
        RPIF53_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI rising edge pending register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpr2](index.html) module"]
pub struct RPR2_SPEC;
impl crate::RegisterSpec for RPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rpr2::R](R) reader structure"]
impl crate::Readable for RPR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rpr2::W](W) writer structure"]
impl crate::Writable for RPR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RPR2 to value 0"]
impl crate::Resettable for RPR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
