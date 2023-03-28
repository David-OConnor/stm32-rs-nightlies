#[doc = "Register `RCC_MP_SREQSETR` reader"]
pub struct R(crate::R<RCC_MP_SREQSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_SREQSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_SREQSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_SREQSETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MP_SREQSETR` writer"]
pub struct W(crate::W<RCC_MP_SREQSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_SREQSETR_SPEC>;
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
impl From<crate::W<RCC_MP_SREQSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_SREQSETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STPREQ_P0` reader - STPREQ_P0"]
pub type STPREQ_P0_R = crate::BitReader<bool>;
#[doc = "Field `STPREQ_P0` writer - STPREQ_P0"]
pub type STPREQ_P0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_SREQSETR_SPEC, bool, O>;
#[doc = "Field `STPREQ_P1` reader - STPREQ_P1"]
pub type STPREQ_P1_R = crate::BitReader<bool>;
#[doc = "Field `STPREQ_P1` writer - STPREQ_P1"]
pub type STPREQ_P1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_SREQSETR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - STPREQ_P0"]
    #[inline(always)]
    pub fn stpreq_p0(&self) -> STPREQ_P0_R {
        STPREQ_P0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - STPREQ_P1"]
    #[inline(always)]
    pub fn stpreq_p1(&self) -> STPREQ_P1_R {
        STPREQ_P1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - STPREQ_P0"]
    #[inline(always)]
    #[must_use]
    pub fn stpreq_p0(&mut self) -> STPREQ_P0_W<0> {
        STPREQ_P0_W::new(self)
    }
    #[doc = "Bit 1 - STPREQ_P1"]
    #[inline(always)]
    #[must_use]
    pub fn stpreq_p1(&mut self) -> STPREQ_P1_W<1> {
        STPREQ_P1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Writing has no effect, reading will return the values of the bits. Writing a sets the corresponding bit to . The MCU cannot access to this register. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_sreqsetr](index.html) module"]
pub struct RCC_MP_SREQSETR_SPEC;
impl crate::RegisterSpec for RCC_MP_SREQSETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mp_sreqsetr::R](R) reader structure"]
impl crate::Readable for RCC_MP_SREQSETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mp_sreqsetr::W](W) writer structure"]
impl crate::Writable for RCC_MP_SREQSETR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_MP_SREQSETR to value 0"]
impl crate::Resettable for RCC_MP_SREQSETR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
