#[doc = "Register `RCC_PLL2FRACR` reader"]
pub struct R(crate::R<RCC_PLL2FRACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_PLL2FRACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_PLL2FRACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_PLL2FRACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_PLL2FRACR` writer"]
pub struct W(crate::W<RCC_PLL2FRACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_PLL2FRACR_SPEC>;
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
impl From<crate::W<RCC_PLL2FRACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_PLL2FRACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRACV` reader - FRACV"]
pub type FRACV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FRACV` writer - FRACV"]
pub type FRACV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RCC_PLL2FRACR_SPEC, u16, u16, 13, O>;
#[doc = "Field `FRACLE` reader - FRACLE"]
pub type FRACLE_R = crate::BitReader<bool>;
#[doc = "Field `FRACLE` writer - FRACLE"]
pub type FRACLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_PLL2FRACR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 3:15 - FRACV"]
    #[inline(always)]
    pub fn fracv(&self) -> FRACV_R {
        FRACV_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
    #[doc = "Bit 16 - FRACLE"]
    #[inline(always)]
    pub fn fracle(&self) -> FRACLE_R {
        FRACLE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 3:15 - FRACV"]
    #[inline(always)]
    #[must_use]
    pub fn fracv(&mut self) -> FRACV_W<3> {
        FRACV_W::new(self)
    }
    #[doc = "Bit 16 - FRACLE"]
    #[inline(always)]
    #[must_use]
    pub fn fracle(&mut self) -> FRACLE_W<16> {
        FRACLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to fine-tune the frequency of the PLL2 VCO. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_pll2fracr](index.html) module"]
pub struct RCC_PLL2FRACR_SPEC;
impl crate::RegisterSpec for RCC_PLL2FRACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_pll2fracr::R](R) reader structure"]
impl crate::Readable for RCC_PLL2FRACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_pll2fracr::W](W) writer structure"]
impl crate::Writable for RCC_PLL2FRACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_PLL2FRACR to value 0"]
impl crate::Resettable for RCC_PLL2FRACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
