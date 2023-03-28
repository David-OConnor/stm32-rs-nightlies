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
#[doc = "Field `LPTIM1SEL` reader - LPTIM1 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type LPTIM1SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPTIM1SEL` writer - LPTIM1 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type LPTIM1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `LPTIM2SEL` reader - LPTIM2 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type LPTIM2SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPTIM2SEL` writer - LPTIM2 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type LPTIM2SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR2_SPEC, u8, u8, 3, O>;
impl R {
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
}
impl W {
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
