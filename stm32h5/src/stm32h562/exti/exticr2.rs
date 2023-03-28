#[doc = "Register `EXTICR2` reader"]
pub struct R(crate::R<EXTICR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTICR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTICR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTICR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTICR2` writer"]
pub struct W(crate::W<EXTICR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTICR2_SPEC>;
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
impl From<crate::W<EXTICR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTICR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI4` reader - EXTI4 GPIO port selection These bits are written by software to select the source input for EXTI4 external interrupt. When EXTI_SECCFGR1.SEC4 is disabled, EXTI4 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC4 is enabled, EXTI4 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV4 is disabled, EXTI4 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV4 is enabled, EXTI4 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI4` writer - EXTI4 GPIO port selection These bits are written by software to select the source input for EXTI4 external interrupt. When EXTI_SECCFGR1.SEC4 is disabled, EXTI4 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC4 is enabled, EXTI4 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV4 is disabled, EXTI4 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV4 is enabled, EXTI4 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR2_SPEC, u8, u8, 8, O>;
#[doc = "Field `EXTI5` reader - EXTI5 GPIO port selection These bits are written by software to select the source input for EXTI5 external interrupt. When EXTI_SECCFGR1.SEC5 is disabled, EXTI5 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC5 is enabled, EXTI5 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV5 is disabled, EXTI5 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV5 is enabled, EXTI5 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI5` writer - EXTI5 GPIO port selection These bits are written by software to select the source input for EXTI5 external interrupt. When EXTI_SECCFGR1.SEC5 is disabled, EXTI5 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC5 is enabled, EXTI5 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV5 is disabled, EXTI5 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV5 is enabled, EXTI5 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR2_SPEC, u8, u8, 8, O>;
#[doc = "Field `EXTI6` reader - EXTI6 GPIO port selection These bits are written by software to select the source input for EXTI6 external interrupt. When EXTI_SECCFGR1.SEC6 is disabled, EXTI6 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC6 is enabled, EXTI6 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV6 is disabled, EXTI6 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV6 is enabled, EXTI6 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI6` writer - EXTI6 GPIO port selection These bits are written by software to select the source input for EXTI6 external interrupt. When EXTI_SECCFGR1.SEC6 is disabled, EXTI6 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC6 is enabled, EXTI6 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV6 is disabled, EXTI6 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV6 is enabled, EXTI6 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR2_SPEC, u8, u8, 8, O>;
#[doc = "Field `EXTI7` reader - EXTI7 GPIO port selection These bits are written by software to select the source input for EXTI7 external interrupt. When EXTI_SECCFGR1.SEC7 is disabled, EXTI7 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC7 is enabled, EXTI7 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV7 is disabled, EXTI7 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV7 is enabled, EXTI7 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI7` writer - EXTI7 GPIO port selection These bits are written by software to select the source input for EXTI7 external interrupt. When EXTI_SECCFGR1.SEC7 is disabled, EXTI7 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC7 is enabled, EXTI7 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV7 is disabled, EXTI7 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV7 is enabled, EXTI7 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - EXTI4 GPIO port selection These bits are written by software to select the source input for EXTI4 external interrupt. When EXTI_SECCFGR1.SEC4 is disabled, EXTI4 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC4 is enabled, EXTI4 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV4 is disabled, EXTI4 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV4 is enabled, EXTI4 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn exti4(&self) -> EXTI4_R {
        EXTI4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - EXTI5 GPIO port selection These bits are written by software to select the source input for EXTI5 external interrupt. When EXTI_SECCFGR1.SEC5 is disabled, EXTI5 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC5 is enabled, EXTI5 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV5 is disabled, EXTI5 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV5 is enabled, EXTI5 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn exti5(&self) -> EXTI5_R {
        EXTI5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - EXTI6 GPIO port selection These bits are written by software to select the source input for EXTI6 external interrupt. When EXTI_SECCFGR1.SEC6 is disabled, EXTI6 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC6 is enabled, EXTI6 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV6 is disabled, EXTI6 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV6 is enabled, EXTI6 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn exti6(&self) -> EXTI6_R {
        EXTI6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - EXTI7 GPIO port selection These bits are written by software to select the source input for EXTI7 external interrupt. When EXTI_SECCFGR1.SEC7 is disabled, EXTI7 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC7 is enabled, EXTI7 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV7 is disabled, EXTI7 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV7 is enabled, EXTI7 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn exti7(&self) -> EXTI7_R {
        EXTI7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EXTI4 GPIO port selection These bits are written by software to select the source input for EXTI4 external interrupt. When EXTI_SECCFGR1.SEC4 is disabled, EXTI4 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC4 is enabled, EXTI4 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV4 is disabled, EXTI4 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV4 is enabled, EXTI4 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    #[must_use]
    pub fn exti4(&mut self) -> EXTI4_W<0> {
        EXTI4_W::new(self)
    }
    #[doc = "Bits 8:15 - EXTI5 GPIO port selection These bits are written by software to select the source input for EXTI5 external interrupt. When EXTI_SECCFGR1.SEC5 is disabled, EXTI5 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC5 is enabled, EXTI5 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV5 is disabled, EXTI5 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV5 is enabled, EXTI5 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    #[must_use]
    pub fn exti5(&mut self) -> EXTI5_W<8> {
        EXTI5_W::new(self)
    }
    #[doc = "Bits 16:23 - EXTI6 GPIO port selection These bits are written by software to select the source input for EXTI6 external interrupt. When EXTI_SECCFGR1.SEC6 is disabled, EXTI6 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC6 is enabled, EXTI6 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV6 is disabled, EXTI6 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV6 is enabled, EXTI6 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    #[must_use]
    pub fn exti6(&mut self) -> EXTI6_W<16> {
        EXTI6_W::new(self)
    }
    #[doc = "Bits 24:31 - EXTI7 GPIO port selection These bits are written by software to select the source input for EXTI7 external interrupt. When EXTI_SECCFGR1.SEC7 is disabled, EXTI7 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC7 is enabled, EXTI7 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV7 is disabled, EXTI7 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV7 is enabled, EXTI7 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    #[must_use]
    pub fn exti7(&mut self) -> EXTI7_W<24> {
        EXTI7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI external interrupt selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr2](index.html) module"]
pub struct EXTICR2_SPEC;
impl crate::RegisterSpec for EXTICR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exticr2::R](R) reader structure"]
impl crate::Readable for EXTICR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exticr2::W](W) writer structure"]
impl crate::Writable for EXTICR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTICR2 to value 0"]
impl crate::Resettable for EXTICR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
