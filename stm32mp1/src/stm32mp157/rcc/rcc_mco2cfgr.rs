#[doc = "Register `RCC_MCO2CFGR` reader"]
pub struct R(crate::R<RCC_MCO2CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MCO2CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MCO2CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MCO2CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MCO2CFGR` writer"]
pub struct W(crate::W<RCC_MCO2CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MCO2CFGR_SPEC>;
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
impl From<crate::W<RCC_MCO2CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MCO2CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCO2SEL` reader - MCO2SEL"]
pub type MCO2SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MCO2SEL` writer - MCO2SEL"]
pub type MCO2SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_MCO2CFGR_SPEC, u8, u8, 3, O>;
#[doc = "Field `MCO2DIV` reader - MCO2DIV"]
pub type MCO2DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MCO2DIV` writer - MCO2DIV"]
pub type MCO2DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_MCO2CFGR_SPEC, u8, u8, 4, O>;
#[doc = "Field `MCO2ON` reader - MCO2ON"]
pub type MCO2ON_R = crate::BitReader<bool>;
#[doc = "Field `MCO2ON` writer - MCO2ON"]
pub type MCO2ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MCO2CFGR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - MCO2SEL"]
    #[inline(always)]
    pub fn mco2sel(&self) -> MCO2SEL_R {
        MCO2SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:7 - MCO2DIV"]
    #[inline(always)]
    pub fn mco2div(&self) -> MCO2DIV_R {
        MCO2DIV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - MCO2ON"]
    #[inline(always)]
    pub fn mco2on(&self) -> MCO2ON_R {
        MCO2ON_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - MCO2SEL"]
    #[inline(always)]
    #[must_use]
    pub fn mco2sel(&mut self) -> MCO2SEL_W<0> {
        MCO2SEL_W::new(self)
    }
    #[doc = "Bits 4:7 - MCO2DIV"]
    #[inline(always)]
    #[must_use]
    pub fn mco2div(&mut self) -> MCO2DIV_W<4> {
        MCO2DIV_W::new(self)
    }
    #[doc = "Bit 12 - MCO2ON"]
    #[inline(always)]
    #[must_use]
    pub fn mco2on(&mut self) -> MCO2ON_W<12> {
        MCO2ON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to select the clock generated on MCO2 output.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mco2cfgr](index.html) module"]
pub struct RCC_MCO2CFGR_SPEC;
impl crate::RegisterSpec for RCC_MCO2CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mco2cfgr::R](R) reader structure"]
impl crate::Readable for RCC_MCO2CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mco2cfgr::W](W) writer structure"]
impl crate::Writable for RCC_MCO2CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_MCO2CFGR to value 0"]
impl crate::Resettable for RCC_MCO2CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
