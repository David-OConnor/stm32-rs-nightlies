#[doc = "Register `RCC_MP_APB3LPENCLRR` reader"]
pub struct R(crate::R<RCC_MP_APB3LPENCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_APB3LPENCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_APB3LPENCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_APB3LPENCLRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MP_APB3LPENCLRR` writer"]
pub struct W(crate::W<RCC_MP_APB3LPENCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_APB3LPENCLRR_SPEC>;
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
impl From<crate::W<RCC_MP_APB3LPENCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_APB3LPENCLRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPTIM2LPEN` reader - LPTIM2LPEN"]
pub type LPTIM2LPEN_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM2LPEN` writer - LPTIM2LPEN"]
pub type LPTIM2LPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_APB3LPENCLRR_SPEC, bool, O>;
#[doc = "Field `LPTIM3LPEN` reader - LPTIM3LPEN"]
pub type LPTIM3LPEN_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM3LPEN` writer - LPTIM3LPEN"]
pub type LPTIM3LPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_APB3LPENCLRR_SPEC, bool, O>;
#[doc = "Field `LPTIM4LPEN` reader - LPTIM4LPEN"]
pub type LPTIM4LPEN_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM4LPEN` writer - LPTIM4LPEN"]
pub type LPTIM4LPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_APB3LPENCLRR_SPEC, bool, O>;
#[doc = "Field `LPTIM5LPEN` reader - LPTIM5LPEN"]
pub type LPTIM5LPEN_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM5LPEN` writer - LPTIM5LPEN"]
pub type LPTIM5LPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_APB3LPENCLRR_SPEC, bool, O>;
#[doc = "Field `SAI4LPEN` reader - SAI4LPEN"]
pub type SAI4LPEN_R = crate::BitReader<bool>;
#[doc = "Field `SAI4LPEN` writer - SAI4LPEN"]
pub type SAI4LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB3LPENCLRR_SPEC, bool, O>;
#[doc = "Field `SYSCFGLPEN` reader - SYSCFGLPEN"]
pub type SYSCFGLPEN_R = crate::BitReader<bool>;
#[doc = "Field `SYSCFGLPEN` writer - SYSCFGLPEN"]
pub type SYSCFGLPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_APB3LPENCLRR_SPEC, bool, O>;
#[doc = "Field `VREFLPEN` reader - VREFLPEN"]
pub type VREFLPEN_R = crate::BitReader<bool>;
#[doc = "Field `VREFLPEN` writer - VREFLPEN"]
pub type VREFLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB3LPENCLRR_SPEC, bool, O>;
#[doc = "Field `DTSLPEN` reader - DTSLPEN"]
pub type DTSLPEN_R = crate::BitReader<bool>;
#[doc = "Field `DTSLPEN` writer - DTSLPEN"]
pub type DTSLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB3LPENCLRR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - LPTIM2LPEN"]
    #[inline(always)]
    pub fn lptim2lpen(&self) -> LPTIM2LPEN_R {
        LPTIM2LPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPTIM3LPEN"]
    #[inline(always)]
    pub fn lptim3lpen(&self) -> LPTIM3LPEN_R {
        LPTIM3LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LPTIM4LPEN"]
    #[inline(always)]
    pub fn lptim4lpen(&self) -> LPTIM4LPEN_R {
        LPTIM4LPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LPTIM5LPEN"]
    #[inline(always)]
    pub fn lptim5lpen(&self) -> LPTIM5LPEN_R {
        LPTIM5LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - SAI4LPEN"]
    #[inline(always)]
    pub fn sai4lpen(&self) -> SAI4LPEN_R {
        SAI4LPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - SYSCFGLPEN"]
    #[inline(always)]
    pub fn syscfglpen(&self) -> SYSCFGLPEN_R {
        SYSCFGLPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - VREFLPEN"]
    #[inline(always)]
    pub fn vreflpen(&self) -> VREFLPEN_R {
        VREFLPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - DTSLPEN"]
    #[inline(always)]
    pub fn dtslpen(&self) -> DTSLPEN_R {
        DTSLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPTIM2LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2lpen(&mut self) -> LPTIM2LPEN_W<0> {
        LPTIM2LPEN_W::new(self)
    }
    #[doc = "Bit 1 - LPTIM3LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn lptim3lpen(&mut self) -> LPTIM3LPEN_W<1> {
        LPTIM3LPEN_W::new(self)
    }
    #[doc = "Bit 2 - LPTIM4LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn lptim4lpen(&mut self) -> LPTIM4LPEN_W<2> {
        LPTIM4LPEN_W::new(self)
    }
    #[doc = "Bit 3 - LPTIM5LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn lptim5lpen(&mut self) -> LPTIM5LPEN_W<3> {
        LPTIM5LPEN_W::new(self)
    }
    #[doc = "Bit 8 - SAI4LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn sai4lpen(&mut self) -> SAI4LPEN_W<8> {
        SAI4LPEN_W::new(self)
    }
    #[doc = "Bit 11 - SYSCFGLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn syscfglpen(&mut self) -> SYSCFGLPEN_W<11> {
        SYSCFGLPEN_W::new(self)
    }
    #[doc = "Bit 13 - VREFLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn vreflpen(&mut self) -> VREFLPEN_W<13> {
        VREFLPEN_W::new(self)
    }
    #[doc = "Bit 16 - DTSLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn dtslpen(&mut self) -> DTSLPEN_W<16> {
        DTSLPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_apb3lpenclrr](index.html) module"]
pub struct RCC_MP_APB3LPENCLRR_SPEC;
impl crate::RegisterSpec for RCC_MP_APB3LPENCLRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mp_apb3lpenclrr::R](R) reader structure"]
impl crate::Readable for RCC_MP_APB3LPENCLRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mp_apb3lpenclrr::W](W) writer structure"]
impl crate::Writable for RCC_MP_APB3LPENCLRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_MP_APB3LPENCLRR to value 0x0003_290f"]
impl crate::Resettable for RCC_MP_APB3LPENCLRR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_290f;
}
