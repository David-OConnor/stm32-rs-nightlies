#[doc = "Register `RCC_RCK12SELR` reader"]
pub struct R(crate::R<RCC_RCK12SELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_RCK12SELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_RCK12SELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_RCK12SELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_RCK12SELR` writer"]
pub struct W(crate::W<RCC_RCK12SELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_RCK12SELR_SPEC>;
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
impl From<crate::W<RCC_RCK12SELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_RCK12SELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLL12SRC` reader - PLL12SRC"]
pub type PLL12SRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLL12SRC` writer - PLL12SRC"]
pub type PLL12SRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RCC_RCK12SELR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PLL12SRCRDY` reader - PLL12SRCRDY"]
pub type PLL12SRCRDY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:1 - PLL12SRC"]
    #[inline(always)]
    pub fn pll12src(&self) -> PLL12SRC_R {
        PLL12SRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 31 - PLL12SRCRDY"]
    #[inline(always)]
    pub fn pll12srcrdy(&self) -> PLL12SRCRDY_R {
        PLL12SRCRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - PLL12SRC"]
    #[inline(always)]
    #[must_use]
    pub fn pll12src(&mut self) -> PLL12SRC_W<0> {
        PLL12SRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to select the reference clock for PLL1 and PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_rck12selr](index.html) module"]
pub struct RCC_RCK12SELR_SPEC;
impl crate::RegisterSpec for RCC_RCK12SELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_rck12selr::R](R) reader structure"]
impl crate::Readable for RCC_RCK12SELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_rck12selr::W](W) writer structure"]
impl crate::Writable for RCC_RCK12SELR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_RCK12SELR to value 0x8000_0000"]
impl crate::Resettable for RCC_RCK12SELR_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
