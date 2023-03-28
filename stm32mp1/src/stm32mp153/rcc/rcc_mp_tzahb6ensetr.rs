#[doc = "Register `RCC_MP_TZAHB6ENSETR` reader"]
pub struct R(crate::R<RCC_MP_TZAHB6ENSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_TZAHB6ENSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_TZAHB6ENSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_TZAHB6ENSETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MP_TZAHB6ENSETR` writer"]
pub struct W(crate::W<RCC_MP_TZAHB6ENSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_TZAHB6ENSETR_SPEC>;
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
impl From<crate::W<RCC_MP_TZAHB6ENSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_TZAHB6ENSETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MDMAEN` reader - MDMAEN"]
pub type MDMAEN_R = crate::BitReader<bool>;
#[doc = "Field `MDMAEN` writer - MDMAEN"]
pub type MDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_TZAHB6ENSETR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - MDMAEN"]
    #[inline(always)]
    pub fn mdmaen(&self) -> MDMAEN_R {
        MDMAEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MDMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn mdmaen(&mut self) -> MDMAEN_W<0> {
        MDMAEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_tzahb6ensetr](index.html) module"]
pub struct RCC_MP_TZAHB6ENSETR_SPEC;
impl crate::RegisterSpec for RCC_MP_TZAHB6ENSETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mp_tzahb6ensetr::R](R) reader structure"]
impl crate::Readable for RCC_MP_TZAHB6ENSETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mp_tzahb6ensetr::W](W) writer structure"]
impl crate::Writable for RCC_MP_TZAHB6ENSETR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_MP_TZAHB6ENSETR to value 0"]
impl crate::Resettable for RCC_MP_TZAHB6ENSETR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
