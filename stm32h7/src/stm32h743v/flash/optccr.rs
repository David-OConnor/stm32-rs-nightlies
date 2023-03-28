#[doc = "Register `OPTCCR` writer"]
pub struct W(crate::W<OPTCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTCCR_SPEC>;
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
impl From<crate::W<OPTCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLR_OPTCHANGEERR` writer - OPTCHANGEERR reset bit"]
pub type CLR_OPTCHANGEERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTCCR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 30 - OPTCHANGEERR reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn clr_optchangeerr(&mut self) -> CLR_OPTCHANGEERR_W<30> {
        CLR_OPTCHANGEERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH option clear control register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optccr](index.html) module"]
pub struct OPTCCR_SPEC;
impl crate::RegisterSpec for OPTCCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [optccr::W](W) writer structure"]
impl crate::Writable for OPTCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPTCCR to value 0"]
impl crate::Resettable for OPTCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
