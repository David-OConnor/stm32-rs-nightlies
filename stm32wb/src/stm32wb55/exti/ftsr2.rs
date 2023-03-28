#[doc = "Register `FTSR2` reader"]
pub struct R(crate::R<FTSR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FTSR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FTSR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FTSR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FTSR2` writer"]
pub struct W(crate::W<FTSR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FTSR2_SPEC>;
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
impl From<crate::W<FTSR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FTSR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FT33` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT33_R = crate::BitReader<bool>;
#[doc = "Field `FT33` writer - Falling trigger event configuration bit of Configurable Event input"]
pub type FT33_W<'a, const O: u8> = crate::BitWriter<'a, u32, FTSR2_SPEC, bool, O>;
#[doc = "Field `FT40_41` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT40_41_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FT40_41` writer - Falling trigger event configuration bit of Configurable Event input"]
pub type FT40_41_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FTSR2_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 1 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft33(&self) -> FT33_R {
        FT33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft40_41(&self) -> FT40_41_R {
        FT40_41_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn ft33(&mut self) -> FT33_W<1> {
        FT33_W::new(self)
    }
    #[doc = "Bits 8:9 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn ft40_41(&mut self) -> FT40_41_W<8> {
        FT40_41_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "falling trigger selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftsr2](index.html) module"]
pub struct FTSR2_SPEC;
impl crate::RegisterSpec for FTSR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ftsr2::R](R) reader structure"]
impl crate::Readable for FTSR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ftsr2::W](W) writer structure"]
impl crate::Writable for FTSR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FTSR2 to value 0"]
impl crate::Resettable for FTSR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
