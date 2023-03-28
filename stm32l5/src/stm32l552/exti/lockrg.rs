#[doc = "Register `LOCKRG` reader"]
pub struct R(crate::R<LOCKRG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOCKRG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOCKRG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOCKRG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOCKRG` writer"]
pub struct W(crate::W<LOCKRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOCKRG_SPEC>;
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
impl From<crate::W<LOCKRG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOCKRG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK` reader - LOCK"]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` writer - LOCK"]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOCKRG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - LOCK"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LOCK"]
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
#[doc = "EXTI lock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lockrg](index.html) module"]
pub struct LOCKRG_SPEC;
impl crate::RegisterSpec for LOCKRG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lockrg::R](R) reader structure"]
impl crate::Readable for LOCKRG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lockrg::W](W) writer structure"]
impl crate::Writable for LOCKRG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOCKRG to value 0"]
impl crate::Resettable for LOCKRG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
