#[doc = "Register `LTDC_SRCR` reader"]
pub struct R(crate::R<LTDC_SRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTDC_SRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTDC_SRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTDC_SRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTDC_SRCR` writer"]
pub struct W(crate::W<LTDC_SRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTDC_SRCR_SPEC>;
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
impl From<crate::W<LTDC_SRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTDC_SRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IMR` reader - IMR"]
pub type IMR_R = crate::BitReader<bool>;
#[doc = "Field `IMR` writer - IMR"]
pub type IMR_W<'a, const O: u8> = crate::BitWriter<'a, u32, LTDC_SRCR_SPEC, bool, O>;
#[doc = "Field `VBR` reader - VBR"]
pub type VBR_R = crate::BitReader<bool>;
#[doc = "Field `VBR` writer - VBR"]
pub type VBR_W<'a, const O: u8> = crate::BitWriter<'a, u32, LTDC_SRCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - IMR"]
    #[inline(always)]
    pub fn imr(&self) -> IMR_R {
        IMR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VBR"]
    #[inline(always)]
    pub fn vbr(&self) -> VBR_R {
        VBR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IMR"]
    #[inline(always)]
    #[must_use]
    pub fn imr(&mut self) -> IMR_W<0> {
        IMR_W::new(self)
    }
    #[doc = "Bit 1 - VBR"]
    #[inline(always)]
    #[must_use]
    pub fn vbr(&mut self) -> VBR_W<1> {
        VBR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register allows to reload either immediately or during the vertical blanking period, the shadow registers values to the active registers. The shadow registers are all Layer1 and Layer2 registers except the LTDC_L1CLUTWR and the LTDC_L2CLUTWR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_srcr](index.html) module"]
pub struct LTDC_SRCR_SPEC;
impl crate::RegisterSpec for LTDC_SRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltdc_srcr::R](R) reader structure"]
impl crate::Readable for LTDC_SRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltdc_srcr::W](W) writer structure"]
impl crate::Writable for LTDC_SRCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LTDC_SRCR to value 0"]
impl crate::Resettable for LTDC_SRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
