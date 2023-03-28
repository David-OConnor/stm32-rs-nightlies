#[doc = "Register `RCC_MP_GCR` reader"]
pub struct R(crate::R<RCC_MP_GCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_GCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_GCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_GCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MP_GCR` writer"]
pub struct W(crate::W<RCC_MP_GCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_GCR_SPEC>;
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
impl From<crate::W<RCC_MP_GCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_GCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BOOT_MCU` reader - BOOT_MCU"]
pub type BOOT_MCU_R = crate::BitReader<bool>;
#[doc = "Field `BOOT_MCU` writer - BOOT_MCU"]
pub type BOOT_MCU_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_GCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - BOOT_MCU"]
    #[inline(always)]
    pub fn boot_mcu(&self) -> BOOT_MCU_R {
        BOOT_MCU_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BOOT_MCU"]
    #[inline(always)]
    #[must_use]
    pub fn boot_mcu(&mut self) -> BOOT_MCU_W<0> {
        BOOT_MCU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The register contains global control bits. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_gcr](index.html) module"]
pub struct RCC_MP_GCR_SPEC;
impl crate::RegisterSpec for RCC_MP_GCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mp_gcr::R](R) reader structure"]
impl crate::Readable for RCC_MP_GCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mp_gcr::W](W) writer structure"]
impl crate::Writable for RCC_MP_GCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_MP_GCR to value 0"]
impl crate::Resettable for RCC_MP_GCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
