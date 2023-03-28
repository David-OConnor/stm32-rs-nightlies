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
#[doc = "Field `I2S1SEL` reader - 2S1SEL"]
pub type I2S1SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2S1SEL` writer - 2S1SEL"]
pub type I2S1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `I2S2SEL` reader - I2S2SEL"]
pub type I2S2SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2S2SEL` writer - I2S2SEL"]
pub type I2S2SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `FDCANSEL` reader - FDCANSEL"]
pub type FDCANSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FDCANSEL` writer - FDCANSEL"]
pub type FDCANSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `USBSEL` reader - USBSEL"]
pub type USBSEL_R = crate::BitReader<bool>;
#[doc = "Field `USBSEL` writer - USBSEL"]
pub type USBSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCIPR2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - 2S1SEL"]
    #[inline(always)]
    pub fn i2s1sel(&self) -> I2S1SEL_R {
        I2S1SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - I2S2SEL"]
    #[inline(always)]
    pub fn i2s2sel(&self) -> I2S2SEL_R {
        I2S2SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:9 - FDCANSEL"]
    #[inline(always)]
    pub fn fdcansel(&self) -> FDCANSEL_R {
        FDCANSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - USBSEL"]
    #[inline(always)]
    pub fn usbsel(&self) -> USBSEL_R {
        USBSEL_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 2S1SEL"]
    #[inline(always)]
    #[must_use]
    pub fn i2s1sel(&mut self) -> I2S1SEL_W<0> {
        I2S1SEL_W::new(self)
    }
    #[doc = "Bits 2:3 - I2S2SEL"]
    #[inline(always)]
    #[must_use]
    pub fn i2s2sel(&mut self) -> I2S2SEL_W<2> {
        I2S2SEL_W::new(self)
    }
    #[doc = "Bits 8:9 - FDCANSEL"]
    #[inline(always)]
    #[must_use]
    pub fn fdcansel(&mut self) -> FDCANSEL_W<8> {
        FDCANSEL_W::new(self)
    }
    #[doc = "Bit 12 - USBSEL"]
    #[inline(always)]
    #[must_use]
    pub fn usbsel(&mut self) -> USBSEL_W<12> {
        USBSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripherals independent clock configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccipr2](index.html) module"]
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
