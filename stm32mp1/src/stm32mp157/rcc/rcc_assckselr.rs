#[doc = "Register `RCC_ASSCKSELR` reader"]
pub struct R(crate::R<RCC_ASSCKSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_ASSCKSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_ASSCKSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_ASSCKSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_ASSCKSELR` writer"]
pub struct W(crate::W<RCC_ASSCKSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_ASSCKSELR_SPEC>;
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
impl From<crate::W<RCC_ASSCKSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_ASSCKSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AXISSRC` reader - AXISSRC"]
pub type AXISSRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AXISSRC` writer - AXISSRC"]
pub type AXISSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_ASSCKSELR_SPEC, u8, u8, 3, O>;
#[doc = "Field `AXISSRCRDY` reader - AXISSRCRDY"]
pub type AXISSRCRDY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:2 - AXISSRC"]
    #[inline(always)]
    pub fn axissrc(&self) -> AXISSRC_R {
        AXISSRC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 31 - AXISSRCRDY"]
    #[inline(always)]
    pub fn axissrcrdy(&self) -> AXISSRCRDY_R {
        AXISSRCRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - AXISSRC"]
    #[inline(always)]
    #[must_use]
    pub fn axissrc(&mut self) -> AXISSRC_W<0> {
        AXISSRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to select the clock source for the AXI sub-system. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_assckselr](index.html) module"]
pub struct RCC_ASSCKSELR_SPEC;
impl crate::RegisterSpec for RCC_ASSCKSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_assckselr::R](R) reader structure"]
impl crate::Readable for RCC_ASSCKSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_assckselr::W](W) writer structure"]
impl crate::Writable for RCC_ASSCKSELR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_ASSCKSELR to value 0x8000_0000"]
impl crate::Resettable for RCC_ASSCKSELR_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
