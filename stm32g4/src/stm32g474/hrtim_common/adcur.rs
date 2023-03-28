#[doc = "Register `ADCUR` reader"]
pub struct R(crate::R<ADCUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCUR` writer"]
pub struct W(crate::W<ADCUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCUR_SPEC>;
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
impl From<crate::W<ADCUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD5USRC` reader - AD5USRC"]
pub type AD5USRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AD5USRC` writer - AD5USRC"]
pub type AD5USRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCUR_SPEC, u8, u8, 3, O>;
#[doc = "Field `AD6USRC` reader - AD6USRC"]
pub type AD6USRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AD6USRC` writer - AD6USRC"]
pub type AD6USRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCUR_SPEC, u8, u8, 3, O>;
#[doc = "Field `AD7USRC` reader - AD7USRC"]
pub type AD7USRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AD7USRC` writer - AD7USRC"]
pub type AD7USRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCUR_SPEC, u8, u8, 3, O>;
#[doc = "Field `AD8USRC` reader - AD8USRC"]
pub type AD8USRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AD8USRC` writer - AD8USRC"]
pub type AD8USRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCUR_SPEC, u8, u8, 3, O>;
#[doc = "Field `AD9USRC` reader - AD9USRC"]
pub type AD9USRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AD9USRC` writer - AD9USRC"]
pub type AD9USRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCUR_SPEC, u8, u8, 3, O>;
#[doc = "Field `AD10USRC` reader - AD10USRC"]
pub type AD10USRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AD10USRC` writer - AD10USRC"]
pub type AD10USRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCUR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - AD5USRC"]
    #[inline(always)]
    pub fn ad5usrc(&self) -> AD5USRC_R {
        AD5USRC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - AD6USRC"]
    #[inline(always)]
    pub fn ad6usrc(&self) -> AD6USRC_R {
        AD6USRC_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - AD7USRC"]
    #[inline(always)]
    pub fn ad7usrc(&self) -> AD7USRC_R {
        AD7USRC_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - AD8USRC"]
    #[inline(always)]
    pub fn ad8usrc(&self) -> AD8USRC_R {
        AD8USRC_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - AD9USRC"]
    #[inline(always)]
    pub fn ad9usrc(&self) -> AD9USRC_R {
        AD9USRC_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - AD10USRC"]
    #[inline(always)]
    pub fn ad10usrc(&self) -> AD10USRC_R {
        AD10USRC_R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - AD5USRC"]
    #[inline(always)]
    #[must_use]
    pub fn ad5usrc(&mut self) -> AD5USRC_W<0> {
        AD5USRC_W::new(self)
    }
    #[doc = "Bits 4:6 - AD6USRC"]
    #[inline(always)]
    #[must_use]
    pub fn ad6usrc(&mut self) -> AD6USRC_W<4> {
        AD6USRC_W::new(self)
    }
    #[doc = "Bits 8:10 - AD7USRC"]
    #[inline(always)]
    #[must_use]
    pub fn ad7usrc(&mut self) -> AD7USRC_W<8> {
        AD7USRC_W::new(self)
    }
    #[doc = "Bits 12:14 - AD8USRC"]
    #[inline(always)]
    #[must_use]
    pub fn ad8usrc(&mut self) -> AD8USRC_W<12> {
        AD8USRC_W::new(self)
    }
    #[doc = "Bits 16:18 - AD9USRC"]
    #[inline(always)]
    #[must_use]
    pub fn ad9usrc(&mut self) -> AD9USRC_W<16> {
        AD9USRC_W::new(self)
    }
    #[doc = "Bits 20:22 - AD10USRC"]
    #[inline(always)]
    #[must_use]
    pub fn ad10usrc(&mut self) -> AD10USRC_W<20> {
        AD10USRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HRTIM ADC Trigger Update Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcur](index.html) module"]
pub struct ADCUR_SPEC;
impl crate::RegisterSpec for ADCUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcur::R](R) reader structure"]
impl crate::Readable for ADCUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcur::W](W) writer structure"]
impl crate::Writable for ADCUR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCUR to value 0"]
impl crate::Resettable for ADCUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
