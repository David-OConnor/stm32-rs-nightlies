#[doc = "Register `IWDG_PR` reader"]
pub struct R(crate::R<IWDG_PR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IWDG_PR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IWDG_PR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IWDG_PR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IWDG_PR` writer"]
pub struct W(crate::W<IWDG_PR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IWDG_PR_SPEC>;
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
impl From<crate::W<IWDG_PR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IWDG_PR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PR` reader - PR"]
pub type PR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PR` writer - PR"]
pub type PR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IWDG_PR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - PR"]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - PR"]
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PR_W<0> {
        PR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Prescaler register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iwdg_pr](index.html) module"]
pub struct IWDG_PR_SPEC;
impl crate::RegisterSpec for IWDG_PR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iwdg_pr::R](R) reader structure"]
impl crate::Readable for IWDG_PR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iwdg_pr::W](W) writer structure"]
impl crate::Writable for IWDG_PR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IWDG_PR to value 0x07"]
impl crate::Resettable for IWDG_PR_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
