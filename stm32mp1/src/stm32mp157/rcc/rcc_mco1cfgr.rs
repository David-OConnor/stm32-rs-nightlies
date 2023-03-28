#[doc = "Register `RCC_MCO1CFGR` reader"]
pub struct R(crate::R<RCC_MCO1CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MCO1CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MCO1CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MCO1CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MCO1CFGR` writer"]
pub struct W(crate::W<RCC_MCO1CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MCO1CFGR_SPEC>;
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
impl From<crate::W<RCC_MCO1CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MCO1CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCO1SEL` reader - MCO1SEL"]
pub type MCO1SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MCO1SEL` writer - MCO1SEL"]
pub type MCO1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_MCO1CFGR_SPEC, u8, u8, 3, O>;
#[doc = "Field `MCO1DIV` reader - MCO1DIV"]
pub type MCO1DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MCO1DIV` writer - MCO1DIV"]
pub type MCO1DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_MCO1CFGR_SPEC, u8, u8, 4, O>;
#[doc = "Field `MCO1ON` reader - MCO1ON"]
pub type MCO1ON_R = crate::BitReader<bool>;
#[doc = "Field `MCO1ON` writer - MCO1ON"]
pub type MCO1ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MCO1CFGR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - MCO1SEL"]
    #[inline(always)]
    pub fn mco1sel(&self) -> MCO1SEL_R {
        MCO1SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:7 - MCO1DIV"]
    #[inline(always)]
    pub fn mco1div(&self) -> MCO1DIV_R {
        MCO1DIV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - MCO1ON"]
    #[inline(always)]
    pub fn mco1on(&self) -> MCO1ON_R {
        MCO1ON_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - MCO1SEL"]
    #[inline(always)]
    #[must_use]
    pub fn mco1sel(&mut self) -> MCO1SEL_W<0> {
        MCO1SEL_W::new(self)
    }
    #[doc = "Bits 4:7 - MCO1DIV"]
    #[inline(always)]
    #[must_use]
    pub fn mco1div(&mut self) -> MCO1DIV_W<4> {
        MCO1DIV_W::new(self)
    }
    #[doc = "Bit 12 - MCO1ON"]
    #[inline(always)]
    #[must_use]
    pub fn mco1on(&mut self) -> MCO1ON_W<12> {
        MCO1ON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to select the clock generated on MCO1 output.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mco1cfgr](index.html) module"]
pub struct RCC_MCO1CFGR_SPEC;
impl crate::RegisterSpec for RCC_MCO1CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mco1cfgr::R](R) reader structure"]
impl crate::Readable for RCC_MCO1CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mco1cfgr::W](W) writer structure"]
impl crate::Writable for RCC_MCO1CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_MCO1CFGR to value 0"]
impl crate::Resettable for RCC_MCO1CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
