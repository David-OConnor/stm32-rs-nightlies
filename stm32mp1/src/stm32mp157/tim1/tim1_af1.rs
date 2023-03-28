#[doc = "Register `TIM1_AF1` reader"]
pub struct R(crate::R<TIM1_AF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM1_AF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM1_AF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM1_AF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM1_AF1` writer"]
pub struct W(crate::W<TIM1_AF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM1_AF1_SPEC>;
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
impl From<crate::W<TIM1_AF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM1_AF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BKINE` reader - BKINE"]
pub type BKINE_R = crate::BitReader<bool>;
#[doc = "Field `BKINE` writer - BKINE"]
pub type BKINE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_AF1_SPEC, bool, O>;
#[doc = "Field `BKDF1BK0E` reader - BKDF1BK0E"]
pub type BKDF1BK0E_R = crate::BitReader<bool>;
#[doc = "Field `BKDF1BK0E` writer - BKDF1BK0E"]
pub type BKDF1BK0E_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_AF1_SPEC, bool, O>;
#[doc = "Field `BKINP` reader - BKINP"]
pub type BKINP_R = crate::BitReader<bool>;
#[doc = "Field `BKINP` writer - BKINP"]
pub type BKINP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_AF1_SPEC, bool, O>;
#[doc = "Field `ETRSEL` reader - ETRSEL"]
pub type ETRSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETRSEL` writer - ETRSEL"]
pub type ETRSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM1_AF1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - BKINE"]
    #[inline(always)]
    pub fn bkine(&self) -> BKINE_R {
        BKINE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - BKDF1BK0E"]
    #[inline(always)]
    pub fn bkdf1bk0e(&self) -> BKDF1BK0E_R {
        BKDF1BK0E_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BKINP"]
    #[inline(always)]
    pub fn bkinp(&self) -> BKINP_R {
        BKINP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 14:17 - ETRSEL"]
    #[inline(always)]
    pub fn etrsel(&self) -> ETRSEL_R {
        ETRSEL_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - BKINE"]
    #[inline(always)]
    #[must_use]
    pub fn bkine(&mut self) -> BKINE_W<0> {
        BKINE_W::new(self)
    }
    #[doc = "Bit 8 - BKDF1BK0E"]
    #[inline(always)]
    #[must_use]
    pub fn bkdf1bk0e(&mut self) -> BKDF1BK0E_W<8> {
        BKDF1BK0E_W::new(self)
    }
    #[doc = "Bit 9 - BKINP"]
    #[inline(always)]
    #[must_use]
    pub fn bkinp(&mut self) -> BKINP_W<9> {
        BKINP_W::new(self)
    }
    #[doc = "Bits 14:17 - ETRSEL"]
    #[inline(always)]
    #[must_use]
    pub fn etrsel(&mut self) -> ETRSEL_W<14> {
        ETRSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM1 alternate function option register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_af1](index.html) module"]
pub struct TIM1_AF1_SPEC;
impl crate::RegisterSpec for TIM1_AF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim1_af1::R](R) reader structure"]
impl crate::Readable for TIM1_AF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim1_af1::W](W) writer structure"]
impl crate::Writable for TIM1_AF1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM1_AF1 to value 0x01"]
impl crate::Resettable for TIM1_AF1_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
