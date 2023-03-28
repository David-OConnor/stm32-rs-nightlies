#[doc = "Register `HDP_GPOCLR` writer"]
pub struct W(crate::W<HDP_GPOCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HDP_GPOCLR_SPEC>;
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
impl From<crate::W<HDP_GPOCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HDP_GPOCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HDPGPOCLR` writer - HDPGPOCLR"]
pub type HDPGPOCLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HDP_GPOCLR_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7 - HDPGPOCLR"]
    #[inline(always)]
    #[must_use]
    pub fn hdpgpoclr(&mut self) -> HDPGPOCLR_W<0> {
        HDPGPOCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HDP GPO clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hdp_gpoclr](index.html) module"]
pub struct HDP_GPOCLR_SPEC;
impl crate::RegisterSpec for HDP_GPOCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [hdp_gpoclr::W](W) writer structure"]
impl crate::Writable for HDP_GPOCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HDP_GPOCLR to value 0"]
impl crate::Resettable for HDP_GPOCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
