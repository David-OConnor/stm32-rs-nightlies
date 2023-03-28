#[doc = "Register `LOCKR` reader"]
pub struct R(crate::R<LOCKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOCKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOCKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOCKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOCKR` writer"]
pub struct W(crate::W<LOCKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOCKR_SPEC>;
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
impl From<crate::W<LOCKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOCKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK` reader - Global security and privilege configuration registers (EXTI_SECCFGR and EXTI_PRIVCFGR) lock This bit is written once after reset."]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` writer - Global security and privilege configuration registers (EXTI_SECCFGR and EXTI_PRIVCFGR) lock This bit is written once after reset."]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOCKR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Global security and privilege configuration registers (EXTI_SECCFGR and EXTI_PRIVCFGR) lock This bit is written once after reset."]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global security and privilege configuration registers (EXTI_SECCFGR and EXTI_PRIVCFGR) lock This bit is written once after reset."]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<0> {
        LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI lock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lockr](index.html) module"]
pub struct LOCKR_SPEC;
impl crate::RegisterSpec for LOCKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lockr::R](R) reader structure"]
impl crate::Readable for LOCKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lockr::W](W) writer structure"]
impl crate::Writable for LOCKR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOCKR to value 0"]
impl crate::Resettable for LOCKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
