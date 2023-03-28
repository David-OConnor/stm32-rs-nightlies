#[doc = "Register `RCC_MP_MLAHBENCLRR` reader"]
pub struct R(crate::R<RCC_MP_MLAHBENCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_MLAHBENCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_MLAHBENCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_MLAHBENCLRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MP_MLAHBENCLRR` writer"]
pub struct W(crate::W<RCC_MP_MLAHBENCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_MLAHBENCLRR_SPEC>;
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
impl From<crate::W<RCC_MP_MLAHBENCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_MLAHBENCLRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RETRAMEN` reader - RETRAMEN"]
pub type RETRAMEN_R = crate::BitReader<bool>;
#[doc = "Field `RETRAMEN` writer - RETRAMEN"]
pub type RETRAMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_MLAHBENCLRR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 4 - RETRAMEN"]
    #[inline(always)]
    pub fn retramen(&self) -> RETRAMEN_R {
        RETRAMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - RETRAMEN"]
    #[inline(always)]
    #[must_use]
    pub fn retramen(&mut self) -> RETRAMEN_W<4> {
        RETRAMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to clear the peripheral clock enable bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_mlahbenclrr](index.html) module"]
pub struct RCC_MP_MLAHBENCLRR_SPEC;
impl crate::RegisterSpec for RCC_MP_MLAHBENCLRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mp_mlahbenclrr::R](R) reader structure"]
impl crate::Readable for RCC_MP_MLAHBENCLRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mp_mlahbenclrr::W](W) writer structure"]
impl crate::Writable for RCC_MP_MLAHBENCLRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_MP_MLAHBENCLRR to value 0x10"]
impl crate::Resettable for RCC_MP_MLAHBENCLRR_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
