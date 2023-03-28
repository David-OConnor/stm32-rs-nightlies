#[doc = "Register `EXTICR1` reader"]
pub struct R(crate::R<EXTICR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTICR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTICR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTICR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTICR1` writer"]
pub struct W(crate::W<EXTICR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTICR1_SPEC>;
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
impl From<crate::W<EXTICR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTICR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI0` reader - EXTI0 GPIO port selection These bits are written by software to select the source input for EXTI0 external interrupt. When EXTI_SECCFGR1.SEC0 is disabled, EXTI0 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC0 is enabled, EXTI0 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV0 is disabled, EXTI0 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV0 is enabled, EXTI0 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI0` writer - EXTI0 GPIO port selection These bits are written by software to select the source input for EXTI0 external interrupt. When EXTI_SECCFGR1.SEC0 is disabled, EXTI0 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC0 is enabled, EXTI0 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV0 is disabled, EXTI0 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV0 is enabled, EXTI0 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `EXTI1` reader - EXTI1 GPIO port selection These bits are written by software to select the source input for EXTI1 external interrupt. When EXTI_SECCFGR1.SEC1 is disabled, EXTI1 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC1 is enabled, EXTI1 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV1 is disabled, EXTI1 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV1 is enabled, EXTI1 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI1` writer - EXTI1 GPIO port selection These bits are written by software to select the source input for EXTI1 external interrupt. When EXTI_SECCFGR1.SEC1 is disabled, EXTI1 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC1 is enabled, EXTI1 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV1 is disabled, EXTI1 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV1 is enabled, EXTI1 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `EXTI2` reader - EXTI2 GPIO port selection These bits are written by software to select the source input for EXTI2 external interrupt. When EXTI_SECCFGR1.SEC2 is disabled, EXTI2 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC2 is enabled, EXTI2 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV2 is disabled, EXTI2 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV2 is enabled, EXTI2 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI2` writer - EXTI2 GPIO port selection These bits are written by software to select the source input for EXTI2 external interrupt. When EXTI_SECCFGR1.SEC2 is disabled, EXTI2 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC2 is enabled, EXTI2 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV2 is disabled, EXTI2 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV2 is enabled, EXTI2 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `EXTI3` reader - EXTI3 GPIO port selection These bits are written by software to select the source input for EXTI3 external interrupt. When EXTI_SECCFGR1.SEC3 is disabled, EXTI3 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC3 is enabled, EXTI3 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV3 is disabled, EXTI3 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV3 is enabled, EXTI3 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI3` writer - EXTI3 GPIO port selection These bits are written by software to select the source input for EXTI3 external interrupt. When EXTI_SECCFGR1.SEC3 is disabled, EXTI3 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC3 is enabled, EXTI3 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV3 is disabled, EXTI3 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV3 is enabled, EXTI3 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - EXTI0 GPIO port selection These bits are written by software to select the source input for EXTI0 external interrupt. When EXTI_SECCFGR1.SEC0 is disabled, EXTI0 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC0 is enabled, EXTI0 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV0 is disabled, EXTI0 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV0 is enabled, EXTI0 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn exti0(&self) -> EXTI0_R {
        EXTI0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - EXTI1 GPIO port selection These bits are written by software to select the source input for EXTI1 external interrupt. When EXTI_SECCFGR1.SEC1 is disabled, EXTI1 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC1 is enabled, EXTI1 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV1 is disabled, EXTI1 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV1 is enabled, EXTI1 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn exti1(&self) -> EXTI1_R {
        EXTI1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - EXTI2 GPIO port selection These bits are written by software to select the source input for EXTI2 external interrupt. When EXTI_SECCFGR1.SEC2 is disabled, EXTI2 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC2 is enabled, EXTI2 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV2 is disabled, EXTI2 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV2 is enabled, EXTI2 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn exti2(&self) -> EXTI2_R {
        EXTI2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - EXTI3 GPIO port selection These bits are written by software to select the source input for EXTI3 external interrupt. When EXTI_SECCFGR1.SEC3 is disabled, EXTI3 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC3 is enabled, EXTI3 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV3 is disabled, EXTI3 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV3 is enabled, EXTI3 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn exti3(&self) -> EXTI3_R {
        EXTI3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EXTI0 GPIO port selection These bits are written by software to select the source input for EXTI0 external interrupt. When EXTI_SECCFGR1.SEC0 is disabled, EXTI0 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC0 is enabled, EXTI0 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV0 is disabled, EXTI0 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV0 is enabled, EXTI0 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    #[must_use]
    pub fn exti0(&mut self) -> EXTI0_W<0> {
        EXTI0_W::new(self)
    }
    #[doc = "Bits 8:15 - EXTI1 GPIO port selection These bits are written by software to select the source input for EXTI1 external interrupt. When EXTI_SECCFGR1.SEC1 is disabled, EXTI1 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC1 is enabled, EXTI1 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV1 is disabled, EXTI1 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV1 is enabled, EXTI1 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    #[must_use]
    pub fn exti1(&mut self) -> EXTI1_W<8> {
        EXTI1_W::new(self)
    }
    #[doc = "Bits 16:23 - EXTI2 GPIO port selection These bits are written by software to select the source input for EXTI2 external interrupt. When EXTI_SECCFGR1.SEC2 is disabled, EXTI2 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC2 is enabled, EXTI2 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV2 is disabled, EXTI2 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV2 is enabled, EXTI2 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    #[must_use]
    pub fn exti2(&mut self) -> EXTI2_W<16> {
        EXTI2_W::new(self)
    }
    #[doc = "Bits 24:31 - EXTI3 GPIO port selection These bits are written by software to select the source input for EXTI3 external interrupt. When EXTI_SECCFGR1.SEC3 is disabled, EXTI3 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC3 is enabled, EXTI3 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV3 is disabled, EXTI3 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV3 is enabled, EXTI3 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    #[must_use]
    pub fn exti3(&mut self) -> EXTI3_W<24> {
        EXTI3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI external interrupt selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr1](index.html) module"]
pub struct EXTICR1_SPEC;
impl crate::RegisterSpec for EXTICR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exticr1::R](R) reader structure"]
impl crate::Readable for EXTICR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exticr1::W](W) writer structure"]
impl crate::Writable for EXTICR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTICR1 to value 0"]
impl crate::Resettable for EXTICR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
