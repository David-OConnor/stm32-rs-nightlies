#[doc = "Register `IWDG_KR` writer"]
pub struct W(crate::W<IWDG_KR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IWDG_KR_SPEC>;
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
impl From<crate::W<IWDG_KR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IWDG_KR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY` writer - KEY"]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IWDG_KR_SPEC, u16, u16, 16, O>;
impl W {
    #[doc = "Bits 0:15 - KEY"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<0> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iwdg_kr](index.html) module"]
pub struct IWDG_KR_SPEC;
impl crate::RegisterSpec for IWDG_KR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [iwdg_kr::W](W) writer structure"]
impl crate::Writable for IWDG_KR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IWDG_KR to value 0"]
impl crate::Resettable for IWDG_KR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
