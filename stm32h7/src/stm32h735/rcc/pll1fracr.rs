#[doc = "Register `PLL1FRACR` reader"]
pub struct R(crate::R<PLL1FRACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL1FRACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL1FRACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL1FRACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL1FRACR` writer"]
pub struct W(crate::W<PLL1FRACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL1FRACR_SPEC>;
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
impl From<crate::W<PLL1FRACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL1FRACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRACN1` reader - Fractional part of the multiplication factor for PLL1 VCO"]
pub type FRACN1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FRACN1` writer - Fractional part of the multiplication factor for PLL1 VCO"]
pub type FRACN1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PLL1FRACR_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 3:15 - Fractional part of the multiplication factor for PLL1 VCO"]
    #[inline(always)]
    pub fn fracn1(&self) -> FRACN1_R {
        FRACN1_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 3:15 - Fractional part of the multiplication factor for PLL1 VCO"]
    #[inline(always)]
    #[must_use]
    pub fn fracn1(&mut self) -> FRACN1_W<3> {
        FRACN1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC PLL1 Fractional Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll1fracr](index.html) module"]
pub struct PLL1FRACR_SPEC;
impl crate::RegisterSpec for PLL1FRACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll1fracr::R](R) reader structure"]
impl crate::Readable for PLL1FRACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll1fracr::W](W) writer structure"]
impl crate::Writable for PLL1FRACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL1FRACR to value 0"]
impl crate::Resettable for PLL1FRACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
