#[doc = "Register `LTDC_L1CLUTWR` writer"]
pub struct W(crate::W<LTDC_L1CLUTWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTDC_L1CLUTWR_SPEC>;
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
impl From<crate::W<LTDC_L1CLUTWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTDC_L1CLUTWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLUE` writer - BLUE"]
pub type BLUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LTDC_L1CLUTWR_SPEC, u8, u8, 8, O>;
#[doc = "Field `GREEN` writer - GREEN"]
pub type GREEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LTDC_L1CLUTWR_SPEC, u8, u8, 8, O>;
#[doc = "Field `RED` writer - RED"]
pub type RED_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LTDC_L1CLUTWR_SPEC, u8, u8, 8, O>;
#[doc = "Field `CLUTADD` writer - CLUTADD"]
pub type CLUTADD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LTDC_L1CLUTWR_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7 - BLUE"]
    #[inline(always)]
    #[must_use]
    pub fn blue(&mut self) -> BLUE_W<0> {
        BLUE_W::new(self)
    }
    #[doc = "Bits 8:15 - GREEN"]
    #[inline(always)]
    #[must_use]
    pub fn green(&mut self) -> GREEN_W<8> {
        GREEN_W::new(self)
    }
    #[doc = "Bits 16:23 - RED"]
    #[inline(always)]
    #[must_use]
    pub fn red(&mut self) -> RED_W<16> {
        RED_W::new(self)
    }
    #[doc = "Bits 24:31 - CLUTADD"]
    #[inline(always)]
    #[must_use]
    pub fn clutadd(&mut self) -> CLUTADD_W<24> {
        CLUTADD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register defines the CLUT address and the RGB value.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l1clutwr](index.html) module"]
pub struct LTDC_L1CLUTWR_SPEC;
impl crate::RegisterSpec for LTDC_L1CLUTWR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ltdc_l1clutwr::W](W) writer structure"]
impl crate::Writable for LTDC_L1CLUTWR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LTDC_L1CLUTWR to value 0"]
impl crate::Resettable for LTDC_L1CLUTWR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
