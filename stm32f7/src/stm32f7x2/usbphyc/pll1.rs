#[doc = "Register `PLL1` reader"]
pub struct R(crate::R<PLL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL1` writer"]
pub struct W(crate::W<PLL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL1_SPEC>;
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
impl From<crate::W<PLL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLL1EN` reader - Enable the PLL1 inside PHY"]
pub type PLL1EN_R = crate::BitReader<bool>;
#[doc = "Field `PLL1EN` writer - Enable the PLL1 inside PHY"]
pub type PLL1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL1_SPEC, bool, O>;
#[doc = "Field `PLL1SEL` reader - : Controls the PHY PLL1 input clock frequency selection"]
pub type PLL1SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLL1SEL` writer - : Controls the PHY PLL1 input clock frequency selection"]
pub type PLL1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL1_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - Enable the PLL1 inside PHY"]
    #[inline(always)]
    pub fn pll1en(&self) -> PLL1EN_R {
        PLL1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - : Controls the PHY PLL1 input clock frequency selection"]
    #[inline(always)]
    pub fn pll1sel(&self) -> PLL1SEL_R {
        PLL1SEL_R::new(((self.bits >> 1) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the PLL1 inside PHY"]
    #[inline(always)]
    #[must_use]
    pub fn pll1en(&mut self) -> PLL1EN_W<0> {
        PLL1EN_W::new(self)
    }
    #[doc = "Bits 1:3 - : Controls the PHY PLL1 input clock frequency selection"]
    #[inline(always)]
    #[must_use]
    pub fn pll1sel(&mut self) -> PLL1SEL_W<1> {
        PLL1SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USBPHYC PLL1 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll1](index.html) module"]
pub struct PLL1_SPEC;
impl crate::RegisterSpec for PLL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll1::R](R) reader structure"]
impl crate::Readable for PLL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll1::W](W) writer structure"]
impl crate::Writable for PLL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL1 to value 0"]
impl crate::Resettable for PLL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
