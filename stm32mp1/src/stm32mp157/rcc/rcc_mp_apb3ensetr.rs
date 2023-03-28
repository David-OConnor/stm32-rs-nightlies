#[doc = "Register `RCC_MP_APB3ENSETR` reader"]
pub struct R(crate::R<RCC_MP_APB3ENSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_APB3ENSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_APB3ENSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_APB3ENSETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MP_APB3ENSETR` writer"]
pub struct W(crate::W<RCC_MP_APB3ENSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_APB3ENSETR_SPEC>;
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
impl From<crate::W<RCC_MP_APB3ENSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_APB3ENSETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPTIM2EN` reader - LPTIM2EN"]
pub type LPTIM2EN_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM2EN` writer - LPTIM2EN"]
pub type LPTIM2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB3ENSETR_SPEC, bool, O>;
#[doc = "Field `LPTIM3EN` reader - LPTIM3EN"]
pub type LPTIM3EN_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM3EN` writer - LPTIM3EN"]
pub type LPTIM3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB3ENSETR_SPEC, bool, O>;
#[doc = "Field `LPTIM4EN` reader - LPTIM4EN"]
pub type LPTIM4EN_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM4EN` writer - LPTIM4EN"]
pub type LPTIM4EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB3ENSETR_SPEC, bool, O>;
#[doc = "Field `LPTIM5EN` reader - LPTIM5EN"]
pub type LPTIM5EN_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM5EN` writer - LPTIM5EN"]
pub type LPTIM5EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB3ENSETR_SPEC, bool, O>;
#[doc = "Field `SAI4EN` reader - SAI4EN"]
pub type SAI4EN_R = crate::BitReader<bool>;
#[doc = "Field `SAI4EN` writer - SAI4EN"]
pub type SAI4EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB3ENSETR_SPEC, bool, O>;
#[doc = "Field `SYSCFGEN` reader - SYSCFGEN"]
pub type SYSCFGEN_R = crate::BitReader<bool>;
#[doc = "Field `SYSCFGEN` writer - SYSCFGEN"]
pub type SYSCFGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB3ENSETR_SPEC, bool, O>;
#[doc = "Field `VREFEN` reader - VREFEN"]
pub type VREFEN_R = crate::BitReader<bool>;
#[doc = "Field `VREFEN` writer - VREFEN"]
pub type VREFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB3ENSETR_SPEC, bool, O>;
#[doc = "Field `DTSEN` reader - DTSEN"]
pub type DTSEN_R = crate::BitReader<bool>;
#[doc = "Field `DTSEN` writer - DTSEN"]
pub type DTSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB3ENSETR_SPEC, bool, O>;
#[doc = "Field `HDPEN` reader - HDPEN"]
pub type HDPEN_R = crate::BitReader<bool>;
#[doc = "Field `HDPEN` writer - HDPEN"]
pub type HDPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB3ENSETR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - LPTIM2EN"]
    #[inline(always)]
    pub fn lptim2en(&self) -> LPTIM2EN_R {
        LPTIM2EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPTIM3EN"]
    #[inline(always)]
    pub fn lptim3en(&self) -> LPTIM3EN_R {
        LPTIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LPTIM4EN"]
    #[inline(always)]
    pub fn lptim4en(&self) -> LPTIM4EN_R {
        LPTIM4EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LPTIM5EN"]
    #[inline(always)]
    pub fn lptim5en(&self) -> LPTIM5EN_R {
        LPTIM5EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - SAI4EN"]
    #[inline(always)]
    pub fn sai4en(&self) -> SAI4EN_R {
        SAI4EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - SYSCFGEN"]
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - VREFEN"]
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - DTSEN"]
    #[inline(always)]
    pub fn dtsen(&self) -> DTSEN_R {
        DTSEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - HDPEN"]
    #[inline(always)]
    pub fn hdpen(&self) -> HDPEN_R {
        HDPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPTIM2EN"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2en(&mut self) -> LPTIM2EN_W<0> {
        LPTIM2EN_W::new(self)
    }
    #[doc = "Bit 1 - LPTIM3EN"]
    #[inline(always)]
    #[must_use]
    pub fn lptim3en(&mut self) -> LPTIM3EN_W<1> {
        LPTIM3EN_W::new(self)
    }
    #[doc = "Bit 2 - LPTIM4EN"]
    #[inline(always)]
    #[must_use]
    pub fn lptim4en(&mut self) -> LPTIM4EN_W<2> {
        LPTIM4EN_W::new(self)
    }
    #[doc = "Bit 3 - LPTIM5EN"]
    #[inline(always)]
    #[must_use]
    pub fn lptim5en(&mut self) -> LPTIM5EN_W<3> {
        LPTIM5EN_W::new(self)
    }
    #[doc = "Bit 8 - SAI4EN"]
    #[inline(always)]
    #[must_use]
    pub fn sai4en(&mut self) -> SAI4EN_W<8> {
        SAI4EN_W::new(self)
    }
    #[doc = "Bit 11 - SYSCFGEN"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<11> {
        SYSCFGEN_W::new(self)
    }
    #[doc = "Bit 13 - VREFEN"]
    #[inline(always)]
    #[must_use]
    pub fn vrefen(&mut self) -> VREFEN_W<13> {
        VREFEN_W::new(self)
    }
    #[doc = "Bit 16 - DTSEN"]
    #[inline(always)]
    #[must_use]
    pub fn dtsen(&mut self) -> DTSEN_W<16> {
        DTSEN_W::new(self)
    }
    #[doc = "Bit 20 - HDPEN"]
    #[inline(always)]
    #[must_use]
    pub fn hdpen(&mut self) -> HDPEN_W<20> {
        HDPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to set the peripheral clock enable bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_apb3ensetr](index.html) module"]
pub struct RCC_MP_APB3ENSETR_SPEC;
impl crate::RegisterSpec for RCC_MP_APB3ENSETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mp_apb3ensetr::R](R) reader structure"]
impl crate::Readable for RCC_MP_APB3ENSETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mp_apb3ensetr::W](W) writer structure"]
impl crate::Writable for RCC_MP_APB3ENSETR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_MP_APB3ENSETR to value 0"]
impl crate::Resettable for RCC_MP_APB3ENSETR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
