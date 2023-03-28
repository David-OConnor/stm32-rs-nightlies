#[doc = "Register `RCC_MPCKSELR` reader"]
pub struct R(crate::R<RCC_MPCKSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MPCKSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MPCKSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MPCKSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MPCKSELR` writer"]
pub struct W(crate::W<RCC_MPCKSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MPCKSELR_SPEC>;
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
impl From<crate::W<RCC_MPCKSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MPCKSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPUSRC` reader - MPUSRC"]
pub type MPUSRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MPUSRC` writer - MPUSRC"]
pub type MPUSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_MPCKSELR_SPEC, u8, u8, 2, O>;
#[doc = "Field `MPUSRCRDY` reader - MPUSRCRDY"]
pub type MPUSRCRDY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:1 - MPUSRC"]
    #[inline(always)]
    pub fn mpusrc(&self) -> MPUSRC_R {
        MPUSRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 31 - MPUSRCRDY"]
    #[inline(always)]
    pub fn mpusrcrdy(&self) -> MPUSRCRDY_R {
        MPUSRCRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - MPUSRC"]
    #[inline(always)]
    #[must_use]
    pub fn mpusrc(&mut self) -> MPUSRC_W<0> {
        MPUSRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to select the clock source for the MPU. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mpckselr](index.html) module"]
pub struct RCC_MPCKSELR_SPEC;
impl crate::RegisterSpec for RCC_MPCKSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mpckselr::R](R) reader structure"]
impl crate::Readable for RCC_MPCKSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mpckselr::W](W) writer structure"]
impl crate::Writable for RCC_MPCKSELR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_MPCKSELR to value 0x8000_0000"]
impl crate::Resettable for RCC_MPCKSELR_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
