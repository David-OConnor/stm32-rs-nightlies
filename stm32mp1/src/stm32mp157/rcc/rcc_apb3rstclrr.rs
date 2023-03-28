#[doc = "Register `RCC_APB3RSTCLRR` reader"]
pub struct R(crate::R<RCC_APB3RSTCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_APB3RSTCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_APB3RSTCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_APB3RSTCLRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_APB3RSTCLRR` writer"]
pub struct W(crate::W<RCC_APB3RSTCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_APB3RSTCLRR_SPEC>;
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
impl From<crate::W<RCC_APB3RSTCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_APB3RSTCLRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPTIM2RST` reader - LPTIM2RST"]
pub type LPTIM2RST_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM2RST` writer - LPTIM2RST"]
pub type LPTIM2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3RSTCLRR_SPEC, bool, O>;
#[doc = "Field `LPTIM3RST` reader - LPTIM3RST"]
pub type LPTIM3RST_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM3RST` writer - LPTIM3RST"]
pub type LPTIM3RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3RSTCLRR_SPEC, bool, O>;
#[doc = "Field `LPTIM4RST` reader - LPTIM4RST"]
pub type LPTIM4RST_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM4RST` writer - LPTIM4RST"]
pub type LPTIM4RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3RSTCLRR_SPEC, bool, O>;
#[doc = "Field `LPTIM5RST` reader - LPTIM5RST"]
pub type LPTIM5RST_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM5RST` writer - LPTIM5RST"]
pub type LPTIM5RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3RSTCLRR_SPEC, bool, O>;
#[doc = "Field `SAI4RST` reader - SAI4RST"]
pub type SAI4RST_R = crate::BitReader<bool>;
#[doc = "Field `SAI4RST` writer - SAI4RST"]
pub type SAI4RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3RSTCLRR_SPEC, bool, O>;
#[doc = "Field `SYSCFGRST` reader - SYSCFGRST"]
pub type SYSCFGRST_R = crate::BitReader<bool>;
#[doc = "Field `SYSCFGRST` writer - SYSCFGRST"]
pub type SYSCFGRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3RSTCLRR_SPEC, bool, O>;
#[doc = "Field `VREFRST` reader - VREFRST"]
pub type VREFRST_R = crate::BitReader<bool>;
#[doc = "Field `VREFRST` writer - VREFRST"]
pub type VREFRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3RSTCLRR_SPEC, bool, O>;
#[doc = "Field `DTSRST` reader - DTSRST"]
pub type DTSRST_R = crate::BitReader<bool>;
#[doc = "Field `DTSRST` writer - DTSRST"]
pub type DTSRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3RSTCLRR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - LPTIM2RST"]
    #[inline(always)]
    pub fn lptim2rst(&self) -> LPTIM2RST_R {
        LPTIM2RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPTIM3RST"]
    #[inline(always)]
    pub fn lptim3rst(&self) -> LPTIM3RST_R {
        LPTIM3RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LPTIM4RST"]
    #[inline(always)]
    pub fn lptim4rst(&self) -> LPTIM4RST_R {
        LPTIM4RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LPTIM5RST"]
    #[inline(always)]
    pub fn lptim5rst(&self) -> LPTIM5RST_R {
        LPTIM5RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - SAI4RST"]
    #[inline(always)]
    pub fn sai4rst(&self) -> SAI4RST_R {
        SAI4RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - SYSCFGRST"]
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - VREFRST"]
    #[inline(always)]
    pub fn vrefrst(&self) -> VREFRST_R {
        VREFRST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - DTSRST"]
    #[inline(always)]
    pub fn dtsrst(&self) -> DTSRST_R {
        DTSRST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPTIM2RST"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2rst(&mut self) -> LPTIM2RST_W<0> {
        LPTIM2RST_W::new(self)
    }
    #[doc = "Bit 1 - LPTIM3RST"]
    #[inline(always)]
    #[must_use]
    pub fn lptim3rst(&mut self) -> LPTIM3RST_W<1> {
        LPTIM3RST_W::new(self)
    }
    #[doc = "Bit 2 - LPTIM4RST"]
    #[inline(always)]
    #[must_use]
    pub fn lptim4rst(&mut self) -> LPTIM4RST_W<2> {
        LPTIM4RST_W::new(self)
    }
    #[doc = "Bit 3 - LPTIM5RST"]
    #[inline(always)]
    #[must_use]
    pub fn lptim5rst(&mut self) -> LPTIM5RST_W<3> {
        LPTIM5RST_W::new(self)
    }
    #[doc = "Bit 8 - SAI4RST"]
    #[inline(always)]
    #[must_use]
    pub fn sai4rst(&mut self) -> SAI4RST_W<8> {
        SAI4RST_W::new(self)
    }
    #[doc = "Bit 11 - SYSCFGRST"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<11> {
        SYSCFGRST_W::new(self)
    }
    #[doc = "Bit 13 - VREFRST"]
    #[inline(always)]
    #[must_use]
    pub fn vrefrst(&mut self) -> VREFRST_W<13> {
        VREFRST_W::new(self)
    }
    #[doc = "Bit 16 - DTSRST"]
    #[inline(always)]
    #[must_use]
    pub fn dtsrst(&mut self) -> DTSRST_W<16> {
        DTSRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to release the reset of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb3rstclrr](index.html) module"]
pub struct RCC_APB3RSTCLRR_SPEC;
impl crate::RegisterSpec for RCC_APB3RSTCLRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_apb3rstclrr::R](R) reader structure"]
impl crate::Readable for RCC_APB3RSTCLRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_apb3rstclrr::W](W) writer structure"]
impl crate::Writable for RCC_APB3RSTCLRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_APB3RSTCLRR to value 0"]
impl crate::Resettable for RCC_APB3RSTCLRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
