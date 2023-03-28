#[doc = "Register `SRDCFGR` reader"]
pub struct R(crate::R<SRDCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRDCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRDCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRDCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRDCFGR` writer"]
pub struct W(crate::W<SRDCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRDCFGR_SPEC>;
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
impl From<crate::W<SRDCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRDCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRDPPRE` reader - SmartRun domain APB4 prescaler Set and reset by software to control the SmartRun domain APB4 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk4 after SRDPPRE write. 0xx: rcc_pclk4 = rcc_hclk4 (default after reset)"]
pub type SRDPPRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SRDPPRE` writer - SmartRun domain APB4 prescaler Set and reset by software to control the SmartRun domain APB4 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk4 after SRDPPRE write. 0xx: rcc_pclk4 = rcc_hclk4 (default after reset)"]
pub type SRDPPRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRDCFGR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 4:6 - SmartRun domain APB4 prescaler Set and reset by software to control the SmartRun domain APB4 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk4 after SRDPPRE write. 0xx: rcc_pclk4 = rcc_hclk4 (default after reset)"]
    #[inline(always)]
    pub fn srdppre(&self) -> SRDPPRE_R {
        SRDPPRE_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - SmartRun domain APB4 prescaler Set and reset by software to control the SmartRun domain APB4 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk4 after SRDPPRE write. 0xx: rcc_pclk4 = rcc_hclk4 (default after reset)"]
    #[inline(always)]
    #[must_use]
    pub fn srdppre(&mut self) -> SRDPPRE_W<4> {
        SRDPPRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srdcfgr](index.html) module"]
pub struct SRDCFGR_SPEC;
impl crate::RegisterSpec for SRDCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srdcfgr::R](R) reader structure"]
impl crate::Readable for SRDCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srdcfgr::W](W) writer structure"]
impl crate::Writable for SRDCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRDCFGR to value 0"]
impl crate::Resettable for SRDCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
