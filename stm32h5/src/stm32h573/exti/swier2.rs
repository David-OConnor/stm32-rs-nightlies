#[doc = "Register `SWIER2` reader"]
pub struct R(crate::R<SWIER2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWIER2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWIER2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWIER2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWIER2` writer"]
pub struct W(crate::W<SWIER2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWIER2_SPEC>;
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
impl From<crate::W<SWIER2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWIER2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWI46` reader - Software interrupt on event x When EXTI_SECCFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI46_R = crate::BitReader<bool>;
#[doc = "Field `SWI46` writer - Software interrupt on event x When EXTI_SECCFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI46_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWIER2_SPEC, bool, O>;
#[doc = "Field `SWI50` reader - Software interrupt on event x When EXTI_SECCFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI50_R = crate::BitReader<bool>;
#[doc = "Field `SWI50` writer - Software interrupt on event x When EXTI_SECCFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI50_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWIER2_SPEC, bool, O>;
#[doc = "Field `SWI53` reader - Software interrupt on event x When EXTI_SECCFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI53_R = crate::BitReader<bool>;
#[doc = "Field `SWI53` writer - Software interrupt on event x When EXTI_SECCFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI53_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWIER2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 14 - Software interrupt on event x When EXTI_SECCFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn swi46(&self) -> SWI46_R {
        SWI46_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - Software interrupt on event x When EXTI_SECCFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn swi50(&self) -> SWI50_R {
        SWI50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - Software interrupt on event x When EXTI_SECCFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn swi53(&self) -> SWI53_R {
        SWI53_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Software interrupt on event x When EXTI_SECCFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    #[must_use]
    pub fn swi46(&mut self) -> SWI46_W<14> {
        SWI46_W::new(self)
    }
    #[doc = "Bit 18 - Software interrupt on event x When EXTI_SECCFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    #[must_use]
    pub fn swi50(&mut self) -> SWI50_W<18> {
        SWI50_W::new(self)
    }
    #[doc = "Bit 21 - Software interrupt on event x When EXTI_SECCFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    #[must_use]
    pub fn swi53(&mut self) -> SWI53_W<21> {
        SWI53_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI software interrupt event register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swier2](index.html) module"]
pub struct SWIER2_SPEC;
impl crate::RegisterSpec for SWIER2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swier2::R](R) reader structure"]
impl crate::Readable for SWIER2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swier2::W](W) writer structure"]
impl crate::Writable for SWIER2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWIER2 to value 0"]
impl crate::Resettable for SWIER2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
