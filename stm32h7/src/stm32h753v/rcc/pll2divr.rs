#[doc = "Register `PLL2DIVR` reader"]
pub struct R(crate::R<PLL2DIVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL2DIVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL2DIVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL2DIVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL2DIVR` writer"]
pub struct W(crate::W<PLL2DIVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL2DIVR_SPEC>;
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
impl From<crate::W<PLL2DIVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL2DIVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIVN2` reader - Multiplication factor for PLL1 VCO"]
pub type DIVN2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DIVN2` writer - Multiplication factor for PLL1 VCO"]
pub type DIVN2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL2DIVR_SPEC, u16, u16, 9, O>;
#[doc = "Field `DIVP2` reader - PLL1 DIVP division factor"]
pub type DIVP2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIVP2` writer - PLL1 DIVP division factor"]
pub type DIVP2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PLL2DIVR_SPEC, u8, u8, 7, O>;
#[doc = "Field `DIVQ2` reader - PLL1 DIVQ division factor"]
pub type DIVQ2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIVQ2` writer - PLL1 DIVQ division factor"]
pub type DIVQ2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PLL2DIVR_SPEC, u8, u8, 7, O>;
#[doc = "Field `DIVR2` reader - PLL1 DIVR division factor"]
pub type DIVR2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIVR2` writer - PLL1 DIVR division factor"]
pub type DIVR2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PLL2DIVR_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:8 - Multiplication factor for PLL1 VCO"]
    #[inline(always)]
    pub fn divn2(&self) -> DIVN2_R {
        DIVN2_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15 - PLL1 DIVP division factor"]
    #[inline(always)]
    pub fn divp2(&self) -> DIVP2_R {
        DIVP2_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - PLL1 DIVQ division factor"]
    #[inline(always)]
    pub fn divq2(&self) -> DIVQ2_R {
        DIVQ2_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - PLL1 DIVR division factor"]
    #[inline(always)]
    pub fn divr2(&self) -> DIVR2_R {
        DIVR2_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - Multiplication factor for PLL1 VCO"]
    #[inline(always)]
    #[must_use]
    pub fn divn2(&mut self) -> DIVN2_W<0> {
        DIVN2_W::new(self)
    }
    #[doc = "Bits 9:15 - PLL1 DIVP division factor"]
    #[inline(always)]
    #[must_use]
    pub fn divp2(&mut self) -> DIVP2_W<9> {
        DIVP2_W::new(self)
    }
    #[doc = "Bits 16:22 - PLL1 DIVQ division factor"]
    #[inline(always)]
    #[must_use]
    pub fn divq2(&mut self) -> DIVQ2_W<16> {
        DIVQ2_W::new(self)
    }
    #[doc = "Bits 24:30 - PLL1 DIVR division factor"]
    #[inline(always)]
    #[must_use]
    pub fn divr2(&mut self) -> DIVR2_W<24> {
        DIVR2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC PLL2 Dividers Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll2divr](index.html) module"]
pub struct PLL2DIVR_SPEC;
impl crate::RegisterSpec for PLL2DIVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll2divr::R](R) reader structure"]
impl crate::Readable for PLL2DIVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll2divr::W](W) writer structure"]
impl crate::Writable for PLL2DIVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL2DIVR to value 0x0101_0280"]
impl crate::Resettable for PLL2DIVR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0101_0280;
}
