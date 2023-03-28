#[doc = "Register `RCC_AXIDIVR` reader"]
pub struct R(crate::R<RCC_AXIDIVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_AXIDIVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_AXIDIVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_AXIDIVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_AXIDIVR` writer"]
pub struct W(crate::W<RCC_AXIDIVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_AXIDIVR_SPEC>;
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
impl From<crate::W<RCC_AXIDIVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_AXIDIVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AXIDIV` reader - AXIDIV"]
pub type AXIDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AXIDIV` writer - AXIDIV"]
pub type AXIDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_AXIDIVR_SPEC, u8, u8, 3, O>;
#[doc = "Field `AXIDIVRDY` reader - AXIDIVRDY"]
pub type AXIDIVRDY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:2 - AXIDIV"]
    #[inline(always)]
    pub fn axidiv(&self) -> AXIDIV_R {
        AXIDIV_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 31 - AXIDIVRDY"]
    #[inline(always)]
    pub fn axidivrdy(&self) -> AXIDIVRDY_R {
        AXIDIVRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - AXIDIV"]
    #[inline(always)]
    #[must_use]
    pub fn axidiv(&mut self) -> AXIDIV_W<0> {
        AXIDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the AXI Matrix clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_axidivr](index.html) module"]
pub struct RCC_AXIDIVR_SPEC;
impl crate::RegisterSpec for RCC_AXIDIVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_axidivr::R](R) reader structure"]
impl crate::Readable for RCC_AXIDIVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_axidivr::W](W) writer structure"]
impl crate::Writable for RCC_AXIDIVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_AXIDIVR to value 0x8000_0000"]
impl crate::Resettable for RCC_AXIDIVR_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
