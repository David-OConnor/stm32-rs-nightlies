#[doc = "Register `CCIPR4` reader"]
pub struct R(crate::R<CCIPR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCIPR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCIPR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCIPR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCIPR4` writer"]
pub struct W(crate::W<CCIPR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCIPR4_SPEC>;
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
impl From<crate::W<CCIPR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCIPR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSTICKSEL` reader - SYSTICK clock source selection Note: rcc_hclk frequency must be four times higher than lsi_ker_ck/lse_ck (period (LSI/LSE) ≥ 4 * period (HCLK)."]
pub type SYSTICKSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYSTICKSEL` writer - SYSTICK clock source selection Note: rcc_hclk frequency must be four times higher than lsi_ker_ck/lse_ck (period (LSI/LSE) ≥ 4 * period (HCLK)."]
pub type SYSTICKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR4_SPEC, u8, u8, 2, O>;
#[doc = "Field `USBFSSEL` reader - USBFS kernel clock source selection"]
pub type USBFSSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USBFSSEL` writer - USBFS kernel clock source selection"]
pub type USBFSSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR4_SPEC, u8, u8, 2, O>;
#[doc = "Field `I2C1SEL` reader - I2C1 kernel clock source selection"]
pub type I2C1SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2C1SEL` writer - I2C1 kernel clock source selection"]
pub type I2C1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR4_SPEC, u8, u8, 2, O>;
#[doc = "Field `I2C2SEL` reader - I2C2 kernel clock source selection"]
pub type I2C2SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2C2SEL` writer - I2C2 kernel clock source selection"]
pub type I2C2SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR4_SPEC, u8, u8, 2, O>;
#[doc = "Field `I3C1SEL` reader - I3C1 kernel clock source selection"]
pub type I3C1SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I3C1SEL` writer - I3C1 kernel clock source selection"]
pub type I3C1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR4_SPEC, u8, u8, 2, O>;
#[doc = "Field `I3C2SEL` reader - I3C2 kernel clock source selection"]
pub type I3C2SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I3C2SEL` writer - I3C2 kernel clock source selection"]
pub type I3C2SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR4_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 2:3 - SYSTICK clock source selection Note: rcc_hclk frequency must be four times higher than lsi_ker_ck/lse_ck (period (LSI/LSE) ≥ 4 * period (HCLK)."]
    #[inline(always)]
    pub fn systicksel(&self) -> SYSTICKSEL_R {
        SYSTICKSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - USBFS kernel clock source selection"]
    #[inline(always)]
    pub fn usbfssel(&self) -> USBFSSEL_R {
        USBFSSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 16:17 - I2C1 kernel clock source selection"]
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2C1SEL_R {
        I2C1SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - I2C2 kernel clock source selection"]
    #[inline(always)]
    pub fn i2c2sel(&self) -> I2C2SEL_R {
        I2C2SEL_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 24:25 - I3C1 kernel clock source selection"]
    #[inline(always)]
    pub fn i3c1sel(&self) -> I3C1SEL_R {
        I3C1SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - I3C2 kernel clock source selection"]
    #[inline(always)]
    pub fn i3c2sel(&self) -> I3C2SEL_R {
        I3C2SEL_R::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 2:3 - SYSTICK clock source selection Note: rcc_hclk frequency must be four times higher than lsi_ker_ck/lse_ck (period (LSI/LSE) ≥ 4 * period (HCLK)."]
    #[inline(always)]
    #[must_use]
    pub fn systicksel(&mut self) -> SYSTICKSEL_W<2> {
        SYSTICKSEL_W::new(self)
    }
    #[doc = "Bits 4:5 - USBFS kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usbfssel(&mut self) -> USBFSSEL_W<4> {
        USBFSSEL_W::new(self)
    }
    #[doc = "Bits 16:17 - I2C1 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1sel(&mut self) -> I2C1SEL_W<16> {
        I2C1SEL_W::new(self)
    }
    #[doc = "Bits 18:19 - I2C2 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2sel(&mut self) -> I2C2SEL_W<18> {
        I2C2SEL_W::new(self)
    }
    #[doc = "Bits 24:25 - I3C1 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i3c1sel(&mut self) -> I3C1SEL_W<24> {
        I3C1SEL_W::new(self)
    }
    #[doc = "Bits 26:27 - I3C2 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i3c2sel(&mut self) -> I3C2SEL_W<26> {
        I3C2SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC kernel clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccipr4](index.html) module"]
pub struct CCIPR4_SPEC;
impl crate::RegisterSpec for CCIPR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccipr4::R](R) reader structure"]
impl crate::Readable for CCIPR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccipr4::W](W) writer structure"]
impl crate::Writable for CCIPR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCIPR4 to value 0"]
impl crate::Resettable for CCIPR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
