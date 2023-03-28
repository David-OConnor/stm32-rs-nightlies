#[doc = "Register `RCC_PLL2CFGR1` reader"]
pub struct R(crate::R<RCC_PLL2CFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_PLL2CFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_PLL2CFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_PLL2CFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_PLL2CFGR1` writer"]
pub struct W(crate::W<RCC_PLL2CFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_PLL2CFGR1_SPEC>;
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
impl From<crate::W<RCC_PLL2CFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_PLL2CFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIVN` reader - DIVN"]
pub type DIVN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DIVN` writer - DIVN"]
pub type DIVN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_PLL2CFGR1_SPEC, u16, u16, 9, O>;
#[doc = "Field `DIVM2` reader - DIVM2"]
pub type DIVM2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIVM2` writer - DIVM2"]
pub type DIVM2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_PLL2CFGR1_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:8 - DIVN"]
    #[inline(always)]
    pub fn divn(&self) -> DIVN_R {
        DIVN_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:21 - DIVM2"]
    #[inline(always)]
    pub fn divm2(&self) -> DIVM2_R {
        DIVM2_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - DIVN"]
    #[inline(always)]
    #[must_use]
    pub fn divn(&mut self) -> DIVN_W<0> {
        DIVN_W::new(self)
    }
    #[doc = "Bits 16:21 - DIVM2"]
    #[inline(always)]
    #[must_use]
    pub fn divm2(&mut self) -> DIVM2_W<16> {
        DIVM2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to configure the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_pll2cfgr1](index.html) module"]
pub struct RCC_PLL2CFGR1_SPEC;
impl crate::RegisterSpec for RCC_PLL2CFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_pll2cfgr1::R](R) reader structure"]
impl crate::Readable for RCC_PLL2CFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_pll2cfgr1::W](W) writer structure"]
impl crate::Writable for RCC_PLL2CFGR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_PLL2CFGR1 to value 0x0001_0063"]
impl crate::Resettable for RCC_PLL2CFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0063;
}
