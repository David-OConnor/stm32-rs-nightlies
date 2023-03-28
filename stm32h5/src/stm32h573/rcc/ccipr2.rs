#[doc = "Register `CCIPR2` reader"]
pub struct R(crate::R<CCIPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCIPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCIPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCIPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCIPR2` writer"]
pub struct W(crate::W<CCIPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCIPR2_SPEC>;
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
impl From<crate::W<CCIPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCIPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USART11SEL` reader - USART11 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type USART11SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USART11SEL` writer - USART11 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type USART11SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `USART12SEL` reader - USART12 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type USART12SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USART12SEL` writer - USART12 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type USART12SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `LPTIM1SEL` reader - LPTIM1 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type LPTIM1SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPTIM1SEL` writer - LPTIM1 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type LPTIM1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `LPTIM2SEL` reader - LPTIM2 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type LPTIM2SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPTIM2SEL` writer - LPTIM2 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type LPTIM2SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `LPTIM3SEL` reader - LPTIM3 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type LPTIM3SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPTIM3SEL` writer - LPTIM3 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type LPTIM3SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `LPTIM4SEL` reader - LPTIM4 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type LPTIM4SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPTIM4SEL` writer - LPTIM4 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type LPTIM4SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `LPTIM5SEL` reader - LPTIM5 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type LPTIM5SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPTIM5SEL` writer - LPTIM5 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type LPTIM5SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `LPTIM6SEL` reader - LPTIM6 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type LPTIM6SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPTIM6SEL` writer - LPTIM6 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type LPTIM6SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR2_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - USART11 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn usart11sel(&self) -> USART11SEL_R {
        USART11SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - USART12 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn usart12sel(&self) -> USART12SEL_R {
        USART12SEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - LPTIM1 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - LPTIM2 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn lptim2sel(&self) -> LPTIM2SEL_R {
        LPTIM2SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - LPTIM3 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn lptim3sel(&self) -> LPTIM3SEL_R {
        LPTIM3SEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - LPTIM4 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn lptim4sel(&self) -> LPTIM4SEL_R {
        LPTIM4SEL_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - LPTIM5 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn lptim5sel(&self) -> LPTIM5SEL_R {
        LPTIM5SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - LPTIM6 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn lptim6sel(&self) -> LPTIM6SEL_R {
        LPTIM6SEL_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - USART11 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn usart11sel(&mut self) -> USART11SEL_W<0> {
        USART11SEL_W::new(self)
    }
    #[doc = "Bits 4:6 - USART12 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn usart12sel(&mut self) -> USART12SEL_W<4> {
        USART12SEL_W::new(self)
    }
    #[doc = "Bits 8:10 - LPTIM1 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W<8> {
        LPTIM1SEL_W::new(self)
    }
    #[doc = "Bits 12:14 - LPTIM2 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2sel(&mut self) -> LPTIM2SEL_W<12> {
        LPTIM2SEL_W::new(self)
    }
    #[doc = "Bits 16:18 - LPTIM3 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn lptim3sel(&mut self) -> LPTIM3SEL_W<16> {
        LPTIM3SEL_W::new(self)
    }
    #[doc = "Bits 20:22 - LPTIM4 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn lptim4sel(&mut self) -> LPTIM4SEL_W<20> {
        LPTIM4SEL_W::new(self)
    }
    #[doc = "Bits 24:26 - LPTIM5 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn lptim5sel(&mut self) -> LPTIM5SEL_W<24> {
        LPTIM5SEL_W::new(self)
    }
    #[doc = "Bits 28:30 - LPTIM6 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn lptim6sel(&mut self) -> LPTIM6SEL_W<28> {
        LPTIM6SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC kernel clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccipr2](index.html) module"]
pub struct CCIPR2_SPEC;
impl crate::RegisterSpec for CCIPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccipr2::R](R) reader structure"]
impl crate::Readable for CCIPR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccipr2::W](W) writer structure"]
impl crate::Writable for CCIPR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCIPR2 to value 0"]
impl crate::Resettable for CCIPR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
