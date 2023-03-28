#[doc = "Register `RCC_MP_IWDGFZCLRR` reader"]
pub struct R(crate::R<RCC_MP_IWDGFZCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_IWDGFZCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_IWDGFZCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_IWDGFZCLRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MP_IWDGFZCLRR` writer"]
pub struct W(crate::W<RCC_MP_IWDGFZCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_IWDGFZCLRR_SPEC>;
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
impl From<crate::W<RCC_MP_IWDGFZCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_IWDGFZCLRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FZ_IWDG1` reader - FZ_IWDG1"]
pub type FZ_IWDG1_R = crate::BitReader<bool>;
#[doc = "Field `FZ_IWDG1` writer - FZ_IWDG1"]
pub type FZ_IWDG1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_IWDGFZCLRR_SPEC, bool, O>;
#[doc = "Field `FZ_IWDG2` reader - FZ_IWDG2"]
pub type FZ_IWDG2_R = crate::BitReader<bool>;
#[doc = "Field `FZ_IWDG2` writer - FZ_IWDG2"]
pub type FZ_IWDG2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_IWDGFZCLRR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - FZ_IWDG1"]
    #[inline(always)]
    pub fn fz_iwdg1(&self) -> FZ_IWDG1_R {
        FZ_IWDG1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FZ_IWDG2"]
    #[inline(always)]
    pub fn fz_iwdg2(&self) -> FZ_IWDG2_R {
        FZ_IWDG2_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FZ_IWDG1"]
    #[inline(always)]
    #[must_use]
    pub fn fz_iwdg1(&mut self) -> FZ_IWDG1_W<0> {
        FZ_IWDG1_W::new(self)
    }
    #[doc = "Bit 1 - FZ_IWDG2"]
    #[inline(always)]
    #[must_use]
    pub fn fz_iwdg2(&mut self) -> FZ_IWDG2_W<1> {
        FZ_IWDG2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used by the BOOTROM in order to unfreeze the IWDGs clocks. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to . If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_iwdgfzclrr](index.html) module"]
pub struct RCC_MP_IWDGFZCLRR_SPEC;
impl crate::RegisterSpec for RCC_MP_IWDGFZCLRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mp_iwdgfzclrr::R](R) reader structure"]
impl crate::Readable for RCC_MP_IWDGFZCLRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mp_iwdgfzclrr::W](W) writer structure"]
impl crate::Writable for RCC_MP_IWDGFZCLRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_MP_IWDGFZCLRR to value 0"]
impl crate::Resettable for RCC_MP_IWDGFZCLRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
