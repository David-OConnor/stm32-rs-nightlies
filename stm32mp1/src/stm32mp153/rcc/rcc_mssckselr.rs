#[doc = "Register `RCC_MSSCKSELR` reader"]
pub struct R(crate::R<RCC_MSSCKSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MSSCKSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MSSCKSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MSSCKSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MSSCKSELR` writer"]
pub struct W(crate::W<RCC_MSSCKSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MSSCKSELR_SPEC>;
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
impl From<crate::W<RCC_MSSCKSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MSSCKSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCUSSRC` reader - MCUSSRC"]
pub type MCUSSRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MCUSSRC` writer - MCUSSRC"]
pub type MCUSSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_MSSCKSELR_SPEC, u8, u8, 2, O>;
#[doc = "Field `MCUSSRCRDY` reader - MCUSSRCRDY"]
pub type MCUSSRCRDY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:1 - MCUSSRC"]
    #[inline(always)]
    pub fn mcussrc(&self) -> MCUSSRC_R {
        MCUSSRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 31 - MCUSSRCRDY"]
    #[inline(always)]
    pub fn mcussrcrdy(&self) -> MCUSSRCRDY_R {
        MCUSSRCRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - MCUSSRC"]
    #[inline(always)]
    #[must_use]
    pub fn mcussrc(&mut self) -> MCUSSRC_W<0> {
        MCUSSRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to select the clock source for the MCU sub-system, including the MCU itself. If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mssckselr](index.html) module"]
pub struct RCC_MSSCKSELR_SPEC;
impl crate::RegisterSpec for RCC_MSSCKSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mssckselr::R](R) reader structure"]
impl crate::Readable for RCC_MSSCKSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mssckselr::W](W) writer structure"]
impl crate::Writable for RCC_MSSCKSELR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_MSSCKSELR to value 0x8000_0000"]
impl crate::Resettable for RCC_MSSCKSELR_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
